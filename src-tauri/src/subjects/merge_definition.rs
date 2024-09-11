use serde::{Deserialize, Serialize};

use crate::{
    database::datatypes::{db_subject::DatabaseSubject, db_wildcard::DatabaseWildcard},
    helpers::syntax_utils::WildcardSyntaxExt,
};

use super::{merge_node::MergeNode, node_type::NodeType};

#[derive(Serialize, Deserialize, Default, Clone, Hash, PartialEq, Eq)]
pub struct MergeDefinition {
    pub merge_pattern: Vec<Vec<MergeItem>>,
    pub enabled: bool,
}

#[derive(Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct MergeItem {
    pub node: MergeNode,
    pub enabled: bool,
    pub kind: NodeType,
}

impl MergeDefinition {
    pub fn from_subject(subject: &DatabaseSubject) -> MergeDefinition {
        let mut merge_patterns: Vec<Vec<MergeItem>> = Vec::new();
        let mut fields = MergeDefinition::parse_subjects(subject.subjects());
        fields.append(&mut MergeDefinition::parse_wildcards(subject.wildcards()));

        fields
            .iter()
            .for_each(|field| merge_patterns.push(vec![field.clone()]));

        MergeDefinition {
            merge_pattern: merge_patterns,
            enabled: true,
        }
    }

    pub fn parse_subjects(subjects: &Vec<DatabaseSubject>) -> Vec<MergeItem> {
        let mut fields: Vec<MergeItem> = Vec::new();
        for subject in subjects {
            let field = MergeItem {
                node: MergeNode::Wildcard(subject.uuid.to_wildcard()),
                enabled: true,
                kind: NodeType::Subject,
            };
            fields.push(field);
        }
        fields
    }

    pub fn parse_wildcards(wildcards: &Vec<DatabaseWildcard>) -> Vec<MergeItem> {
        let mut fields: Vec<MergeItem> = Vec::new();
        for wildcard in wildcards {
            let field = MergeItem {
                node: MergeNode::Wildcard(wildcard.uuid.to_wildcard()),
                enabled: true,
                kind: NodeType::Wildcard,
            };
            fields.push(field);
        }
        fields
    }
}

impl Serialize for MergeItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("MergeDefinition", 2)?;
        state.serialize_field("node", &self.node.to_string())?;
        state.serialize_field("enabled", &self.enabled)?;
        state.serialize_field("kind", &self.kind)?;
        state.end()
    }
}
