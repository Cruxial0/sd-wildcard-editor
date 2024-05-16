// Inspired by https://github.com/RandomEngy/tauri-sqlite

use rusqlite::Connection;
use std::fs;
use tauri::AppHandle;

use crate::{logging::logger::{self, Logger}, state::ServiceAccess};

use super::migration_handler;

const CURRENT_DB_VERSION: u32 = 2;
const DEBUG: bool = true;
static LOG_SOURCE: &str = "DatabaseInitialize";

/// Initializes the database connection, creating the .sqlite file if needed, and upgrading the database
/// if it's out of date.
pub fn initialize_database(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let sqlite_path = app_dir.join("SD-WILDCARD-EDITOR.sqlite");

    app_handle.logger(|x| x.log_info(&format!("{:?}", &sqlite_path), LOG_SOURCE, logger::LogVisibility::Backend));
    let logger = *app_handle.get_logger();

    let mut db = Connection::open(sqlite_path)?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| Ok(row.get(0)?))?;
    drop(user_pragma);

    upgrade_database_if_needed(&mut db, existing_user_version, &logger)?;

    Ok(db)
}

/// Upgrades the database to the current version.
pub fn upgrade_database_if_needed(db: &mut Connection, existing_version: u32, logger: &Logger) -> Result<(), rusqlite::Error> {
    if existing_version < CURRENT_DB_VERSION || DEBUG {
        db.pragma_update(None, "journal_mode", "WAL")?;

        let mut tx = db.transaction()?;

        tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;

        migration_handler::apply_migrations(&mut tx, CURRENT_DB_VERSION, logger);

        tx.commit()?;
    }

    Ok(())
}

// pub fn add_item(title: &str, db: &Connection) -> Result<(), rusqlite::Error> {
//     let mut statement = db.prepare("INSERT INTO items (title) VALUES (@title)")?;
//     statement.execute(named_params! { "@title": title })?;

//     Ok(())
// }

// pub fn get_all(db: &Connection) -> Result<Vec<String>, rusqlite::Error> {
//     let mut statement = db.prepare("SELECT * FROM items")?;
//     let mut rows = statement.query([])?;
//     let mut items = Vec::new();
//     while let Some(row) = rows.next()? {
//         let title: String = row.get("title")?;

//         items.push(title);
//     }

//     Ok(items)
// }
