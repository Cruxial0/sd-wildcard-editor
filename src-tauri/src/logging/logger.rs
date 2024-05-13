use chrono::{DateTime, Local};
use tauri::{command::CommandArg, AppHandle};

use crate::logging::logger_helpers::{format_datetime_backend, format_source_backend, format_content_backend};

use super::{logger_helpers::{format_content_frontend, format_datetime_frontend, format_source_frontend}, logger_settings::DATETIME_FORMAT};

pub enum LogVisibility {
    Backend = 0,
    Frontend = 1,
    Both = 2
}

pub enum LogSeverity {

}

/// Prints a result to the rust console
fn internal_log(content: String, date_time: String, source: String, is_error: bool){
    println!("[{}] {} | {}", format_datetime_backend(date_time, is_error), format_source_backend(source, is_error), format_content_backend(content, is_error));
}

/// Prints a result to the frontend console
fn external_log(content: String, date_time: String, source: String, is_error: bool){
    let frontend_content = format_content_frontend(content, is_error);
    let frontend_datetime = format_datetime_frontend(date_time);
    let frontend_source = format_source_frontend(source, is_error);

    todo!(); // send payload to frontend console (needs access to AppHandle::emit_all)
}

fn stage(content: &str, source: &str, visibility: LogVisibility, is_error: bool){
    let current_local: DateTime<Local> = Local::now();
    let date_time = current_local.format(DATETIME_FORMAT);

    match visibility {
        LogVisibility::Backend => internal_log(content.to_string(), date_time.to_string(), source.to_owned(), is_error),
        LogVisibility::Frontend => external_log(content.to_string(), date_time.to_string(), source.to_owned(), is_error),
        LogVisibility::Both => {
            internal_log(content.to_string(), date_time.to_string(), source.to_owned(), is_error);
            external_log(content.to_string(), date_time.to_string(), source.to_owned(), is_error);
        },
    }
}

pub fn log(content: &str, source: &str, visibility: LogVisibility){
    stage(content, source, visibility, false)
}

pub fn log_error(content: &str, source: &str, visibility: LogVisibility){
    stage(content, source, visibility, true)
}