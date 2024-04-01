// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::{Path, PathBuf}};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_wildcard() -> String {
    let root = get_public_directory();
    let path = Path::new(root.as_str()).join("wildcard.txt");
    fs::read_to_string(path).expect("Could not read file.")
}

fn get_public_directory() -> String{
    let root: PathBuf = project_root::get_project_root().expect("Could not file project root");
    let path: PathBuf = [root.to_str().unwrap(), "..", "public"].iter().collect();
    String::from(path.to_str().expect("Could not convert path to string."))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, load_wildcard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
