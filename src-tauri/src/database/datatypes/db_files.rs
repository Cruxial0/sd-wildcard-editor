use std::path::PathBuf;

use crate::database::operations::db_item::DatabaseItem;

#[derive(Default)]
pub struct DatabaseTrackedFile {
    pub name: String,
    pub kind: u32,
    pub path: PathBuf
}

impl DatabaseItem for DatabaseTrackedFile {
    type Item = DatabaseTrackedFile;

    fn parse(&self, stmt: &mut rusqlite::Statement) -> Result<Self, rusqlite::Error> {
        todo!()
    }

    fn id(&self) -> u32 {
        todo!()
    }

    fn table(&self) -> crate::database::operations::tables::DatabaseTable {
        todo!()
    }

    fn fields<'a>(&self) -> Vec<String> {
        todo!()
    }

    fn values<'a>(&self) -> Vec<rusqlite::types::Value> {
        todo!()
    }
}