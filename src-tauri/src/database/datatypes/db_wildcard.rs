use std::{
    fs,
    path::{Path, PathBuf},
};

use rusqlite::{Error, Statement};
use tauri::AppHandle;
use uuid::Uuid;
use walkdir::DirEntry;

use crate::{
    database::operations::{db_item::DatabaseItem, db_read, tables::DatabaseTable},
    deployment::{deploy_node::DeployNode, deployable::Deployable},
    helpers::{dir_utils::get_public_directory, file_utils},
    logging::logger,
    state::ServiceAccess,
};

use super::db_files::DatabaseTrackedFiles;

#[derive(Clone, Hash, Eq)]
pub struct DatabaseWildcard {
    pub uuid: String,
    pub name: String,
    pub path: PathBuf,
    pub lines: Vec<String>,
}

impl DatabaseWildcard {
    pub fn from_id(id: &str) -> DatabaseWildcard {
        DatabaseWildcard {
            uuid: id.to_owned(),
            ..Default::default()
        }
    }

    pub fn from_direntry(handle: &AppHandle, entry: &DirEntry) -> DatabaseWildcard {
        let content: Vec<String> = std::fs::read_to_string(entry.path())
            .expect("File should be readable")
            .lines()
            .map(|l| l.to_string())
            .collect();

        let unique_id = DatabaseTrackedFiles::get_uuid_of_dir_entry(&entry, handle);
        let abs_path = PathBuf::from(entry.path());
        let rel_path = match abs_path.strip_prefix(get_public_directory()) {
            Ok(x) => x,
            Err(e) => {
                println!("{:?}", e);
                Path::new("")
            }
        };

        DatabaseWildcard {
            uuid: unique_id.to_string(),
            name: entry.file_name().to_str().to_owned().unwrap().to_string(),
            path: PathBuf::from(rel_path),
            lines: content,
        }
    }

    pub fn update_content(
        &self,
        handle: &AppHandle,
        lines: Vec<String>,
    ) -> Result<(), std::io::Error> {
        let path = PathBuf::from(get_public_directory()).join(&self.path);
        println!("lines:\n{:?}", &lines);
        println!("writing to path: {:?}", &path);
        fs::write(path, lines.join("\n"))
    }
}

impl Default for DatabaseWildcard {
    fn default() -> Self {
        DatabaseWildcard {
            uuid: Uuid::nil().to_string(),
            name: "New Wildcard".to_owned(),
            path: PathBuf::new(),
            lines: Vec::new(),
        }
    }
}

impl PartialEq for DatabaseWildcard {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Deployable for DatabaseWildcard {
    fn generate_deploy_node(
        &self,
        path: impl AsRef<Path>,
        handle: &AppHandle,
    ) -> Option<crate::deployment::deploy_node::DeployNode> {
        Some(DeployNode::new(
            self.lines.clone(),
            &self.path.clone(),
            Vec::new(),
        ))
    }
}

impl DatabaseItem for DatabaseWildcard {
    type Item = DatabaseWildcard;

    fn parse(&self, stmt: &mut Statement) -> Result<Self, Error> {
        let data = stmt.query_row([], |row| {
            let path: String = row
                .get::<usize, String>(2)
                .expect("Path should be deserializable")
                .into();
            Ok(DatabaseWildcard {
                uuid: row.get(0)?,
                name: row.get(1)?,
                path: PathBuf::from(path),
                ..Default::default()
            })
        });

        data
    }

    // Overridden function to read lines directly from IO
    fn read_db(&self, app: &AppHandle) -> Option<Self> {
        let wildcard = db_read::load(app, self);
        match wildcard {
            Some(wc) => Some(DatabaseWildcard {
                uuid: wc.uuid,
                name: wc.name,
                path: wc.path.clone(),
                lines: file_utils::lines_from_file(&wc.path),
            }),
            None => None,
        }
    }

    fn id(&self) -> String {
        self.uuid.clone()
    }

    fn table(&self) -> DatabaseTable {
        DatabaseTable::Wildcards
    }

    fn fields(&self) -> Vec<String> {
        vec!["uuid", "name", "path"]
            .iter()
            .map(|x| String::from(*x))
            .collect()
    }

    fn values(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<rusqlite::types::Value> = Vec::new();
        let name = self.name.clone();
        values.push(self.uuid.clone().into());
        values.push(name.into());
        values.push(self.path.to_str().unwrap().to_owned().into());
        values
    }
}

impl serde::Serialize for DatabaseWildcard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Wildcard", 3)?;
        state.serialize_field("uuid", &self.uuid)?;
        state.serialize_field("name", &self.name.to_string())?;
        state.serialize_field("content", &self.lines)?;
        state.end()
    }
}
