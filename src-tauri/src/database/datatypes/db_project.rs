use itertools::Itertools;
use rusqlite::{Error, types::Value, Statement};

use crate::{database::operations::{db_item::DatabaseItem, tables::DatabaseTable}, logging::logger};

use super::db_wildcard::DatabaseWildcard;

pub struct DatabaseProject {
    id: u32,
    name: String,
    description: String,
    wildcard_ids: Vec<u32>,
    project_ids: Vec<u32>
}

impl DatabaseProject {
    pub fn add_wildcard(&mut self, wildcard: &DatabaseWildcard) {
        if self.wildcard_ids.contains(&wildcard.id) { return; }
        self.wildcard_ids.push(wildcard.id);
        logger::log(&format!("{:?}", self.wildcard_ids), "DatabaseProject", logger::LogVisibility::Backend);
    }

    pub fn add_project(&mut self, project: &DatabaseProject) {
        if self.project_ids.contains(&project.id) { return; }
        self.project_ids.push(project.id)
    }

    pub fn from_id(id: &u32) -> DatabaseProject {
        DatabaseProject {
            id: *id,
            ..Default::default()
        }
    }
}

impl Default for DatabaseProject {
    fn default() -> Self {
        DatabaseProject {
            id: 1,
            name: "DefaultProject".to_owned(),
            description: "Default Description".to_owned(),
            wildcard_ids: Vec::new(),
            project_ids: Vec::new()
        }
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