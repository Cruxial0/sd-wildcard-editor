use std::{borrow::BorrowMut, collections::HashMap, fs::{self, FileType}, io::Error, path::PathBuf, time::Instant};

use tauri::AppHandle;
use walkdir::{DirEntry, WalkDir};

use crate::{database::{datatypes::{db_files::DatabaseTrackedFiles, db_project::DatabaseProject, db_wildcard::DatabaseWildcard, db_workspace::Workspace}, operations::{db_item::DatabaseItem, tables::DatabaseTable}}, logging::logger::LogVisibility, state::ServiceAccess};

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

fn add_project_entry<'a>(handle: &AppHandle, projects: &'a mut Vec<DatabaseProject>, entry: &DirEntry, tracked_files: &mut DatabaseTrackedFiles) {
    let parent = get_parent(entry).expect("Parent should exist").to_owned();
    let path = entry.path().to_string_lossy().to_string();

    if tracked_files.file_exists(PathBuf::from(&path), true) {
        return;
    }

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

    let start = Instant::now();
    let mut files = 0;
    let mut directories = 0;
    let mut tracked_files = match DatabaseTrackedFiles::default().read(&handle){
        Some(x) => x,
        None => DatabaseTrackedFiles::default(),
    };

    // depth 0 = base folder, depth 1 = loose files, depth >1 = files within directories
    for item in items {
        let entry = item.expect("Entry should be valid");
        if entry.file_type().is_dir() { directories += 1; }
        if entry.file_type().is_file() { files += 1; }

        handle.logger(|lgr| lgr.log_debug(&format!("Parsing {:?}", entry.file_name()), "ParseDir", LogVisibility::Backend));

        match entry.depth() {
            0 => (),
            1 => add_project_entry(handle, &mut projects, &entry, &mut tracked_files),
            2.. => add_project_entry(handle, &mut projects, &entry, &mut tracked_files),
            _ => todo!()
        }
    }

    // Initialize a default workspace ahead of time to ensure a workspace is always generated
    let mut workspace: Workspace = Workspace::default();

    if projects.len() > 0 {
        workspace = Workspace::from_project(handle, &projects.remove(0));
        workspace.wildcards().iter().for_each(|w| w.write(handle, None, None));
        workspace.projetcs().iter().for_each(|p| p.write(handle, None, None));

        for pr in projects {
            pr.write(handle, None, None);
            for wc in pr.wildcards() {
                wc.write(handle, None, None);
            }
        }

        let duration = start.elapsed();

        let msg = format!("Loaded {} projects and {} wildcards in {:?}", directories, files, duration);
        handle.logger(|lgr| lgr.log_info(&msg, "ParseDirectory", LogVisibility::Backend))
    }
    
    tracked_files.write(handle, None, None);
    workspace.write(handle, None, None);
    
}