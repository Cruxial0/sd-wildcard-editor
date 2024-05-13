use rusqlite::Statement;

use crate::database::operations::{db_item::DatabaseItem, tables::DatabaseTable};

#[derive(Default)]
pub struct Project {
    id: u32,
    name: String,
    description: String,
    wildcard_ids: Vec<u32>,
    project_ids: Vec<u32>
}

impl DatabaseItem for Project {
    type Item = Project;

    fn parse(&self, stmt: &mut Statement) -> Option<Self::Item> {
        // SELECT * FROM Wildcards Where ID in (1, 2, 3, etc.)
        todo!()
    }

    fn table(&self) -> DatabaseTable {
        DatabaseTable::Projects
    }

    fn fields(&self) -> Vec<String> {
        todo!()
    }
    
    fn values(&self) -> Vec<rusqlite::types::Value> {
        todo!()
    }
    
    fn id(&self) -> u32 {
        todo!()
    }
}