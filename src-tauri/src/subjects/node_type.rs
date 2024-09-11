use serde::{de::Error, Deserialize, Serialize};

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum NodeType {
    Wildcard = 0,
    Subject = 1,
    Text = 2
}

impl Serialize for NodeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_u8(self.clone() as u8)
    }
}

impl<'de> Deserialize<'de> for NodeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let state = u8::deserialize(deserializer);
        match state {
            Ok(x) => match x {
                x if x == NodeType::Wildcard as u8 => Ok(NodeType::Wildcard),
                x if x == NodeType::Subject as u8 => Ok(NodeType::Subject),
                x if x == NodeType::Text as u8 => Ok(NodeType::Text),
                _ => Err(Error::custom("Couldn't match NodeType"))
            },
            Err(x) => Err(x),
        }
    }
}