// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod wildcards;

use wildcards::loader;

fn main() {
    tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![
            loader::load_wildcard,
            loader::load_wildcards

        ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
}
