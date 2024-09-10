use std::{borrow::Cow, default, fmt::format, fs, io::{self, Error}, os::windows::fs::MetadataExt, path::PathBuf, str::FromStr, time::SystemTime};

use chrono::{DateTime, Utc};
use itertools::Itertools;
use rusqlite::{params, types::Value, Params};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Sha512, Digest};
use tauri::AppHandle;
use uuid::Uuid;
use walkdir::DirEntry;

use crate::{database::operations::{db_item::DatabaseItem, tables::DatabaseTable}, helpers::dir_utils, logging::logger::LogVisibility, state::ServiceAccess};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct TrackedFile {
    pub uuid: String,
    pub data: String,
    pub path: PathBuf
}

impl TrackedFile {
    fn set_id(&mut self, id: String) {
        self.uuid = id
    }

    fn set_path(&mut self, path: String) {
        self.path = PathBuf::from(path)
    }
}


pub struct DatabaseTrackedFiles {
    uuid: String,
    files: Vec<TrackedFile>
}

impl Default for DatabaseTrackedFiles {
    fn default() -> Self {
        Self { uuid: String::new(), files: Vec::new() }
    }
}

impl DatabaseTrackedFiles {

    pub fn read(&mut self, handle: &AppHandle) -> Self{
        let entries = handle.db(|conn| {
            let mut stmt = conn.prepare("SELECT uuid, data, path FROM TrackedFiles").unwrap();
            let entry_iter = stmt.query_map([], |row| {
                Ok(TrackedFile{
                    uuid: row.get(0)?,
                    data: row.get(1)?,
                    path: PathBuf::from(row.get::<usize, String>(2)?)
                })
            }).unwrap();

            let file_entries: Vec<TrackedFile> = entry_iter.map(|x| x.unwrap()).collect();
            file_entries
        });

        DatabaseTrackedFiles {
            uuid: Uuid::nil().to_string(),
            files: entries
        }
    }

    pub fn get_uuid_of_dir_entry(file: &walkdir::DirEntry, handle: &AppHandle) -> String {
        let content = DatabaseTrackedFiles::get_file_content(file, handle).unwrap();
        let data_hash = DatabaseTrackedFiles::hash_file_data(&content, file.metadata().unwrap().creation_time());
        Uuid::new_v5(&Uuid::NAMESPACE_X500, &data_hash.as_bytes()).to_string()
    }

    pub fn update_row(handle: &AppHandle, sql: String) {
        handle.db(|conn| {
            let mut stmt = conn.prepare(&sql).expect("Should be able to prepare SQL query");
            let result = stmt.execute([]);

            match result {
                Ok(x) => handle.logger(|lgr| lgr.log_debug(&format!("Query Succeeded. {:?} rows affected.", x), "TrackedFilesUpdate", LogVisibility::Backend)),
                Err(e) => handle.logger(|lgr| lgr.log_debug(&format!("Query Failed: {:?}.", e), "TrackedFilesUpdate", LogVisibility::Backend)),
            }
        })
    }

    fn hash_file_path(path: &std::path::PathBuf) -> String {
        let path_str = path.to_string_lossy();
        let mut hasher = Sha256::new();
        hasher.update(path_str.as_bytes());
        let result = hasher.finalize();
        
       format!("{:x}", result)
    }

    pub fn hash_file_data(data: &Vec<String>, creation_time: u64) -> String {
        let mut joined = data.join("");
        joined.push_str(&creation_time.to_string());
        let mut hasher = Sha256::new();
        hasher.update(joined.as_bytes());
        let result = hasher.finalize();

        format!("{:x}", result)
    }

    fn get_file_content(file: &walkdir::DirEntry, handle: &AppHandle) -> Result<Vec<String>, Error> {
        let mut meta = file.metadata().unwrap();

        if !meta.is_dir() {
            if meta.is_file(){
                if file.path().extension().unwrap() != "txt" {return Err(io::Error::new(io::ErrorKind::Unsupported, "Can't parse files with an unsupported format."))};
            } else {return Err(io::Error::new(io::ErrorKind::Unsupported, "Can't parse files with an unsupported format."))};
        }
        
        let mut data: Vec<String> = match meta.is_dir() {
            true => {
                handle.logger(|lgr| lgr.log_trace(&format!("Detected directory: {:?}", file.path().display()), "VerifyFile", LogVisibility::Backend));
                dir_utils::get_list_of_files(file.path())
            },
            false => fs::read_to_string(file.path()).unwrap().lines().map(|l| l.to_string()).collect(),
        };

        Ok(data)
    }

