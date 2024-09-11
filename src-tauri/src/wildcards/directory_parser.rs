use std::{
    borrow::BorrowMut,
    collections::HashMap,
    fs::{self, FileType},
    io::Error,
    path::{Path, PathBuf},
    time::{Duration, Instant},
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
    }, helpers::{dir_utils::get_parent, path_utils::pathbuf_filename}, logging::logger::LogVisibility, state::ServiceAccess
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

fn get_or_create_parent_subject<'c>(subjects: &'c mut Vec<DatabaseSubject>, parent: PathBuf, handle: &AppHandle) -> &'c mut DatabaseSubject {
    let mut subject_buffer: DatabaseSubject;
    let mut subject: &mut DatabaseSubject = match get_subject_index_by_name(subjects, pathbuf_filename(&parent)) {
        Some(idx) => {
            
            subjects.get_mut(idx).unwrap()
        },
        None => {
            subject_buffer = DatabaseSubject::from_parent(handle, &parent).unwrap();
            subjects.push(subject_buffer);
            subjects.last_mut().unwrap()
        }
    };
    subject
}

fn add_wildcard_to_subject(entry: &DirEntry, logger: Box<crate::logging::logger::Logger>, subject: &mut DatabaseSubject, handle: &AppHandle) {
    logger.log_trace("Entered function: add_wildcard_to_subject", "AddWildcardTosubject", LogVisibility::Backend);
    match entry.path().extension() {
        Some(ext) => {
            logger.log_trace(&format!("Loading File: {:?}", entry.file_name()), "AddProjectEntry", LogVisibility::Backend);
            if ext == "txt" {
                subject.add_wildcard(&DatabaseWildcard::from_direntry(handle, entry))
            }
        }
        None => {
            let msg = format!("Skipped '{:?}': File does not have an extension.", entry.file_name());
            logger.log_warn(&msg, "AddProjectEntry", LogVisibility::Backend);
        }
    }
}

fn add_subject_entry<'a>(handle: &AppHandle, subjects: &'a mut Vec<DatabaseSubject>, entry: &DirEntry, tracked_files: &mut DatabaseTrackedFiles) {
    let parent = get_parent(entry).expect("Parent should exist").to_owned();
    let path = entry.path().to_string_lossy().to_string();

    let logger = handle.get_logger();

    logger.log_trace("Entered function: add_subject_entry", "AddSubjectEntry", LogVisibility::Backend);

    if !tracked_files.verify_file(entry, handle) {
        return;
    }

    // Try to retrieve parent subject, or create a new one if it does not exist
    let mut subject = get_or_create_parent_subject(subjects, parent, handle);

    // Add new subject or wildcard to parent subject
    if entry.file_type().is_dir() {
        logger.log_trace("Found directory", "AddSubjectEntry", LogVisibility::Backend);
        let p = DatabaseSubject::from_direntry(handle, entry).unwrap();
        subject.add_subject(&p);
        subjects.push(p);
    } 
    else if entry.file_type().is_file() {
        logger.log_trace("Found file", "AddSubjectEntry", LogVisibility::Backend);
        add_wildcard_to_subject(entry, logger, subject, handle);
    }
}

pub fn parse_directory_chain(handle: &AppHandle, dir: &str) {
    let mut subjects: Vec<DatabaseSubject> = Vec::new();
    let mut items = WalkDir::new(dir).follow_links(true);

    let start = Instant::now();
    let mut files = 0;
    let mut directories = 0;
    let mut tracked_files = DatabaseTrackedFiles::default().read(&handle);

    let logger = handle.get_logger();

    logger.log_trace("Entered function: parse_directory_chain", "ParseDirectoryChain", LogVisibility::Backend);

    // depth 0 = base folder, depth 1 = loose files, depth >1 = files within directories
    for item in items {
        let entry = item.expect("Entry should be valid");

        // Increment debug variables
        if entry.file_type().is_dir() { directories += 1; }
        if entry.file_type().is_file() { files += 1; }

        logger.log_debug(&format!("Parsing {:?}", entry.file_name()), "ParseDir", LogVisibility::Backend);

        match entry.depth() {
            0 => (),
            1.. => add_subject_entry(handle, &mut subjects, &entry, &mut tracked_files),
            _ => logger.log_warn(&format!("Tried to load {:?} at unknown depth", entry.file_name()), "ParseDir", LogVisibility::Both)
        }
    }
    let debug: &Vec<String> = &subjects.iter().map(|x| x.name.clone()).collect();

    // Initialize a default workspace ahead of time to ensure a workspace is always generated
    // First tries loading a workspace from the database, or creates one from the bottom-most subject if it doesn't exist
    let mut workspace: Workspace =
        match Workspace::from_id(&Uuid::nil().to_string()).read_db(handle) {
            Some(x) => match x.read_db(handle) {
                Some(mut x) => {
                    x.load(handle, true);
                    x
                }
                None => {
                    if(subjects.len() > 0) {
                        Workspace::from_subject(handle, &subjects.remove(0))
                    } else {
                        Workspace::default()
                    }
                    
                },
            },
            None => {
                if(subjects.len() > 0) {
                    Workspace::from_subject(handle, &subjects.remove(0))
                } else {
                    Workspace::default()
                }   
            }
        };

    workspace
        .wildcards()
        .iter()
        .for_each(|w| w.write_db(handle, None, None));

    let duration_parse = start.elapsed();
    let msg_parse = format!("Parsed {} subjects and {} wildcards in {:?}", directories, files, duration_parse);
    logger.log_info(&msg_parse, "ParseDirectory", LogVisibility::Backend);

    // Write all items to database and initialize default merge definitions
    if subjects.len() > 0 {
        workspace
            .projetcs()
            .iter()
            .for_each(|p| p.write_db(handle, None, None));

        for mut subj in subjects {
            subj.write_db(handle, None, None);
            for wc in subj.wildcards() {
                wc.write_db(handle, None, None);
            }
            subj.initialize_default_merge_definition(handle);
        }

        let duration_db = start.elapsed() - duration_parse;

        let msg_db = format!("Wrote {} subjects and {} wildcards to database in {:?}", directories, files, duration_db);
        logger.log_info(&msg_db, "ParseDirectory", LogVisibility::Backend);
    } else {
        logger.log_warn("Failed to write commit to database: No subjects were found", "ParseDirectory", LogVisibility::Backend);
    }

    workspace.write_db(handle, None, None);
}
