use rusqlite::{params_from_iter, types::Value, Statement};
use tauri::AppHandle;

use crate::{helpers::vec_utils::{rusqlite_value_to_csv, to_comma_seperated}, logging::logger::{self, LogVisibility}, state::ServiceAccess};

use super::{db_common::exists, db_item::DatabaseItem};

fn value_placeholders_from_fields(fields: &str) -> String {
    let max = fields.split(",").count();
    let mut placeholder = "".to_owned();

    for i in 0..max {
        let buf = format!("?{}, ", i + 1);
        placeholder.push_str(buf.as_ref());
    }
    placeholder.split_at(&placeholder.len() - 2).0.to_owned()
}

fn key_value_placeholders_from_fields(fields: &str) -> String {
    let parts: Vec<&str> = fields.split(",").map(|x| x.trim()).collect();
    let mut placeholder = "".to_owned();
    let mut i = 0;

    for p in parts {
        let buf = format!("{} = ?{},", p, i + 1);
        placeholder.push_str(buf.as_ref());
        i += 1;
    }
    placeholder.split_at(&placeholder.len() - 1).0.to_owned()
}

fn debug_key_value_placeholders_from_fields(fields: &str, values: &Vec<Value>) -> String {
    let parts: Vec<&str> = fields.split(",").map(|x| x.trim()).collect();
    let mut placeholder = "".to_owned();
    let mut i = 0;

    for p in parts {
        let buf = format!("{} = {:?}, ", p, values[i]);
        placeholder.push_str(buf.as_ref());
        i += 1;
    }
    placeholder.split_at(&placeholder.len() - 2).0.to_owned()
}

/// Write a new record to the database if item does not already exist
/// else, modify the existing value
pub fn write_or_insert<T: DatabaseItem>(app: &AppHandle, data: &T, fields: Option<&str>, values: Option<Vec<Value>>){
    match exists(app, data) {
        Ok(exists) => {
            if !exists {
                return insert(app, data);
            }
            
            app.logger(|logger| logger.log_debug(&format!("Entry exists. Proceeding to update Database Entry."), "WriteOrInsert", LogVisibility::Backend));
            let f = &data.fields();
            let v = data.values();
            let field = match fields{
                Some(x) => x.to_owned(),
                None => to_comma_seperated(f),
            };
            let value: Vec<Value> = match values {
                Some(x) => x.iter().map(|v| v.to_owned()).collect(),
                None => v.iter().map(|x| x.to_owned()).collect(),
            };

            update(app, data, &field, value)
        }
        Err(e) => {
            app.logger(|logger| logger.log_warn(&format!("Encountered a potential fatal error: {:?}", e), "WriteOrInsert", LogVisibility::Backend));
        }
    }
}

/// Modifies the existing record in the database.
/// Will error if the primary key does not exist.
pub fn update<T: DatabaseItem>(app: &AppHandle, data: &T, fields: &str, values: Vec<Value>) {
    let mut stripped_fields: Vec<String> = fields.split(",").into_iter().map(|x| x.to_owned()).collect();
    stripped_fields.remove(0);
    let f = to_comma_seperated(&stripped_fields);
    let mut v = values;
    v.remove(0);

    let sql = format!("UPDATE {} SET {} WHERE uuid = \"{}\"", data.table().to_str(), key_value_placeholders_from_fields(&f), data.id());

    // let debug_sql = format!("UPDATE {} SET {} WHERE id = {}", data.table().to_str(), debug_key_value_placeholders_from_fields(&f, &v), data.id());
    app.db(|x| {let mut stmt = match x.prepare(&sql) {
        Ok(mut stmt) => execute_update(&mut stmt, v, sql, app),
        Err(e) => {
            app.logger(|logger| logger.log_error(&format!("Encountered error when preparing query: {:?}", e), "DatabaseGenericUpdate", LogVisibility::Backend));
            app.logger(|logger| logger.log_debug(&format!("Failed with query: {:?}", sql), "DatabaseGenericUpdate", LogVisibility::Backend))
        }
    };
    });
}

fn execute_update(stmt: &mut Statement, v: Vec<Value>, sql: String, app: &AppHandle) {
    let result = stmt.execute(params_from_iter(v));

    match result {
        Ok(_) => {
            let msg = format!("Updated database with: '{}'", sql);
            app.logger(|logger| logger.log_debug(&msg, "DatabaseGenericUpdate", LogVisibility::Backend))
        },
        Err(e) => {
            let err = &format!("An error occured: {:?}", e);
            app.logger(|logger| logger.log_error(&err, "DatabaseGenericUpdate", LogVisibility::Backend));
            app.logger(|logger| logger.log_debug(&format!("Failed with query: {}", sql), "DatabaseGenericUpdate", LogVisibility::Backend))
        }
    }
}

/// Writes a new record to the database. 
/// Will error if the primary key already exists.
pub fn insert<T: DatabaseItem>(app: &AppHandle, data: &T) {
    let fields = to_comma_seperated(&data.fields());
    let sql = format!("INSERT INTO {} ({}) VALUES ({});", data.table().to_str(), fields, value_placeholders_from_fields(&fields));
    let debug_sql = format!("INSERT INTO {} ({}) VALUES ({});", data.table().to_str(), fields, rusqlite_value_to_csv(&data.values()));
    app.db(|x| {
        let mut stmt = match x.prepare(&sql) {
            Ok(mut stmt) => execute_insert(&mut stmt, data, sql, debug_sql, app),
            Err(e) => {
                app.logger(|logger| logger.log_error(&format!("Encountered error when preparing query: {:?}", e), "DatabaseGenericUpdate", LogVisibility::Backend));
                app.logger(|logger| logger.log_debug(&format!("Failed with query: {:?}", sql), "DatabaseGenericUpdate", LogVisibility::Backend))
            }
        };
    });
}

fn execute_insert<T: DatabaseItem>(stmt:  &mut Statement<'_>, data: &T, sql: String, debug_sql: String, app: &AppHandle) {
    let result = stmt.execute(params_from_iter(data.values()));

    match result {
        Ok(_) => {
            app.logger(|logger| logger.log_debug(&sql, "DatabaseGenericInsert", LogVisibility::Backend))
        }
        Err(e) => {
            let err = &format!("An error occured: {:?}", e);
            app.logger(|logger| logger.log_error(&err, "DatabaseGenericInsert", LogVisibility::Backend));
            app.logger(|logger| logger.log_trace(&format!("Failed with query: {}", debug_sql), "DatabaseGenericInsert", LogVisibility::Backend))
        }
    }
}