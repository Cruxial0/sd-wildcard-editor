use rusqlite::Error;
use tauri::AppHandle;

use crate::{logging::logger::LogVisibility, state::ServiceAccess};

use super::{db_item::DatabaseItem, tables::DatabaseTable};

pub fn exists<T: DatabaseItem>(handle: &AppHandle, data: &T) -> Result<bool, Error> {
    let logger: Box<crate::logging::logger::Logger> = handle.get_logger();

    logger.log_trace("Entered function: exists<T>", "DatabaseExists", LogVisibility::Backend);

    let sql = format!("SELECT * FROM {} where uuid = \"{}\";", data.table().to_str(), data.id());
    let exists: Result<String, Error> = handle.db(|x| {
        x.query_row(&sql, (), |r| r.get(0))
    });
    
    match exists {
        Ok(x) => Ok(true),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            logger.log_error(&format!("QueryReturnedNoRows for query: {}", sql), "DatabaseExists", LogVisibility::Backend);
            Ok(false)
        },
        Err(e) => {
            let err = format!("An error occured ({:?}): {}", e.sqlite_error_code(), e);
            logger.log_error(&err, "DatabaseExists", LogVisibility::Backend);
            Err(e)
        }
    }
}

pub fn get_unique_id(handle: &AppHandle, table: &DatabaseTable) -> Result<u32, Error> {
    let logger: Box<crate::logging::logger::Logger> = handle.get_logger();

    logger.log_trace("Entered function: get_unique_id", "DatabaseGetUniqueID", LogVisibility::Backend);

    let sql = format!("SELECT * FROM {} ORDER BY uuid DESC LIMIT 1", table.to_str());
    let id: Result<u32, Error> = handle.db(|x| {
        x.query_row(&sql, (), |r| r.get(0))
    });

    match id {
        Ok(x) => Ok(x + 1),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            logger.log_error(&format!("QueryReturnedNoRows for query: {}", sql), "DatabaseGetUniqueID", LogVisibility::Backend);
            Ok(0)
        },
        Err(e) => {
            let err = format!("An error occured ({:?}): {}", e.sqlite_error_code().unwrap(), e);
            logger.log_error(&err, "DatabaseGetUniqueID", LogVisibility::Backend);
            Err(e)
        }
    }
}