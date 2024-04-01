// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs,
    path::{Path, PathBuf},
};

struct Wildcard {
    name: String,
    content: Vec<String>,
}

impl Wildcard {
    fn new(name: &str, content: Vec<String>) -> Wildcard {
        Wildcard {
            name: String::from(name),
            content: content,
        }
    }
}

impl serde::Serialize for Wildcard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Wildcard", 2)?;
        state.serialize_field("name", &self.name.to_string())?;
        state.serialize_field("content", &self.content)?;
        state.end()
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_wildcard(name: String) -> Wildcard {
    let root = get_public_directory();
    let base_path = Path::new(root.as_str()).join("wildcards");
    let path = walk_directory(
        String::from(base_path.to_str().unwrap()),
        Some(vec!["txt"]),
        Some(name),
    );
    let content = fs::read_to_string(&path.first().unwrap()).expect("Could not read file.");
    Wildcard::new(
        path.first().unwrap().to_str().unwrap().split('\\').last().unwrap(),
        content.split_whitespace().map(|v| v.to_string()).collect(),
    )
}

fn load_wildcards_from_paths(paths: Vec<PathBuf>) -> Vec<Wildcard> {
    let mut wildcards: Vec<Wildcard> = vec![];
    for path in paths {
        let content = fs::read_to_string(&path).expect("Could not read file.");
        let wildcard = Wildcard::new(
            path.to_str().unwrap().split('\\').last().unwrap(),
            content.split_whitespace().map(|v| v.to_string()).collect(),
        );
        wildcards.push(wildcard);
    }
    wildcards
}

#[tauri::command]
fn load_wildcards() -> Vec<Wildcard> {
    let root = get_public_directory();
    let path = Path::new(root.as_str()).join("wildcards");
    load_wildcards_from_paths(walk_directory(
        String::from(path.to_str().unwrap()),
        Some(vec!["txt"]),
        None,
    ))
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            load_wildcard,
            load_wildcards
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
