use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::{logging::logger, state::ServiceAccess};

pub trait WildcardFunctionality {
    fn write(&self, app: AppHandle);
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
    Compository(CompositoryWildcard),
}

#[derive(Serialize, Deserialize)]
pub struct SimpleWildcard {
    data: WildcardData,
}

#[derive(Serialize, Deserialize)]
pub struct CompositoryWildcard {
    data: WildcardData,
    children: Vec<Wildcard>,
}

impl SimpleWildcard {
    pub fn new(name: &str, content: Vec<String>, path: PathBuf) -> SimpleWildcard {
        SimpleWildcard {
            data: WildcardData::new(name, content, path),
        }
    }
}

impl CompositoryWildcard {
    pub fn new(name: &str, children: Vec<Wildcard>) -> CompositoryWildcard {
        CompositoryWildcard {
            data: WildcardData::new(name, Vec::new(), PathBuf::new()),
            children: children,
        }
    }
}

impl WildcardFunctionality for SimpleWildcard {
    fn write(&self, app: AppHandle) {
        let lines: String = self.data.content.iter().map(|x| "\"".to_owned() + x + "\", ").collect();
        let data = format!("{}{}{}", "[", lines.split_at(lines.len() - 2).0, "]");

        let sql = "INSERT INTO Wildcards(Name, Path, Lines) VALUES (?1, ?2, ?3)";
        let change = app.db(|x| x.execute(sql, (&self.data.name, self.data.abs_path.to_str(), data)));
        match change {
            Ok(_) => logger::log(&format!("{}{}{}", "Successfully saved '", self.data.name, "' to database"), "WildcardSave", logger::LogVisibility::Backend),
            Err(x) => logger::log_error(&format!("{}{:?}", "An error occured: ", x), "WildcardSave", logger::LogVisibility::Backend),
        }
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
