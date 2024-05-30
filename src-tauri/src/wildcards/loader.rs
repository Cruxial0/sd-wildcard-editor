use std::{collections::HashMap, path::PathBuf};
use tauri::AppHandle;
use walkdir::{DirEntry, WalkDir};

use crate::{database::{datatypes::{db_project::DatabaseProject, db_settings::DatabaseSettings, db_wildcard::DatabaseWildcard, db_workspace::Workspace}, operations::db_item::DatabaseItem}, logging::logger::LogVisibility, state::ServiceAccess};

use super::directory_parser::parse_directory_chain;

#[tauri::command]
pub fn load_workspace(handle: AppHandle) -> Workspace{
    parse_directory_chain(&handle, &get_public_directory());
    let mut workspace = match Workspace::from_id(&0).read(&handle) {
        Some(w) => w,
        None => Workspace::from_id(&0),
    };

    workspace.load(&handle, true);

    workspace
}

#[tauri::command]
pub fn load_wildcard(handle: AppHandle, id: u32) -> Option<DatabaseWildcard> {
    let wc = DatabaseWildcard::from_id(&id);
    wc.read(&handle)
}

fn get_public_directory() -> String {
    let root = std::env::current_exe().unwrap();
    let parent = root.parent().unwrap();
    let str_path: String = [parent.to_str().unwrap(), "\\public", "\\wildcards"].iter().map(|p| String::from(*p)).collect();
    println!("{}", str_path);
    let path: PathBuf = get_or_create_path(&str_path).unwrap();
    String::from(path.to_str().expect("Could not convert path to string."))
}

fn get_or_create_path(path: &str) -> Result<PathBuf, std::io::Error> {
    match std::fs::create_dir_all(path) {
        Ok(_) => Ok(PathBuf::from(path)),
        Err(e) => Err(e),
    }
}