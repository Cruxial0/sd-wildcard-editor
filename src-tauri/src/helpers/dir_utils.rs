use std::{
    fs,
    path::{Path, PathBuf},
};

use walkdir::DirEntry;

pub fn get_list_of_files(dir_path: &Path) -> Vec<String> {
    let mut children: Vec<String> = Vec::new();
    for file in fs::read_dir(dir_path).unwrap() {
        if let Ok(file) = file {
            let path = file.path();
            let ext = path.extension();
            if let Some(extension) = ext {
                children.push(path.to_str().unwrap().to_string())
            }
        }
    }

    children
}

pub fn get_public_directory() -> String {
    let root = if cfg!(debug_assertions) {
        project_root::get_project_root().unwrap()
    } else {
        std::env::current_exe().unwrap()
    };

    let parent = root.parent().unwrap();
    let str_path: String = [parent.to_str().unwrap(), "\\public", "\\wildcards"]
        .iter()
        .map(|p| String::from(*p))
        .collect();
    let path: PathBuf = get_or_create_dir(str_path).unwrap();
    String::from(path.to_str().expect("Could not convert path to string."))
}

pub fn get_or_create_dir(path: impl AsRef<Path>) -> Result<PathBuf, std::io::Error> {
    match std::fs::create_dir_all(path.as_ref()) {
        Ok(_) => Ok(PathBuf::from(path.as_ref())),
        Err(e) => Err(e),
    }
}

pub fn get_parent(entry: &DirEntry) -> Option<&Path> {
    entry.path().parent()
}
