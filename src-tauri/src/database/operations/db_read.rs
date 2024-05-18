use std::fmt::format;

use tauri::AppHandle;

use crate::{
    helpers::vec_utils::to_comma_seperated, logging::logger::LogVisibility, state::ServiceAccess,
};

use super::db_item::DatabaseItem;

pub fn load<T: DatabaseItem>(app: &AppHandle, item: &T) -> Option<T> {
    let sql = format!(
        "SELECT {} FROM {} WHERE id = {};",
        to_comma_seperated(&item.fields()),
        item.table().to_str(),
        item.id()
    );
    let data: Option<T> = app.db_mut(|x| {
        // Prepare a query, then pass returned sqlite::Statement to DatabaseItem::parse, then finally match the returned value.
        match x.prepare(&sql).and_then(|mut s| Ok(item.parse(&mut s))).expect("")
        {
            Ok(x) => {
                let msg = format!("Loaded value from database using: '{}'", sql);
                app.logger(|logger| {
                    logger.log_info(&msg, std::module_path!(), LogVisibility::Backend)
                });
                Some(x)
            }
            Err(_) => {
                let err = &format!("Failed to load data from database using: '{}'", sql);
                app.logger(|logger| {
                    logger.log_error(&err, std::module_path!(), LogVisibility::Frontend)
                });
                None
            }
        }
    });

    data
}

pub fn load_multiple<T: DatabaseItem>(app: &AppHandle, data: &T, ids: Vec<u32>) -> Option<Vec<T>> {
    let mut items: Vec<T> = Vec::new();

    for id in ids {
        match load(app, data) {
            Some(x) => items.push(x),
            None => {
                app.logger(|lgr| lgr.log_error("", "LoadMultiple", LogVisibility::Backend))
            },
        }
    }
    Some(items)
}
