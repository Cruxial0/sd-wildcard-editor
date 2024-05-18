use rusqlite::Error;
use tauri::AppHandle;

use crate::{logging::logger::LogVisibility, state::ServiceAccess};

use super::{db_item::DatabaseItem, tables::DatabaseTable};

pub fn exists<T: DatabaseItem>(app: &AppHandle, data: &T) -> Result<bool, Error> {
    let exists: Result<u32, Error> = app.db(|x| {
        let sql = format!("SELECT * FROM {} where ID = {};", data.table().to_str(), data.id());
        x.query_row(&sql, (), |r| r.get(0))
    });
    
    match exists {
        Ok(x) => Ok(x > 0),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
        Err(e) => {
            let err = format!("An error occured ({:?}): {}", e.sqlite_error_code().unwrap(), e);
            app.logger(|logger| logger.log_error(&err, std::module_path!(), LogVisibility::Backend));
            Err(e)
        }
    }
}

pub fn get_unique_id(app: &AppHandle, table: &DatabaseTable) -> Result<u32, Error> {
    let id: Result<u32, Error> = app.db(|x| {
        let sql = format!("SELECT * FROM {} ORDER BY id DESC LIMIT 1", table.to_str());
        x.query_row(&sql, (), |r| r.get(0))
    });

    match id {
        Ok(x) => Ok(x + 1),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
        Err(e) => {
            let err = format!("An error occured ({:?}): {}", e.sqlite_error_code().unwrap(), e);
            app.logger(|logger| logger.log_error(&err, std::module_path!(), LogVisibility::Backend));
            Err(e)
        }
    }
}