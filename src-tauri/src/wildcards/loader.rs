use std::{collections::HashMap, path::PathBuf};
use tauri::AppHandle;
use walkdir::{DirEntry, WalkDir};

use crate::{database::{datatypes::{db_project::DatabaseProject, db_settings::DatabaseSettings, db_wildcard::DatabaseWildcard}, operations::db_item::DatabaseItem}, logging::logger::LogVisibility, state::ServiceAccess};

use super::directory_parser::parse_directory_chain;

#[tauri::command]
pub fn load_wildcard_db(app: AppHandle) -> DatabaseWildcard{      
    let wc = DatabaseWildcard::default();
    let mut project = DatabaseProject::default();
    project.add_wildcard(&wc);
    parse_directory_chain(&app, &get_public_directory());

    // project.write(&app, None, None);

    wc
}

fn get_public_directory() -> String {
    let root: PathBuf = project_root::get_project_root().expect("Could not file project root");
    let path: PathBuf = [root.to_str().unwrap(), "..", "public", "wildcards"].iter().collect();
    String::from(path.to_str().expect("Could not convert path to string."))
}