use rusqlite::Error;
use tauri::api::dir;

use crate::{database::operations::{db_common::exists, db_item::DatabaseItem, tables::DatabaseTable}, logging::logger};

pub struct DatabaseSettings {
    version: u32,
    tracked_dirs: Vec<String>,
    selected_style: i32
}

impl Default for DatabaseSettings {
    fn default() -> DatabaseSettings {
        DatabaseSettings {
            version: 1,
            tracked_dirs: Vec::new(),
            selected_style: 1
        }
    }
}

impl DatabaseSettings {
    pub fn load_or_default(app: &tauri::AppHandle, version: u32) -> DatabaseSettings {
        
        let settings = DatabaseSettings{version: version, tracked_dirs: Vec::new(), selected_style: 0};
        if !exists(&app, &settings).expect("Something went wrong when checking if Database entry exists") { return DatabaseSettings::default(); }
        match settings.read(app) {
            Some(x) => x,
            None => DatabaseSettings::default(),
        }
    }
    pub fn add_tracked_dir(&mut self, directory: String) {
        if self.tracked_dirs.contains(&directory) { return };
        self.tracked_dirs.push(directory);
    }
}

impl DatabaseItem for DatabaseSettings{
    type Item = DatabaseSettings;

    fn parse(&self, stmt: &mut rusqlite::Statement) -> Result<Self, Error> {
        let data = stmt.query_row((), |row| {
            Ok(DatabaseSettings{
                version: row.get(0)?,
                tracked_dirs: serde_json::from_str(row.get::<usize, String>(1)?.as_str()).expect("JSON Deserialization should succeed"),
                selected_style: row.get(2)?
            })
        });

        data
    }

    fn id(&self) -> u32 {
        self.version
    }

    fn table(&self) -> DatabaseTable {
        DatabaseTable::AppSettings
    }

    fn fields(&self) -> Vec<String> {
        vec!["id", "trackedDirectories", "selectedStyle"]
            .iter().map(|x| String::from(*x)).collect()
    }

    fn values(&self) -> Vec<rusqlite::types::Value> {
        let mut values: Vec<rusqlite::types::Value> = Vec::new();
        values.push(self.version.into());
        values.push(serde_json::to_string(&self.tracked_dirs).expect("JSON serialization should succeed").into());
        values.push(self.selected_style.into());
        values
    }
}