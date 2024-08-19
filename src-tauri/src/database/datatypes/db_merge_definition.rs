use rusqlite::types::Value;
use serde::Serialize;
use tauri::AppHandle;
use uuid::Uuid;

use crate::{database::operations::db_item::DatabaseItem, logging::logger::LogVisibility, state::ServiceAccess, subjects::{merge_definition::{MergeDefinition, MergeField}, merge_node::MergeNode}};

use super::db_subject::DatabaseSubject;

#[derive(Serialize, Default, Clone, Hash, PartialEq, Eq)]
pub struct DatabaseMergeDefinition{
    pub id: String,
    merge_pattern: MergeDefinition
}

impl DatabaseMergeDefinition {
    pub fn create_default(subject: DatabaseSubject, handle: &AppHandle) -> DatabaseMergeDefinition {
        let def = DatabaseMergeDefinition{
            id: Uuid::now_v7().to_string(),
            merge_pattern: MergeDefinition::from_subject(&subject),
        };
        def.write(handle, None, None);

        def
    }

    pub fn load(&mut self, handle: &AppHandle){
        match self.read(handle){
            Some(x) => return,
            None => handle.logger(|lgr| lgr.log_error("Failed to load MergeDefintion: Operation returned Null", "DatabaseMergeDefinition_Load", LogVisibility::Both)),
        }
    }

    pub fn from_id(id: &str) -> DatabaseMergeDefinition {
        DatabaseMergeDefinition {
            id: id.to_owned(),
            ..Default::default()
        }
    }
}

impl DatabaseItem for DatabaseMergeDefinition {
    type Item = DatabaseMergeDefinition;

    fn parse(&self, stmt: &mut rusqlite::Statement) -> Result<Self::Item, rusqlite::Error> {
        let data = stmt.query_row((), |row| {
            Ok(DatabaseMergeDefinition{
                id: row.get(0)?,
                merge_pattern: serde_json::from_str(&row.get::<usize, String>(1)?).expect("Should be able to deserialize MergeDefinition")
            })
        });

        data
    }

    fn id(&self) -> String {
        self.id.clone()
    }

    fn table(&self) -> crate::database::operations::tables::DatabaseTable {
        crate::database::operations::tables::DatabaseTable::MergeDefinitions
    }

    fn fields<'a>(&self) -> Vec<String> {
        vec!["id", "definition"]
            .iter()
            .map(|x| String::from(*x))
            .collect()
    }

    fn values<'a>(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();

        values.push(self.id.clone().into());
        values.push(serde_json::to_string(&self.merge_pattern).expect("Should be able to serialize MergeDefinition").into());

        values
    }
}