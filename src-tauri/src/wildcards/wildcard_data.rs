use std::{fmt::format, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

pub trait WildcardFunctionality {
    fn write(&self);
    fn get_data(&self) -> &WildcardData;
    fn set_content(&mut self, content: Vec<String>);
}

#[derive(Serialize, Deserialize)]
pub struct WildcardData {
    pub name: String,
    pub content: Vec<String>,
    pub abs_path: PathBuf,
}

impl WildcardData {
    fn new(name: &str, content: Vec<String>, path: PathBuf) -> Self {
        WildcardData {
            name: String::from(name),
            content: content,
            abs_path: path,
        }
    }

    fn set_content(&mut self, content: Vec<String>) {
        self.content = content;
    }
}

#[derive(Serialize, Deserialize)]
pub enum Wildcard {
    Simple(SimpleWildcard),
    Compository(CompositoryWildcard)
}

#[derive(Serialize, Deserialize)]
pub struct SimpleWildcard {
    data: WildcardData,
}

#[derive(Serialize, Deserialize)]
pub struct CompositoryWildcard {
    data: WildcardData,
    children: Vec<Wildcard>
}

impl SimpleWildcard {
    pub fn new(name: &str, content: Vec<String>, path: PathBuf) -> SimpleWildcard {
        SimpleWildcard {
            data: WildcardData::new(name, content, path),
        }
    }
}

impl CompositoryWildcard {
    pub fn new(name: &str, children: Vec<Wildcard>) -> CompositoryWildcard{
        CompositoryWildcard {
            data: WildcardData::new(name, Vec::new(), PathBuf::new()),
            children: children
        }
    }
}

impl WildcardFunctionality for SimpleWildcard {
    fn write(&self) {
        let text = self.data.content.join("\n");
        let mut path = self.data.abs_path
            .to_str()
            .expect("Could not unwrap path")
            .to_owned();
        let parts: Vec<String> = path.split(".").map(|x| x.to_owned()).collect();
        let mut part1: String = String::new();
        let _ = &parts[0..parts.len() - 1].iter().for_each(|x| if x != "" {part1 += x});
        part1 = part1.replace("\\\\", "\\..\\");
        let output_path = format!("{0}{1}{2}", part1, "_new.", parts.last().unwrap());

        fs::write(output_path, text).expect("unable to write path");
    }

    fn get_data(&self) -> &WildcardData {
        &self.data
    }

    fn set_content(&mut self, content: Vec<String>) {
        self.data.set_content(content)
    }
}

// impl serde::Serialize for SimpleWildcard {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         use serde::ser::SerializeStruct;

//         let mut state = serializer.serialize_struct("Wildcard", 2)?;
//         state.serialize_field("name", &self.data.name.to_string())?;
//         state.serialize_field("content", &self.data.content)?;
//         state.end()
//     }
// }