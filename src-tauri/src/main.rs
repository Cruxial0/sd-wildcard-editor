// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Temporary - Mutes unused warnings; Remove when production ready
#![allow(unused)] 

#[macro_use]
extern crate lazy_static;

mod database;
mod wildcards;
mod state;
mod logging;
mod helpers;
mod cli_arguments;

use logging::{log_level::LogLevel, logger::LogVisibility};
use state::ServiceAccess;
use tauri::{State, Manager};
use wildcards::loader;
use crate::state::AppState;

fn main() {
    
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            loader::load_workspace,
            loader::load_wildcard
        ])
        .setup(|app| {
            let handle = app.handle();

            let mut cli_args = cli_arguments::CliArguments::default();
            cli_args.match_args(app.get_cli_matches());

            let app_state: State<AppState> = handle.state();

            let mut logger = logging::logger::Logger::initialize_logger(&handle);
            logger.set_log_level(cli_args.get_log_level());
            *app_state.logger.lock().unwrap() = Some(logger);

            let db = database::helper::initialize_database(&handle).expect("Database initialize should succeed");
            *app_state.db.lock().unwrap() = Some(db);

            let session = database::database_session::DatabaseSession::initialize(&handle);
            *app_state.session.lock().unwrap() = Some(session);

            let lgr = *handle.get_logger();
            &lgr.log_error("Test error :D", "AppInitialize", LogVisibility::Backend);
            &lgr.log("A custom log level", "AppInitialize", LogVisibility::Backend, "CUSTOM");
            &lgr.log_trace("Test trace", "AppInitialize", LogVisibility::Backend);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
