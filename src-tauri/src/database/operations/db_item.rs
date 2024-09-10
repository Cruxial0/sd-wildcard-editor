use rusqlite::{types::Value, Statement};
use tauri::AppHandle;

use super::{db_common, db_read, db_write, tables::DatabaseTable};

pub trait DatabaseItem: Default {
    type Item;

    fn parse(&self, stmt: &mut Statement) -> Result<Self, rusqlite::Error>;
    fn id(&self) -> String;
    fn table(&self) -> DatabaseTable;
    fn fields<'a>(&self) -> Vec<String>;
    fn values<'a>(&self) -> Vec<rusqlite::types::Value>;

    fn write_db(&self, app: &AppHandle, field_filter: Option<&str>, values_filter: Option<Vec<Value>>) {
        db_write::write_or_insert(app, self, field_filter, values_filter)
    }

    fn read_db(&self, app: &AppHandle) -> Option<Self> {
        db_read::load(app, self)
    }
}
