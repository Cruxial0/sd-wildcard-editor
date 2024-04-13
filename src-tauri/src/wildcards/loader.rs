use std::{fs, path::{Path, PathBuf}};

use super::wildcard_data::{SimpleWildcard, Wildcard};

#[tauri::command]
pub fn load_wildcard(name: String) -> SimpleWildcard {
    let root = get_public_directory();
    let base_path = Path::new(root.as_str()).join("wildcards");
    let path = walk_directory(
        String::from(base_path.to_str().unwrap()),
        Some(vec!["txt"]),
        Some(name),
    );
    let content = fs::read_to_string(&path.first().unwrap()).expect("Could not read file.").lines().map(|x| x.to_string()).collect();
    SimpleWildcard::new(
        path.first().unwrap().to_str().unwrap().split('\\').last().unwrap(),
        content,
        path.first().unwrap().to_owned()
    )
}

#[tauri::command]
pub fn load_wildcards() -> Vec<SimpleWildcard> {
    let root = get_public_directory();
    let path = Path::new(root.as_str()).join("wildcards");
    load_wildcards_from_paths(walk_directory(
        String::from(path.to_str().unwrap()),
        Some(vec!["txt"]),
        None,
    ))
}

pub fn write_wildcard(wildcard: SimpleWildcard){
    wildcard.write();
}

fn load_wildcards_from_paths(paths: Vec<PathBuf>) -> Vec<SimpleWildcard> {
    let mut wildcards: Vec<SimpleWildcard> = vec![];
    for path in paths {
        let content = fs::read_to_string(&path).expect("Could not read file.");
        let wildcard = SimpleWildcard::new(
            path.to_str().unwrap().split('\\').last().unwrap(),
            content.split_whitespace().map(|v| v.to_string()).collect(),
            (&path).to_owned()
        );
        wildcards.push(wildcard);
    }
    wildcards
}

fn walk_directory(
    path: String,
    extensions: Option<Vec<&str>>,
    name_filter: Option<String>,
) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = vec![];
    let ext = extensions.as_ref();

    for entry in walkdir::WalkDir::new(path.to_owned())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();

        if ext.is_some_and(|_| !ext.unwrap().contains(&f_name.split(".").last().unwrap())) {
            continue;
        }
        if name_filter.as_ref().is_some_and(|f| !f_name.contains(f)) {
            continue;
        }

        files.push(entry.path().to_path_buf());
    }

    files
}

fn get_public_directory() -> String {
    let root: PathBuf = project_root::get_project_root().expect("Could not file project root");
    let path: PathBuf = [root.to_str().unwrap(), "..", "public"].iter().collect();
    String::from(path.to_str().expect("Could not convert path to string."))
}