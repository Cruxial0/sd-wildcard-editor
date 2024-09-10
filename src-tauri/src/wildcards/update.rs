use tauri::AppHandle;

use crate::{database::{datatypes::db_wildcard::DatabaseWildcard, operations::db_item::DatabaseItem}, logging::logger::LogVisibility, state::ServiceAccess};

#[tauri::command]
pub fn update_wildcard(handle: AppHandle, uuid: String, lines: Vec<String>) {

    if let Some(wildcard) = DatabaseWildcard::from_id(&uuid).read_db(&handle) {
        match wildcard.update_content(&handle, lines) {
            Ok(_) => (),
            Err(e) => handle.logger(|lgr| lgr.log_error(&format!("Error encountered when saving wildcard: {:?}", e), "UpdateWildcard", LogVisibility::Both)),
        }
    }
}