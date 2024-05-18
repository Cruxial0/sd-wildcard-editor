use itertools::Itertools;
use rusqlite::{Error, types::Value, Statement};
use tauri::AppHandle;
use walkdir::DirEntry;

use crate::{database::operations::{db_item::DatabaseItem, db_read::load_multiple, tables::DatabaseTable}, logging::logger, state::ServiceAccess};

use super::db_wildcard::DatabaseWildcard;

#[derive(Clone, Hash, Eq)]
pub struct DatabaseProject {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub wildcard_ids: Vec<u32>,
    pub project_ids: Vec<u32>,
    pub wildcards: Vec<DatabaseWildcard>,
    pub projects: Vec<DatabaseProject>
}

impl DatabaseProject {
    pub fn add_wildcard(&mut self, wildcard: &DatabaseWildcard) {
        if self.wildcard_ids.contains(&wildcard.id) { return; }
        self.wildcard_ids.push(wildcard.id);
        self.wildcards.push(wildcard.clone());
    }

    pub fn add_project(&mut self, project: &DatabaseProject) {
        if self.project_ids.contains(&project.id) { return; }
        self.project_ids.push(project.id);
        self.projects.push(project.clone());
    }

    pub fn load_recursive(&mut self, handle: &AppHandle) {
        self.wildcards = load_multiple(handle, &DatabaseWildcard::default(), self.wildcard_ids.clone()).expect("Should be able to load wildcards");
        self.projects = load_multiple(handle, self, self.project_ids.clone()).expect("Should be able to load projects");
    }

    pub fn from_id(id: &u32) -> DatabaseProject {
        DatabaseProject {
            id: *id,
            ..Default::default()
        }
    }

    pub fn wildcards(&self) -> &Vec<DatabaseWildcard> {
        &self.wildcards
    }

    pub fn from_direntry(handle: &AppHandle, name: String) -> Option<DatabaseProject>{
        let unique_id = handle.db_session(|session| session.get_and_claim_id(DatabaseTable::Projects));
        
        match unique_id {
            Ok(id) => {
                Some(DatabaseProject {
                    id: id,
                    name: name,
                    ..Default::default()
                })
            },
            Err(_) => None,
        }
        
    }
}

impl Default for DatabaseProject {
    fn default() -> Self {
        DatabaseProject {
            id: 1,
            name: "DefaultProject".to_owned(),
            description: "".to_owned(),
            wildcard_ids: Vec::new(),
            project_ids: Vec::new(),
            wildcards: Vec::new(),
            projects: Vec::new()
        }
    }
}

impl PartialEq for DatabaseProject {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl DatabaseItem for DatabaseProject {
    type Item = DatabaseProject;

    fn parse(&self, stmt: &mut Statement) -> Result<Self::Item, Error> {
        // SELECT * FROM Wildcards Where ID in (1, 2, 3, etc.)
        let data = stmt.query_row((), |row| {
            Ok(DatabaseProject{
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                wildcard_ids: serde_json::from_str(&row.get::<usize, String>(3)?).expect("JSON Deserialization should succeed"),
                project_ids: serde_json::from_str(&row.get::<usize, String>(4)?).expect("JSON Deserialization should succeed"),
                ..Default::default()
            })
        });

        data
    }

    fn table(&self) -> DatabaseTable {
        DatabaseTable::Projects
    }

    fn fields(&self) -> Vec<String> {
        vec!["id", "name", "description", "wildcards", "projects"]
            .iter().map(|x| String::from(*x)).collect()
    }
    
    fn values(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<Value> = Vec::new();
        let wildcard_ids = serde_json::to_string(&self.wildcard_ids).expect("JSON serialization should succeed");
        let project_ids = serde_json::to_string(&self.project_ids).expect("JSON serialization should succeed");
        values.push(self.id.into());
        values.push(self.name.clone().into());
        values.push(self.description.clone().into());
        values.push(wildcard_ids.into());
        values.push(project_ids.into());
        values
    }
    
    fn id(&self) -> u32 { self.id }
}