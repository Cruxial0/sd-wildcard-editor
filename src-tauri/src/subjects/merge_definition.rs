use serde::{Deserialize, Serialize};

use crate::database::datatypes::{db_subject::DatabaseSubject, db_wildcard::DatabaseWildcard};

use super::merge_node::MergeNode;

#[derive(Serialize, Deserialize, Default, Clone, Hash, PartialEq, Eq)]
pub struct MergeDefinition {
    pub merge_pattern: Vec<Vec<MergeField>>,
    pub enabled: bool
}

#[derive(Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct MergeField {
    pub merge_pattern: MergeNode,
    pub enabled: bool
}

impl MergeDefinition {

    pub fn from_subject(subject: &DatabaseSubject) -> MergeDefinition {
        let mut merge_patterns: Vec<Vec<MergeField>> = Vec::new();
        let mut fields = MergeDefinition::parse_subjects(subject.subjects());
        fields.append(&mut MergeDefinition::parse_wildcards(subject.wildcards()));

        fields.iter().for_each(|field| merge_patterns.push(vec![field.clone()]));

        MergeDefinition {
            merge_pattern: merge_patterns,
            enabled: true
        }
    }

    pub fn parse_subjects(subjects: &Vec<DatabaseSubject>) -> Vec<MergeField> {
        let mut fields: Vec<MergeField> = Vec::new();
        for subject in subjects {
            let field = MergeField {
                merge_pattern: MergeNode::Wildcard(String::from("__".to_owned() + &subject.id + "__")),
                enabled: true
            };
            fields.push(field);
        };
        fields
    }

    pub fn parse_wildcards(wildcards: &Vec<DatabaseWildcard>) -> Vec<MergeField> {
        let mut fields: Vec<MergeField> = Vec::new();
        for wildcard in wildcards {
            let field = MergeField {
                merge_pattern: MergeNode::Wildcard(String::from("__".to_owned() + &wildcard.id + "__")),
                enabled: true,
            };
            fields.push(field);
        };
        fields
    }


}

impl Serialize for MergeField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            use serde::ser::SerializeStruct;

            let mut state = serializer.serialize_struct("MergeDefinition", 2)?;
            state.serialize_field("merge_pattern", &self.merge_pattern.to_string())?;
            state.serialize_field("enabled", &self.enabled)?;
            state.end()
    }
}