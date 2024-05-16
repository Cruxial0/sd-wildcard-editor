use rusqlite::{params_from_iter, types::Value};
use tauri::AppHandle;

use crate::{helpers::vec_utils::{rusqlite_value_to_csv, to_comma_seperated}, logging::logger, state::ServiceAccess};

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
    if exists(app, data).expect("Something went wrong when checking if Database entry exists") {
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
    else {
        insert(app, data);
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

    let sql = format!("UPDATE {} SET {} WHERE id = {}", data.table().to_str(), key_value_placeholders_from_fields(&f), data.id());
    // let debug_sql = format!("UPDATE {} SET {} WHERE id = {}", data.table().to_str(), debug_key_value_placeholders_from_fields(&f, &v), data.id());
    app.db(|x| {let mut stmt = x.prepare(&sql).expect("Should be able to prepare SQL query");
        let result = stmt.execute(params_from_iter(v));

        match result {
            Ok(_) => {
                let msg = format!("Updated database with: '{}'", sql);
                app.logger(|logger| logger.log_info(&msg, "DatabaseGenericInsert", logger::LogVisibility::Both))
            },
            Err(e) => {
                let err = &format!("An error occured: {:?}", e);
                app.logger(|logger| logger.log_error(&err, "DatabaseGenericInsert", logger::LogVisibility::Backend))
            }
        }
    });
}

/// Writes a new record to the database. 
/// Will error if the primary key already exists.
pub fn insert<T: DatabaseItem>(app: &AppHandle, data: &T) {
    let fields = to_comma_seperated(&data.fields());
    let sql = format!("INSERT INTO {} ({}) VALUES ({});", data.table().to_str(), fields, value_placeholders_from_fields(&fields));
    let debug_sql = format!("INSERT INTO {} ({}) VALUES ({});", data.table().to_str(), fields, rusqlite_value_to_csv(&data.values()));
    app.db(|x| {
        let mut stmt = x.prepare(&sql).expect("Should be able to prepare SQL query");
        let result = stmt.execute(params_from_iter(data.values()));

        match result {
            Ok(_) => {
                let msg = &format!("Wrote into database with: '{}'", debug_sql);
                app.logger(|logger| logger.log_info(&msg, "DatabaseGenericWrite", logger::LogVisibility::Backend))
            }
            Err(e) => {
                let err = &format!("An error occured: {:?}", e);
                app.logger(|logger| logger.log_error(&err, "DatabaseGenericWrite", logger::LogVisibility::Backend))
            }
        }
    });
}