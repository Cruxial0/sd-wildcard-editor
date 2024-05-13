use std::path::PathBuf;
use tauri::AppHandle;

use crate::database::{datatypes::{db_settings::DatabaseSettings, db_wildcard::DatabaseWildcard}, operations::db_item::DatabaseItem};

#[tauri::command]
pub fn load_wildcard_db(app: AppHandle) -> DatabaseWildcard{      
    let mut db_settings = DatabaseSettings::load_or_default(&app, 1);
    let wc = DatabaseWildcard::default();

    db_settings.add_tracked_dir(get_public_directory());

    db_settings.write(&app, None, None);
    wc
}

fn get_public_directory() -> String {
    let root: PathBuf = project_root::get_project_root().expect("Could not file project root");
    let path: PathBuf = [root.to_str().unwrap(), "..", "public"].iter().collect();
    String::from(path.to_str().expect("Could not convert path to string."))
}
