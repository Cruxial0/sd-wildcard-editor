use std::{
    borrow::BorrowMut,
    collections::HashMap,
    fs::{self, FileType},
    io::Error,
    path::PathBuf,
    time::Instant,
};

use tauri::AppHandle;
use walkdir::{DirEntry, WalkDir};

use crate::{
    database::{
        datatypes::{
            db_files::DatabaseTrackedFiles, db_project::DatabaseSubject,
            db_wildcard::DatabaseWildcard, db_workspace::Workspace,
        },
        operations::{db_item::DatabaseItem, tables::DatabaseTable},
    },
    logging::logger::LogVisibility,
    state::ServiceAccess,
};

fn get_subject_index_by_name(subjects: &Vec<DatabaseSubject>, name: String) -> Option<usize> {
    subjects
        .iter()
        .enumerate()
        .filter(|(_, r)| r.name == name)
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>()
        .first()
        .copied()
}

fn get_parent(entry: &DirEntry) -> Option<&str> {
    entry.path().parent().unwrap().file_name().unwrap().to_str()
}

fn add_subject_entry<'a>(
    handle: &AppHandle,
    subjects: &'a mut Vec<DatabaseSubject>,
    entry: &DirEntry,
    tracked_files: &mut DatabaseTrackedFiles,
) {
    let parent = get_parent(entry).expect("Parent should exist").to_owned();
    let path = entry.path().to_string_lossy().to_string();

    if tracked_files.file_exists(PathBuf::from(&path), true) {
        return;
    }

    let logger = *handle.get_logger();

    let mut project_buffer: DatabaseSubject;
    let mut subject: &mut DatabaseSubject =
        match get_subject_index_by_name(subjects, parent.clone()) {
            Some(idx) => subjects.get_mut(idx).unwrap(),
            None => {
                project_buffer = DatabaseSubject::from_direntry(handle, parent.clone()).unwrap();
                subjects.push(project_buffer);
                subjects.last_mut().unwrap()
            }
        };

    if entry.file_type().is_dir() {
        let p =
            DatabaseSubject::from_direntry(handle, entry.file_name().to_str().unwrap().to_owned())
                .unwrap();
        subject.add_subject(&p);
        subjects.push(p);
    } else if entry.file_type().is_file() {
        match entry.path().extension() {
            Some(ext) => {
                logger.log_trace(
                    &format!("Loading File: {:?}", entry.file_name()),
                    "AddProjectEntry",
                    LogVisibility::Backend,
                );
                if ext == "txt" {
                    subject.add_wildcard(&DatabaseWildcard::from_direntry(handle, entry))
                }
            }
            None => {
                let msg = format!(
                    "Skipped '{:?}': File does not have an extension.",
                    entry.file_name()
                );
                logger.log_warn(&msg, "AddProjectEntry", LogVisibility::Backend);
            }
        }
    }
}

pub fn parse_directory_chain(handle: &AppHandle, dir: &str) {
    let mut subjects: Vec<DatabaseSubject> = Vec::new();
    let items = WalkDir::new(dir).follow_links(true);

    let start = Instant::now();
    let mut files = 0;
    let mut directories = 0;
    let mut tracked_files = match DatabaseTrackedFiles::default().read(&handle) {
        Some(x) => x,
        None => DatabaseTrackedFiles::default(),
    };

    // depth 0 = base folder, depth 1 = loose files, depth >1 = files within directories
    for item in items {
        let entry = item.expect("Entry should be valid");
        if entry.file_type().is_dir() {
            directories += 1;
        }
        if entry.file_type().is_file() {
            files += 1;
        }

        handle.logger(|lgr| {
            lgr.log_debug(
                &format!("Parsing {:?}", entry.file_name()),
                "ParseDir",
                LogVisibility::Backend,
            )
        });

        match entry.depth() {
            0 => (),
            1 => add_subject_entry(handle, &mut subjects, &entry, &mut tracked_files),
            2.. => add_subject_entry(handle, &mut subjects, &entry, &mut tracked_files),
            _ => handle.logger(|lgr| {
                lgr.log_warn(
                    &format!("Tried to load {:?} at unknown depth", entry.file_name()),
                    "ParseDir",
                    LogVisibility::Both,
                )
            }),
        }
    }

    // Initialize a default workspace ahead of time to ensure a workspace is always generated
    let mut workspace: Workspace = Workspace::default();

    if subjects.len() > 0 {
        workspace = Workspace::from_subject(handle, &subjects.remove(0));
        workspace
            .wildcards()
            .iter()
            .for_each(|w| w.write(handle, None, None));
        workspace
            .projetcs()
            .iter()
            .for_each(|p| p.write(handle, None, None));

        for pr in subjects {
            pr.write(handle, None, None);
            for wc in pr.wildcards() {
                wc.write(handle, None, None);
            }
        }

        let duration = start.elapsed();

        let msg = format!(
            "Loaded {} subjects and {} wildcards in {:?}",
            directories, files, duration
        );
        handle.logger(|lgr| lgr.log_info(&msg, "ParseDirectory", LogVisibility::Backend))
    }

    tracked_files.write(handle, None, None);
    workspace.write(handle, None, None);
}
