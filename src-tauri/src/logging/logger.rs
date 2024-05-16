use chrono::{DateTime, Local};
use tauri::{command::CommandArg, AppHandle, Manager};

use crate::logging::logger_helpers::{
    format_content_backend, format_datetime_backend, format_source_backend,
};

use super::{
    log_level::LogLevel,
    logger_helpers::{format_content_frontend, format_datetime_frontend, format_source_frontend},
    logger_settings::DATETIME_FORMAT,
};

static LOG_EVENT: &str = "console-log";

pub enum LogVisibility {
    Backend = 0,
    Frontend = 1,
    Both = 2,
}

#[derive(serde::Serialize, Clone)]
struct LogPackage {
    strings: Vec<String>,
    styles: Vec<String>,
    severity: LogLevel,
}

#[derive(Clone)]
pub struct Logger {
    app_handle: Option<AppHandle>,
    pub log_level: LogLevel,
}


impl Logger {
    /// Prints a result to the rust console
    fn internal_log(&self, content: String, date_time: String, source: String, is_error: bool) {
        println!(
            "[{}] {} | {}",
            format_datetime_backend(date_time, is_error),
            format_source_backend(source, is_error),
            format_content_backend(content, is_error)
        );
    }

    /// Prints a result to the frontend console
    fn external_log(&self, content: String, date_time: String, source: String, is_error: bool, log_level: LogLevel) {
        let frontend_content = format_content_frontend(content, is_error);
        let frontend_datetime = format_datetime_frontend(date_time);
        let frontend_source = format_source_frontend(source, is_error);

        let package = LogPackage{
            strings: vec![frontend_datetime.0, frontend_source.0, frontend_content.0],
            styles: vec![frontend_datetime.1, frontend_source.1, frontend_content.1],
            severity: log_level
        };

        if let Some(x) = &self.app_handle {
            x.emit_all(LOG_EVENT, package);
        }
    }

    fn stage(&self, content: &str, source: &str, visibility: LogVisibility, is_error: bool, log_level: LogLevel) {

        if log_level > self.log_level {
            return;
        }
        let current_local: DateTime<Local> = Local::now();
        let date_time = current_local.format(DATETIME_FORMAT);

        match visibility {
            LogVisibility::Backend => self.internal_log(
                content.to_string(),
                date_time.to_string(),
                source.to_owned(),
                is_error,
            ),
            LogVisibility::Frontend => self.external_log(
                content.to_string(),
                date_time.to_string(),
                source.to_owned(),
                is_error,
                log_level
            ),
            LogVisibility::Both => {
                self.internal_log(
                    content.to_string(),
                    date_time.to_string(),
                    source.to_owned(),
                    is_error,
                );
                self.external_log(
                    content.to_string(),
                    date_time.to_string(),
                    source.to_owned(),
                    is_error,
                    log_level
                );
            }
        }
    }

    pub fn log_fatal(&self, content: &str, source: &str, visibility: LogVisibility) {
        self.stage(content, source, visibility, true, LogLevel::FATAL)
    }

    pub fn log_error(&self, content: &str, source: &str, visibility: LogVisibility) {
        self.stage(content, source, visibility, true, LogLevel::ERROR)
    }

    pub fn log_warn(&self, content: &str, source: &str, visibility: LogVisibility) {
        self.stage(content, source, visibility, false, LogLevel::WARN)
    }

    pub fn log_info(&self, content: &str, source: &str, visibility: LogVisibility) {
        self.stage(content, source, visibility, false, LogLevel::INFO)
    }

    pub fn log_debug(&self, content: &str, source: &str, visibility: LogVisibility) {
        self.stage(content, source, visibility, false, LogLevel::DEBUG)
    }

    pub fn log_trace(&self, content: &str, source: &str, visibility: LogVisibility) {
        self.stage(content, source, visibility, false, LogLevel::TRACE)
    }

    pub fn initialize_logger(handle: &AppHandle) -> Logger {
        Logger {
            app_handle: Some(handle.clone()),
            log_level: LogLevel::INFO,
        }
    }

    pub fn set_log_level(&mut self, log_level: LogLevel) {
        if self.log_level > log_level {}
        self.log_level = log_level
    }
}
