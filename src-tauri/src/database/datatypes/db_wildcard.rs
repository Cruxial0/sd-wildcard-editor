use std::path::PathBuf;

use rusqlite::Statement;

use crate::{database::operations::{db_item::DatabaseItem, tables::DatabaseTable}, logging::logger};

#[derive(Clone)]
pub struct DatabaseWildcard {
    pub id: u32,
    pub name: String,
    pub path: PathBuf,
    pub lines: Vec<String>
}

impl Default for DatabaseWildcard {
    fn default() -> Self {
        DatabaseWildcard {
            id: 0,
            name: "Default".to_owned(),
            path: PathBuf::new(),
            lines: Vec::new()
        }
    }
}

impl DatabaseItem for DatabaseWildcard {
    type Item = DatabaseWildcard;
    
    fn parse(&self, stmt: &mut Statement) -> Option<Self> {
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

        match data {
            Ok(x) => Some(x),
            Err(e) => {
                logger::log_error(&format!("An error occured: {:?}", e), "WildcardDeserialize", logger::LogVisibility::Backend);
                None
            },
        }
    }

    fn id(&self) -> u32 {
        self.id
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
        values.push(self.id.into());
        values.push(name.into());
        values.push(self.path.to_str().unwrap().to_owned().into());
        values.push(serde_json::to_string(&self.lines).unwrap().into());
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