    fn get_file_status(&mut self, data_hash: &String, path: &String) -> FileStatus {
        let data_exists = self.files.iter().filter(|x| x.uuid == data_hash.clone()).next().is_some();
        let path_exists = self.files.iter().filter(|x| x.path == PathBuf::from(path.clone())).next().is_some();

        match (data_exists, path_exists) {
            (true, true) => FileStatus::Exists,
            (true, false) => FileStatus::FileMoved,
            (false, true) => FileStatus::ContentChanged,
            (false, false) => FileStatus::NotFound,
        }
    }

    fn update_entry(&mut self, file_status: FileStatus, uuid: &String, data_hash: &String, path: &String, handle: &AppHandle) {
        match file_status {
            FileStatus::Exists => (),
            FileStatus::ContentChanged => {
                let mut items = self.files.iter_mut().filter(|x| x.path == PathBuf::from(path.clone())).collect::<Vec<&mut TrackedFile>>();
                if let Some(first) = items.first_mut() {
                    first.set_id(uuid.to_owned());
                    DatabaseTrackedFiles::update_row(handle, format!("UPDATE TrackedFiles SET data={:?} WHERE path={:?}", data_hash, path))
                }
            },
            FileStatus::FileMoved => {
                let mut items = self.files.iter_mut().filter(|x| x.data == data_hash.clone()).collect::<Vec<&mut TrackedFile>>();
                if let Some(first) = items.first_mut() {
                    handle.logger(|lgr| lgr.log_info(&format!("{:?} -> {:?}", &first.path, &path), "VerifyFile_Moved", LogVisibility::Backend));
                    first.set_path(path.to_owned());
                    DatabaseTrackedFiles::update_row(handle, format!("UPDATE TrackedFiles SET path={:?} WHERE data={:?}", path, data_hash))
                }
            },
            FileStatus::NotFound => {
                self.files.push(TrackedFile { uuid: uuid.to_owned(), data: data_hash.to_owned(), path: PathBuf::from(&path) });
                DatabaseTrackedFiles::update_row(handle, format!("INSERT INTO TrackedFiles (uuid, data, path) VALUES ({:?}, {:?}, {:?})", uuid, data_hash, path))
            },
        }
    }

    pub fn verify_file(&mut self, file: &walkdir::DirEntry, handle: &AppHandle) -> bool {
    
        let data = match DatabaseTrackedFiles::get_file_content(&file, handle){
            Ok(x) => Some(x),
            Err(e) => {
                handle.logger(|lgr| lgr.log_info(&format!("Error while getting file data: {:?}", e), "VerifyFile", LogVisibility::Backend));
                None
            },
        };
        
        if data.is_none() {return false;}

        let data_hash = DatabaseTrackedFiles::hash_file_data(&data.unwrap(), file.metadata().unwrap().creation_time());
        handle.logger(|lgr| lgr.log_trace(&format!("Hashed content: {:?}", data_hash), "VerifyFile", LogVisibility::Backend));
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_X500, &data_hash.as_bytes());
        let path = &file.path().to_str().unwrap().to_string();

        let file_status = self.get_file_status(&uuid.to_string(), &path);

        match file_status {
            FileStatus::Exists => {
                handle.logger(|lgr| lgr.log_debug(&format!("Found tracked file: {:?}", file.path().display()), "VerifyFile", LogVisibility::Backend));
                false
            },
            FileStatus::ContentChanged => { 
                handle.logger(|lgr| lgr.log_info(&format!("Detected changed content in file: {:?}", file.path().display()), "VerifyFile", LogVisibility::Backend));
                self.update_entry(FileStatus::ContentChanged, &uuid.to_string(), &data_hash, &path, handle);
                
                false
                },
            FileStatus::FileMoved => { 
                handle.logger(|lgr| lgr.log_info(&format!("Detected moved file: {:?}", file.path().display()), "VerifyFile", LogVisibility::Backend));
                self.update_entry(FileStatus::FileMoved, &uuid.to_string(), &data_hash, &path, handle);
                
                false
                },
            FileStatus::NotFound => { 
                handle.logger(|lgr| lgr.log_info(&format!("Detected new file: {:?}", file.path().display()), "VerifyFile", LogVisibility::Backend));
                self.update_entry(FileStatus::NotFound, &uuid.to_string(), &data_hash, &path, handle); 
                
                true
            },
        }
    }
}

#[derive(Debug)]
pub enum FileStatus {
    Exists = 0,
    ContentChanged = 1,
    FileMoved = 2,
    NotFound = 3,
}