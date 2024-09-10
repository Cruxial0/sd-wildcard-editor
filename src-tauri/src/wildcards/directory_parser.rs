use std::{
    borrow::BorrowMut,
    collections::HashMap,
    fs::{self, FileType},
    io::Error,
    path::{Path, PathBuf},
    time::Instant,
};

use tauri::AppHandle;
use uuid::Uuid;
use walkdir::{DirEntry, WalkDir};

use crate::{
    database::{
        datatypes::{
            db_files::DatabaseTrackedFiles, db_subject::DatabaseSubject,
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

fn get_parent(entry: &DirEntry) -> Option<&Path> {
    entry.path().parent()
}

fn add_subject_entry<'a>(
    handle: &AppHandle,
    subjects: &'a mut Vec<DatabaseSubject>,
    entry: &DirEntry,
    tracked_files: &mut DatabaseTrackedFiles,
) {
    let parent = get_parent(entry).expect("Parent should exist").to_owned();
    let path = entry.path().to_string_lossy().to_string();

    if !tracked_files.verify_file(entry, handle) {
        return;
    }

    let logger = *handle.get_logger();

    let mut project_buffer: DatabaseSubject;
    let mut subject: &mut DatabaseSubject =
        match get_subject_index_by_name(subjects, parent.file_name().unwrap().to_string_lossy().to_string().clone()) {
            Some(idx) => subjects.get_mut(idx).unwrap(),
            None => {
                project_buffer = DatabaseSubject::from_parent(handle, &parent).unwrap();
                subjects.push(project_buffer);
                subjects.last_mut().unwrap()
            }
        };

    if entry.file_type().is_dir() {
        let p =
            DatabaseSubject::from_direntry(handle, entry)
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
    let mut items = WalkDir::new(dir).follow_links(true);

    let start = Instant::now();
    let mut files = 0;
    let mut directories = 0;
    let mut tracked_files = DatabaseTrackedFiles::default().read(&handle);

    // depth 0 = base folder, depth 1 = loose files, depth >1 = files within directories
    for item in items {
        let entry = item.expect("Entry should be valid");
        if entry.file_type().is_dir() { directories += 1; }
        if entry.file_type().is_file() { files += 1; }

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
    let mut workspace: Workspace = match Workspace::from_id(&Uuid::nil().to_string()).read_db(handle) {
        Some(x) => {match x.read_db(handle) {
            Some(mut x) => {x.load(handle, true); x},
            None => Workspace::from_subject(handle, &subjects.remove(0)),
        }},
        None => Workspace::from_subject(handle, &subjects.remove(0)),
    };

    if subjects.len() > 0 {
        workspace
            .wildcards()
            .iter()
            .for_each(|w| w.write_db(handle, None, None));
        workspace
            .projetcs()
            .iter()
            .for_each(|p| p.write_db(handle, None, None));

        for mut subj in subjects {
            subj.write_db(handle, None, None);
            for wc in subj.wildcards() {
                wc.write_db(handle, None, None);
            }
            subj.initialize_merge_definition(handle);

        }

        let duration = start.elapsed();

        let msg = format!(
            "Loaded {} subjects and {} wildcards in {:?}",
            directories, files, duration
        );
        handle.logger(|lgr| lgr.log_info(&msg, "ParseDirectory", LogVisibility::Backend))
    }

    workspace.write_db(handle, None, None);
}
