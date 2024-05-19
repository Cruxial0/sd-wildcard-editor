use std::{collections::HashMap, path::PathBuf};
use tauri::AppHandle;
use walkdir::{DirEntry, WalkDir};

use crate::{database::{datatypes::{db_project::DatabaseProject, db_settings::DatabaseSettings, db_wildcard::DatabaseWildcard, db_workspace::Workspace}, operations::db_item::DatabaseItem}, logging::logger::LogVisibility, state::ServiceAccess};

use super::directory_parser::parse_directory_chain;

#[tauri::command]
pub fn load_workspace(handle: AppHandle) -> Workspace{
    parse_directory_chain(&handle, &get_public_directory());
    let mut workspace = Workspace::from_id(&0).read(&handle).unwrap();

    workspace.load(&handle, true);

    workspace
}

#[tauri::command]
pub fn load_wildcard(handle: AppHandle, id: u32) -> DatabaseWildcard {
    DatabaseWildcard::from_id(&id).read(&handle).unwrap()
}

fn get_public_directory() -> String {
    let root: PathBuf = project_root::get_project_root().expect("Could not file project root");
    let path: PathBuf = [root.to_str().unwrap(), "..", "public", "wildcards"].iter().collect();
    String::from(path.to_str().expect("Could not convert path to string."))
}