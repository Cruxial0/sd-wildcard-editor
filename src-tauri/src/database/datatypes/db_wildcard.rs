use std::path::PathBuf;

use rusqlite::{Error, Statement};
use tauri::AppHandle;
use uuid::Uuid;
use walkdir::DirEntry;

use crate::{database::operations::{db_item::DatabaseItem, tables::DatabaseTable}, logging::logger, state::ServiceAccess};

#[derive(Clone, Hash, Eq)]
pub struct DatabaseWildcard {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub lines: Vec<String>
}

impl DatabaseWildcard {
    pub fn from_id(id: &str) -> DatabaseWildcard {
        DatabaseWildcard {
            id: id.to_owned(),
            ..Default::default()
        }
    }

    pub fn from_direntry(handle: &AppHandle, entry: &DirEntry) -> DatabaseWildcard {
        let content: Vec<String> = std::fs::read_to_string(entry.path())
            .expect("File should be readable").lines().map(|l| l.to_string()).collect();

        let unique_id = Uuid::new_v4();

        DatabaseWildcard {
            id: unique_id.to_string(),
            name: entry.file_name().to_str().to_owned().unwrap().to_string(),
            path: PathBuf::from(entry.path()),
            lines: content
        }
    }
}

impl Default for DatabaseWildcard {
    fn default() -> Self {
        DatabaseWildcard {
            id: Uuid::new_v4().to_string(),
            name: "New Wildcard".to_owned(),
            path: PathBuf::new(),
            lines: Vec::new()
        }
    }
}

impl PartialEq for DatabaseWildcard {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl DatabaseItem for DatabaseWildcard {
    type Item = DatabaseWildcard;
    
    fn parse(&self, stmt: &mut Statement) -> Result<Self, Error> {
        let data = stmt.query_row([], |row| {
            let path: String = row.get::<usize, String>(2).expect("Path should be deserializable").into();
            let lines: String = row.get::<usize, String>(3).expect("Lines should be deserializable").into();
            Ok(DatabaseWildcard{
                id: row.get(0)?,
                name: row.get(1)?,
                path: PathBuf::from(path),
                lines: serde_json::from_str(&lines).expect("Lines Deserialization should succeed")
            })
        });

        data
    }

    fn id(&self) -> String {
        self.id.clone()
    }

    fn table(&self) -> DatabaseTable {
        DatabaseTable::Wildcards
    }
    
    fn fields(&self) -> Vec<String> {
        vec!["id", "name", "path", "lines"].iter().map(|x| String::from(*x)).collect()
    }
    
    fn values(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<rusqlite::types::Value> = Vec::new();
        let name = self.name.clone();
        values.push(self.id.clone().into());
        values.push(name.into());
        values.push(self.path.to_str().unwrap().to_owned().into());
        values.push(serde_json::to_string(&self.lines).expect("JSON serialization should succeed").into());
        values
    }
}

impl serde::Serialize for DatabaseWildcard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Wildcard", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name.to_string())?;
        state.serialize_field("content", &self.lines)?;
        state.end()
    }
}