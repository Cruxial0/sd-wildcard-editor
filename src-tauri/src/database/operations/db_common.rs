use tauri::AppHandle;

use crate::state::ServiceAccess;
use super::tables::DatabaseTable;

pub fn exists(app: AppHandle, id: u32, table: DatabaseTable) -> bool {
    let exists = app.db(|x| x.execute("SELECT EXISTS(SELECT 1 FROM ?1 where ID = ?2)", (table.to_str(), id)));
    match exists {
        Ok(x) => x != 0,
        Err(_) => false,
    }
}