use std::{default, path::PathBuf, str::FromStr};

use itertools::Itertools;
use rusqlite::types::Value;
use sha2::{Sha256, Sha512, Digest};

use crate::database::operations::{db_item::DatabaseItem, tables::DatabaseTable};

#[derive(Default)]
struct TrackedFile {
    id: u32,
    pub name: String,
    pub path: PathBuf
}


pub struct DatabaseTrackedFiles {
    id: u32,
    files: Vec<String>
}

impl Default for DatabaseTrackedFiles {
    fn default() -> Self {
        Self { id: 0, files: Vec::new() }
    }
}

impl DatabaseItem for DatabaseTrackedFiles {
    type Item = DatabaseTrackedFiles;

    fn parse(&self, stmt: &mut rusqlite::Statement) -> Result<Self, rusqlite::Error> {
        let mut tracked_files = DatabaseTrackedFiles::default();
        let data = stmt.query_row([], |row| {
            let id = row.get::<usize, u32>(0);
            let files = &row.get::<usize, String>(1);
            Ok(DatabaseTrackedFiles {
                id: match id {
                    Ok(x) => x,
                    Err(_) => 0,
                },
                files: match files {
                    Ok(x) => serde_json::from_str(x).expect("JSON Deserialization should succeed"),
                    Err(_) => Vec::new(),
                }
            })
        });

        data
    }

    fn id(&self) -> u32 {
        0
    }

    fn table(&self) -> crate::database::operations::tables::DatabaseTable {
        DatabaseTable::TrackedFiles
    }

    fn fields<'a>(&self) -> Vec<String> {
        vec!["id", "files"]
            .iter().map(|x| String::from(*x)).collect()
    }

    fn values<'a>(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<Value> = Vec::new();
        let files = serde_json::to_string(&self.files).expect("JSON serialization should succeed");
        values.push(self.id.into());
        values.push(files.into());
        return values;
    }
}

impl DatabaseTrackedFiles {

    fn hash_file_path(path: &std::path::PathBuf) -> String {
        let path_str = path.to_string_lossy();
        let mut hasher = Sha256::new();
        hasher.update(path_str.as_bytes());
        let result = hasher.finalize();
        
       format!("{:x}", result)
    }

    pub fn file_exists(&mut self, path: std::path::PathBuf, add_missing: bool) -> bool{
        let hash = DatabaseTrackedFiles::hash_file_path(&path);
        let exists = self.files.contains(&hash);

        println!("{}; {}", hash, exists);

        if !exists && add_missing { self.files.push(hash) }
        
        exists
    }

    pub fn debug(&self){
        println!("{}", self.id);
        println!("{:?}", self.files);
    }
}