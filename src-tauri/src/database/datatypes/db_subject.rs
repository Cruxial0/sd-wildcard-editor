use std::{fs::Metadata, os::windows::fs::MetadataExt, path::{Path, PathBuf}};

use itertools::Itertools;
use rusqlite::{types::Value, Error, Statement};
use tauri::AppHandle;
use uuid::Uuid;
use walkdir::DirEntry;

use crate::{
    database::operations::{db_item::DatabaseItem, db_read::load_multiple, tables::DatabaseTable}, deployment::{deploy_node::DeployNode, deployable::Deployable}, helpers::dir_utils::{self, get_public_directory}, logging::logger
};

use super::{
    db_files::DatabaseTrackedFiles, db_merge_definition::DatabaseMergeDefinition,
    db_wildcard::DatabaseWildcard,
};

#[derive(Clone, Hash, Eq)]
pub struct DatabaseSubject {
    pub uuid: String,
    pub name: String,
    pub path: PathBuf,
    pub description: String,
    pub wildcard_ids: Vec<String>,
    pub subject_ids: Vec<String>,
    pub merge_def_ids: Vec<String>,
    wildcards: Vec<DatabaseWildcard>,
    subjects: Vec<DatabaseSubject>,
    merge_definitions: Vec<DatabaseMergeDefinition>,
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
        if self.subject_ids.contains(&subject.uuid) {
            return;
        }
        self.subject_ids.push(subject.uuid.clone());
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
            uuid: id.to_owned(),
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
        self.load_merge_definitions(handle);
    }

    pub fn load_merge_definitions(&mut self, handle: &AppHandle) -> Vec<DatabaseMergeDefinition> {
        let mut defs: Vec<DatabaseMergeDefinition> = Vec::new();
        for merge_def in self.merge_def_ids.clone() {
            let definition = DatabaseMergeDefinition::from_id(&merge_def)
                .read_db(handle)
                .expect("Should be able to read DatabaseMergeDefinition");
            defs.push(definition);
        }
        self.merge_definitions = defs.clone();
        defs
    }

    fn load_wildcards_internal(&mut self, handle: &AppHandle) {
        self.wildcards = self
            .wildcard_ids
            .iter()
            .map(|w| DatabaseWildcard::from_id(w).read_db(handle).unwrap())
            .collect();
    }

    fn load_subjects_internal(&mut self, handle: &AppHandle, load_children: bool) {
        let mut subjects: Vec<DatabaseSubject> = self
            .subject_ids
            .iter()
            .map(|p| DatabaseSubject::from_id(p).read_db(handle).unwrap())
            .collect();
        if load_children {
            subjects.iter_mut().for_each(|x| x.load(handle, true));
        }
        self.subjects = subjects;
    }

    pub fn initialize_merge_definition(&mut self, handle: &AppHandle) {
        let definition = DatabaseMergeDefinition::create_default(self.clone(), handle);

        self.merge_def_ids.push(definition.clone().id);

        self.write_db(handle, None, None)
    }

    pub fn from_direntry(handle: &AppHandle, entry: &DirEntry) -> Option<DatabaseSubject> {
        let unique_id = DatabaseTrackedFiles::get_uuid_of_dir_entry(&entry, handle);
        let abs_path = PathBuf::from(entry.path());
        let rel_path = match abs_path.strip_prefix(get_public_directory()) {
            Ok(x) => x,
            Err(e) => { println!("{:?}", e); Path::new("") }
        };
        
        Some(DatabaseSubject {
            uuid: unique_id.to_string(),
            name: entry.file_name().to_str().unwrap().to_owned(),
            path: PathBuf::from(rel_path),
            ..Default::default()
        })
    }

    pub fn from_parent(handle: &AppHandle, parent: &Path) -> Option<Self> {
        let unique_id = DatabaseTrackedFiles::hash_file_data(
            &dir_utils::get_list_of_files(parent),
            parent.metadata().unwrap().creation_time(),
        );
        Some(DatabaseSubject {
            uuid: unique_id.to_string(),
            name: parent.file_name().unwrap().to_str().unwrap().to_owned(),
            ..Default::default()
        })
    }
}

impl Default for DatabaseSubject {
    fn default() -> Self {
        DatabaseSubject {
            uuid: Uuid::nil().to_string(),
            name: "DefaultSubject".to_owned(),
            path: PathBuf::new(),
            description: "".to_owned(),
            wildcard_ids: Vec::new(),
            subject_ids: Vec::new(),
            wildcards: Vec::new(),
            subjects: Vec::new(),
            merge_def_ids: Vec::new(),
            merge_definitions: Vec::new(),
        }
    }
}

impl PartialEq for DatabaseSubject {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Deployable for DatabaseSubject {
    fn generate_deploy_node(&self, path: impl AsRef<Path>, handle: &AppHandle) -> Option<crate::deployment::deploy_node::DeployNode> {

        if self.merge_definitions.len() < 1 { 
            return None; 
        }

        let mut children: Vec<DeployNode> = Vec::new();
        for sub in &self.subjects {
            if let Some(node) = sub.generate_deploy_node(self.path.clone(), handle) {
                children.push(node);
            }
        };
        for def in &self.merge_definitions {
            if let Some(node) = def.generate_deploy_node(self.path.clone(), handle) {
                children.push(node);
            }
        };
        for wc in &self.wildcards {
            if let Some(node) = wc.generate_deploy_node(self.path.clone(), handle) {
                children.push(node);
            }
        }

        Some(DeployNode::new(Vec::new(), self.path.clone(), children))
    }
}

impl DatabaseItem for DatabaseSubject {
    type Item = DatabaseSubject;

    fn parse(&self, stmt: &mut Statement) -> Result<Self::Item, Error> {
        // SELECT * FROM Wildcards Where ID in (1, 2, 3, etc.)
        let data = stmt.query_row((), |row| {
            Ok(DatabaseSubject {
                uuid: row.get(0)?,
                name: row.get(1)?,
                path: PathBuf::from(row.get::<usize, String>(2)?),
                description: row.get(3)?,
                wildcard_ids: serde_json::from_str(&row.get::<usize, String>(4)?)
                    .expect("JSON Deserialization should succeed"),
                subject_ids: serde_json::from_str(&row.get::<usize, String>(5)?)
                    .expect("JSON Deserialization should succeed"),
                merge_def_ids: serde_json::from_str(&row.get::<usize, String>(6)?)
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
        vec![
            "uuid",
            "name",
            "path",
            "description",
            "wildcards",
            "subjects",
            "mergeDefs",
        ]
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
        let merge_def_ids =
            serde_json::to_string(&self.merge_def_ids).expect("JSON serialization should succeed");
        values.push(self.uuid.clone().into());
        values.push(self.name.clone().into());
        values.push(self.path.to_str().unwrap().to_owned().into());
        values.push(self.description.clone().into());
        values.push(wildcard_ids.into());
        values.push(subjects_ids.into());
        values.push(merge_def_ids.into());
        values
    }

    fn id(&self) -> String {
        self.uuid.clone()
    }
}

impl serde::Serialize for DatabaseSubject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Workspace", 5)?;
        state.serialize_field("id", &self.uuid)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("wildcards", &self.wildcards)?;
        state.serialize_field("subjects", &self.subjects)?;
        state.serialize_field("mergeDefinitions", &self.merge_def_ids);
        state.end()
    }
}
