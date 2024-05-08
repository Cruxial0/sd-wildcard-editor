use tauri::AppHandle;

use crate::{logging::logger, state::ServiceAccess};

use super::{db_item::DatabaseItem, tables::DatabaseTable};

pub fn load<T: DatabaseItem>(app: AppHandle, pk: u32, table: DatabaseTable, item: &T) -> Option<T> {
    let sql = format!("SELECT {} FROM {} WHERE ID = {}", item.fields(), table.to_str(), pk);
    let data: Option<T> = app.db_mut(|x| {
        // Prepare a query, then pass returned sqlite::Statement to DatabaseItem::parse, then finally match the returned value.
        match x.prepare(&sql).and_then(|mut s| Ok(item.parse(&mut s))).expect("") {
            Some(x) => {
                logger::log(&format!("Loaded value from database using: '{}'", sql), "DatabaseGenericLoad", logger::LogVisibility::Backend);
                Some(x)
            },
            None => {
                logger::log_error(&format!("Failed to load data from database using: '{}'", sql), "DatabaseGenericLoad", logger::LogVisibility::Backend);
                None
            }
        }
    });
    
    data
}

pub fn load_all<T: DatabaseItem>(app: AppHandle, pk: u32, table: DatabaseTable, item: &T) -> Option<Vec<T>> {
    todo!()
}