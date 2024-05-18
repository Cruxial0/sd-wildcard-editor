use std::{borrow::BorrowMut, collections::HashMap, fs::{self, FileType}, io::Error};

use tauri::AppHandle;
use walkdir::{DirEntry, WalkDir};

use crate::{database::{datatypes::{db_project::DatabaseProject, db_wildcard::DatabaseWildcard, db_workspace::Workspace}, operations::db_item::DatabaseItem}, logging::logger::LogVisibility, state::ServiceAccess};

fn get_project_index_by_name(projects: &Vec<DatabaseProject>, name: String) -> Option<usize>{
    projects.iter()
        .enumerate()
        .filter(|(_, r)| r.name == name)
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>()
        .first().copied()
}

fn get_parent(entry: &DirEntry) -> Option<&str>{
    entry.path().parent().unwrap().file_name().unwrap().to_str()
}

fn add_project_entry<'a>(handle: &AppHandle, projects: &'a mut Vec<DatabaseProject>, entry: &DirEntry) {
    let parent = get_parent(entry).expect("Parent should exist").to_owned();
    let path = entry.path().to_string_lossy().to_string();
    let logger = *handle.get_logger();
    
    let mut project_buffer: DatabaseProject;
    let mut project: &mut DatabaseProject = match get_project_index_by_name(projects, parent.clone()) {
        Some(idx) => {
            projects.get_mut(idx).unwrap()
        },
        None => {
            project_buffer = DatabaseProject::from_direntry(handle, parent.clone()).unwrap();
            projects.push(project_buffer);
            projects.last_mut().unwrap()
        }
    };

    if entry.file_type().is_dir() { 
        let p = DatabaseProject::from_direntry(handle, entry.file_name().to_str().unwrap().to_owned()).unwrap();
        project.add_project(&p); 
        projects.push(p);
    }
    else if entry.file_type().is_file() {
        match entry.path().extension() {
            Some(ext) => {
                logger.log_trace(&format!("Loading File: {:?}", entry.file_name()), "AddProjectEntry", LogVisibility::Backend);
                if ext == "txt" { project.add_wildcard(&DatabaseWildcard::from_direntry(handle, entry)) }
            },
            None => {
                let msg = format!("Skipped '{:?}': File does not have an extension.", entry.file_name());
                logger.log_warn(&msg, "AddProjectEntry", LogVisibility::Backend);
            },
        }
    }
}

pub fn parse_directory_chain(handle: &AppHandle, dir: &str){
    let mut projects: Vec<DatabaseProject> = Vec::new();
    let items = WalkDir::new(dir).follow_links(true);

    // depth 0 = base folder, depth 1 = loose files, depth >1 = files within directories
    for item in items {
        let entry = item.expect("Entry should be valid");
        // if entry.file_type().is_dir() { continue; }

        match entry.depth() {
            0 => (),
            1 => add_project_entry(handle, &mut projects, &entry),
            2.. => add_project_entry(handle, &mut projects, &entry)
        }
    }

    for pr in projects {
        pr.write(handle, None, None);
        for wc in pr.wildcards {
            wc.write(handle, None, None);
        }
    }
}