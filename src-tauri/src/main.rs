// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

mod database;
mod wildcards;
mod state;
mod logging;

use tauri::{State, Manager};
use wildcards::loader;
use crate::state::AppState;

fn main() {

    tauri::Builder::default()
        .manage(AppState{ db: Default::default() })
        .invoke_handler(tauri::generate_handler![
            loader::load_wildcard,
            loader::load_wildcards,
            loader::write_wildcard,
            loader::load_comp_wildcard,
            loader::load_wildcard_db
        ])
        .setup(|app| {
            let handle = app.handle();

            let app_state: State<AppState> = handle.state();
            let db = database::helper::initialize_database(&handle).expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);
            logging::logger::log_error("Test error :D", "AppInitialize", logging::logger::LogVisibility::Backend);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
