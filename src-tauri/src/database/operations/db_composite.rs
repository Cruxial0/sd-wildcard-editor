use rusqlite::params;
use tauri::AppHandle;

use crate::state::ServiceAccess;

pub fn query_name_by_uuid(uuid: &str, handle: &AppHandle) -> Result<String, rusqlite::Error> {
    let sql = "SELECT name
    FROM wildcards 
    WHERE uuid = ?1

    UNION ALL

    SELECT name
    FROM subjects 
    WHERE uuid = ?1;";

    let result =
        handle.db(|x| x.query_row(sql, params![uuid], |r| r.get::<usize, String>(0)));
    
    match result {
        Ok(x) => Ok(x),
        Err(x) => {println!("{}", x); Err(x)},
    }
}
