use std::path::PathBuf;

use rusqlite::Statement;

use crate::{database::operations::db_item::DatabaseItem, logging::logger};

#[derive(Default)]
pub struct DatabaseWildcard {
    pub id: u32,
    pub name: String,
    pub path: PathBuf,
    pub lines: Vec<String>
}

impl DatabaseItem for DatabaseWildcard {
    type Item = DatabaseWildcard;
    
    fn parse(&self, stmt: &mut Statement) -> Option<Self> {
        let data = stmt.query_map([], |row| {
            let path: String = row.get::<usize, String>(2).expect("Path should be deserializable").into();
            let lines: String = row.get::<usize, String>(3).expect("Lines should be deserializable").into();
            Ok(DatabaseWildcard{
                id: row.get(0)?,
                name: row.get(1)?,
                path: PathBuf::from(path),
                lines: serde_json::from_str(&lines).expect("Lines Deserialization should succeed")
            })
        }).expect("Wildcard Deserialization should succeed");

        match data.last().expect("Data should not be empty") {
            Ok(x) => Some(x),
            Err(e) => {
                logger::log_error(&format!("An error occured: {:?}", e), "WildcardDeserialize", logger::LogVisibility::Backend);
                None
            },
        }
    }
    
    fn fields(&self) -> String {
        "ID, Name, Path, Lines".to_owned()
    }
}