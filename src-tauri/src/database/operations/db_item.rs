use rusqlite::{types::Value, Statement};
use tauri::AppHandle;

use crate::{logging::logger::LogVisibility, state::ServiceAccess};

use super::{db_common, db_read, db_write, tables::DatabaseTable};

pub trait DatabaseItem: Default {
    type Item;

    fn parse(&self, stmt: &mut Statement) -> Result<Self, rusqlite::Error>;
    fn id(&self) -> String;
    fn table(&self) -> DatabaseTable;
    fn fields<'a>(&self) -> Vec<String>;
    fn values<'a>(&self) -> Vec<rusqlite::types::Value>;

    fn write_db(&self, handle: &AppHandle, field_filter: Option<&str>, values_filter: Option<Vec<Value>>) {
        &handle.logger(|logger| logger.log_trace("Entered function: write_db", "DatabaseItem", LogVisibility::Backend));
        db_write::write_or_insert(handle, self, field_filter, values_filter)
    }

    fn read_db(&self, handle: &AppHandle) -> Option<Self> {
        &handle.logger(|logger| logger.log_trace("Entered function: read_db", "DatabaseItem", LogVisibility::Backend));
        db_read::load(handle, self)
    }
}
