use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum MergeNode {
    Wildcard(String),
    Text(String)
}

impl ToString for MergeNode {
    fn to_string(&self) -> String {
        match self {
            MergeNode::Wildcard(x) => x.clone(),
            MergeNode::Text(x) => x.clone(),
        }
    }
}

impl From<String> for MergeNode {
    fn from(value: String) -> Self {
        if value.starts_with("__") && value.ends_with("__") { return MergeNode::Wildcard(value); }
        else { return MergeNode::Text(value); }
    }
}

impl Into<rusqlite::types::Value> for MergeNode {
    fn into(self) -> rusqlite::types::Value {
        rusqlite::types::Value::Text(self.to_string())
    }
}

impl<'de> Deserialize<'de> for MergeNode{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer);
        Ok(MergeNode::from(s.unwrap()))
    }
}