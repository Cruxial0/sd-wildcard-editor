use itertools::Itertools;
use rusqlite::{types::Value, Error, Statement};
use tauri::AppHandle;
use uuid::Uuid;
use walkdir::DirEntry;

use crate::{
    database::operations::{db_item::DatabaseItem, db_read::load_multiple, tables::DatabaseTable},
    logging::logger,
    state::ServiceAccess,
};

use super::db_wildcard::DatabaseWildcard;

#[derive(Clone, Hash, Eq)]
pub struct DatabaseSubject {
    pub id: String,
    pub name: String,
    pub description: String,
    pub wildcard_ids: Vec<String>,
    pub subject_ids: Vec<String>,
    wildcards: Vec<DatabaseWildcard>,
    subjects: Vec<DatabaseSubject>,
}

impl DatabaseSubject {
    pub fn add_wildcard(&mut self, wildcard: &DatabaseWildcard) {
        if self.wildcard_ids.contains(&wildcard.id) {
            return;
        }
        self.wildcard_ids.push(wildcard.id.clone());
        self.wildcards.push(wildcard.clone());
    }

    pub fn add_subject(&mut self, subject: &DatabaseSubject) {
        if self.subject_ids.contains(&subject.id) {
            return;
        }
        self.subject_ids.push(subject.id.clone());
        self.subjects.push(subject.clone());
    }

    pub fn load_recursive(&mut self, handle: &AppHandle) {
        self.wildcards = load_multiple(
            handle,
            &DatabaseWildcard::default(),
            self.wildcard_ids.clone(),
        )
        .expect("Should be able to load wildcards");
        self.subjects = load_multiple(handle, self, self.subject_ids.clone())
            .expect("Should be able to load subjects");
    }

    pub fn from_id(id: &str) -> DatabaseSubject {
        DatabaseSubject {
            id: id.to_owned(),
            ..Default::default()
        }
    }

    pub fn wildcards(&self) -> &Vec<DatabaseWildcard> {
        &self.wildcards
    }

    pub fn subjects(&self) -> &Vec<DatabaseSubject> {
        &self.subjects
    }

    pub fn load(&mut self, handle: &AppHandle, load_children: bool) {
        self.load_wildcards_internal(handle);
        self.load_subjects_internal(handle, load_children);
    }

    fn load_wildcards_internal(&mut self, handle: &AppHandle) {
        self.wildcards = self
            .wildcard_ids
            .iter()
            .map(|w| DatabaseWildcard::from_id(w).read(handle).unwrap())
            .collect();
    }

    fn load_subjects_internal(&mut self, handle: &AppHandle, load_children: bool) {
        let mut subjects: Vec<DatabaseSubject> = self
            .subject_ids
            .iter()
            .map(|p| DatabaseSubject::from_id(p).read(handle).unwrap())
            .collect();
        if load_children {
            subjects.iter_mut().for_each(|x| x.load(handle, true));
        }
        self.subjects = subjects;
    }

    pub fn from_direntry(handle: &AppHandle, name: String) -> Option<DatabaseSubject> {
        let unique_id = Uuid::new_v4();
        Some(DatabaseSubject {
            id: unique_id.to_string(),
            name: name,
            ..Default::default()
        })
    }
}

impl Default for DatabaseSubject {
    fn default() -> Self {
        DatabaseSubject {
            id: Uuid::new_v4().to_string(),
            name: "DefaultSubject".to_owned(),
            description: "".to_owned(),
            wildcard_ids: Vec::new(),
            subject_ids: Vec::new(),
            wildcards: Vec::new(),
            subjects: Vec::new(),
        }
    }
}

impl PartialEq for DatabaseSubject {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl DatabaseItem for DatabaseSubject {
    type Item = DatabaseSubject;

    fn parse(&self, stmt: &mut Statement) -> Result<Self::Item, Error> {
        // SELECT * FROM Wildcards Where ID in (1, 2, 3, etc.)
        let data = stmt.query_row((), |row| {
            Ok(DatabaseSubject {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                wildcard_ids: serde_json::from_str(&row.get::<usize, String>(3)?)
                    .expect("JSON Deserialization should succeed"),
                subject_ids: serde_json::from_str(&row.get::<usize, String>(4)?)
                    .expect("JSON Deserialization should succeed"),
                ..Default::default()
            })
        });

        data
    }

    fn table(&self) -> DatabaseTable {
        DatabaseTable::Subjects
    }

    fn fields(&self) -> Vec<String> {
        vec!["id", "name", "description", "wildcards", "subjects"]
            .iter()
            .map(|x| String::from(*x))
            .collect()
    }

    fn values(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<Value> = Vec::new();
        let wildcard_ids =
            serde_json::to_string(&self.wildcard_ids).expect("JSON serialization should succeed");
        let subjects_ids =
            serde_json::to_string(&self.subject_ids).expect("JSON serialization should succeed");
        values.push(self.id.clone().into());
        values.push(self.name.clone().into());
        values.push(self.description.clone().into());
        values.push(wildcard_ids.into());
        values.push(subjects_ids.into());
        values
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

impl serde::Serialize for DatabaseSubject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Workspace", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("wildcards", &self.wildcards)?;
        state.serialize_field("subjects", &self.subjects)?;
        state.end()
    }
}
