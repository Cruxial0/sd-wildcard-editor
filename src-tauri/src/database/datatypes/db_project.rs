use rusqlite::Statement;

use crate::database::operations::db_item::DatabaseItem;

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
        todo!()
    }

    fn fields(&self) -> String {
        todo!()
    }
}