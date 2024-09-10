use rusqlite::types::Value;
use serde::Serialize;
use tauri::AppHandle;
use uuid::Uuid;

use crate::{
    database::operations::db_item::DatabaseItem, deployment::{deploy_node::DeployNode, deployable::Deployable}, helpers::{syntax_utils::WildcardSyntaxExt, uuid_utils::get_name_by_uuid}, logging::{log_level::LogLevel, logger::LogVisibility}, state::ServiceAccess, subjects::{
        merge_definition::{MergeDefinition, MergeItem},
        merge_node::MergeNode, node_type::NodeType,
    }
};

use super::db_subject::DatabaseSubject;

#[derive(Serialize, Default, Clone, Hash, PartialEq, Eq)]
pub struct DatabaseMergeDefinition {
    pub id: String,
    pub name: String,
    merge_definition: MergeDefinition,
}

impl DatabaseMergeDefinition {
    pub fn create_default(subject: DatabaseSubject, handle: &AppHandle) -> DatabaseMergeDefinition {
        let def = DatabaseMergeDefinition {
            id: Uuid::now_v7().to_string(),
            name: subject.name.to_owned(),
            merge_definition: MergeDefinition::from_subject(&subject),
        };
        def.write_db(handle, None, None);

        def
    }

    pub fn load(&mut self, handle: &AppHandle) {
        match self.read_db(handle) {
            Some(x) => return,
            None => handle.logger(|lgr| {
                lgr.log_error(
                    "Failed to load MergeDefintion: Operation returned Null",
                    "DatabaseMergeDefinition_Load",
                    LogVisibility::Both,
                )
            }),
        }
    }

    pub fn from_id(id: &str) -> DatabaseMergeDefinition {
        DatabaseMergeDefinition {
            id: id.to_owned(),
            ..Default::default()
        }
    }

    fn get_lines(&self, handle: &AppHandle) -> Vec<Vec<String>> {
        let mut lines: Vec<Vec<String>> = Vec::new();

        for fields in &self.merge_definition.merge_pattern {
            let mut active_fields: Vec<&MergeItem> = fields.iter().filter(|x| x.enabled).collect();
            let literals: Vec<String> = active_fields.iter().map(|x| self.node_to_literal(&x.node, handle)).collect();
            lines.push(literals);
        }

        lines
    }
 
    fn node_to_literal(&self, node: &MergeNode, handle: &AppHandle) -> String {
        match node {
            MergeNode::Wildcard(x) => get_name_by_uuid(handle.clone(), x.to_owned().from_wildcard().unwrap()),
            MergeNode::Text(x) => x.to_owned(),
        }
    }
}

impl Deployable for DatabaseMergeDefinition {
    fn generate_deploy_node(&self, path: impl AsRef<std::path::Path>, handle: &AppHandle) -> Option<DeployNode> {
        if !self.merge_definition.enabled { return None };
        let lgr = handle.get_logger();

        lgr.log_debug("Deploying merge definitions", "MergeDefsDeploy", LogVisibility::Backend);
        let lines: Vec<String> = self.get_lines(handle).iter().map(|x| x.join(", ")).collect();

        Some(DeployNode::new(lines, path.as_ref().join(self.name.clone() + ".txt"), Vec::new()))
    }
}

impl DatabaseItem for DatabaseMergeDefinition {
    type Item = DatabaseMergeDefinition;

    fn parse(&self, stmt: &mut rusqlite::Statement) -> Result<Self::Item, rusqlite::Error> {
        let data = stmt.query_row((), |row| {
            Ok(DatabaseMergeDefinition {
                id: row.get(0)?,
                name: row.get(1)?,
                merge_definition: serde_json::from_str(&row.get::<usize, String>(2)?)
                    .expect("Should be able to deserialize MergeDefinition"),
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
        vec!["uuid", "name", "definition"]
            .iter()
            .map(|x| String::from(*x))
            .collect()
    }

    fn values<'a>(&self) -> Vec<Value> {
        let mut values: Vec<Value> = Vec::new();

        values.push(self.id.clone().into());
        values.push(self.name.clone().into());
        values.push(
            serde_json::to_string(&self.merge_definition)
                .expect("Should be able to serialize MergeDefinition")
                .into(),
        );

        values
    }
}
