use std::{collections::HashMap, path::PathBuf};
use tauri::AppHandle;
use uuid::Uuid;
use walkdir::{DirEntry, WalkDir};

use crate::{
    database::{
        datatypes::{
            db_merge_definition::DatabaseMergeDefinition, db_settings::DatabaseSettings,
            db_subject::DatabaseSubject, db_wildcard::DatabaseWildcard, db_workspace::Workspace,
        },
        operations::db_item::DatabaseItem,
    },
    helpers::dir_utils::get_public_directory,
    logging::logger::LogVisibility,
    state::ServiceAccess,
};

use super::directory_parser::parse_directory_chain;

#[tauri::command]
pub fn load_workspace(handle: AppHandle) -> Workspace {
    parse_directory_chain(&handle, &get_public_directory());
    let mut workspace = match Workspace::from_id(&Uuid::nil().to_string()).read_db(&handle) {
        Some(w) => w,
        None => Workspace::from_id(&Uuid::nil().to_string()),
    };

    workspace.load(&handle, true);
    let deployment = workspace.generate_deployment("C:\\Users\\benja\\Documents\\Wildcards", &handle);
    deployment.deploy();

    workspace
}

#[tauri::command]
pub fn load_wildcard(handle: AppHandle, id: String) -> Option<DatabaseWildcard> {
    let wc = DatabaseWildcard::from_id(&id);
    wc.read_db(&handle)
}

#[tauri::command]
pub fn load_project(handle: AppHandle, id: String) -> Result<DatabaseSubject, String> {
    match DatabaseSubject::from_id(&id).read_db(&handle) {
        Some(mut p) => {
            p.load(&handle, true);
            Ok(p)
        }
        None => Err(format!("Failed to load project with id: {:?}", id)),
    }
}

#[tauri::command]
pub fn wildcard_name_from_id(handle: AppHandle, id: String) -> String {
    match DatabaseWildcard::from_id(&id).read_db(&handle) {
        Some(x) => x.name,
        None => String::from("NULL"),
    }
}

#[tauri::command]
pub fn load_merge_definition_from_subject(
    handle: AppHandle,
    id: String,
) -> Result<Vec<DatabaseMergeDefinition>, String> {
    match DatabaseSubject::from_id(&id).read_db(&handle) {
        Some(mut x) => Ok(x.load_merge_definitions(&handle)),
        None => Err("Could not load merge definition".to_owned()),
    }
}
