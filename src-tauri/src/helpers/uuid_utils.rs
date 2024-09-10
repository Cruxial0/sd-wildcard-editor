use tauri::AppHandle;
use uuid::Uuid;

use crate::database::operations::db_composite::query_name_by_uuid;

#[tauri::command]
pub fn get_uuid() -> String{
    Uuid::new_v4().to_string()
}

#[tauri::command]
pub fn get_name_by_uuid(handle: AppHandle, uuid: String) -> String {
    match query_name_by_uuid(&uuid, &handle) {
        Ok(name) => name,
        Err(_) => String::from("UNDEFINED"),
    }
}