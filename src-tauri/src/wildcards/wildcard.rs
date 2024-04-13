use std::{fs, path::{Path, PathBuf}};

pub trait Wildcard{
    fn write(&self);
    fn get_data(&self) -> &WildcardData;
    fn set_content(&mut self, content: Vec<String>);
}

#[derive(PartialEq)]
pub struct WildcardData{
    pub name: String,
    pub content: Vec<String>,
    pub abs_path: PathBuf
}

impl WildcardData{
    fn new(name: &str, content: Vec<String>, path: PathBuf) -> Self{
        WildcardData{
            name: String::from(name),
            content: content,
            abs_path: path
        }
    }

    fn set_content(&mut self, content: Vec<String>){
        self.content = content;
    }
}

#[derive(PartialEq)]
pub struct SimpleWildcard {
    data: WildcardData
}

impl SimpleWildcard {
    pub fn new(name: &str, content: Vec<String>, path: PathBuf) -> SimpleWildcard {
        SimpleWildcard {
            data: WildcardData::new(name, content, path),
        }
    }
}

impl Wildcard for SimpleWildcard {
    fn write(&self) {
        let text = self.data.content.join("\n");
        let mut path = self.data.abs_path.to_str().expect("Could not unwrap path").to_owned();
        path.push_str("_new");
        fs::write(path, text).expect("unable to write path");
    }

    fn get_data(&self) -> &WildcardData {
        &self.data
    }

    fn set_content(&mut self, content: Vec<String>) {
        self.data.set_content(content)
    }
}

impl serde::Serialize for SimpleWildcard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Wildcard", 2)?;
        state.serialize_field("name", &self.data.name.to_string())?;
        state.serialize_field("content", &self.data.content)?;
        state.end()
    }
}