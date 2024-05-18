use chrono::{DateTime, Local};
use tauri::{command::CommandArg, AppHandle, Manager};

use crate::logging::{logger_helpers::color_backend, logger_settings::DATETIME_COLOR};

use super::{
    log_level::LogLevel, logger_helpers::{adjust_source_length, color_frontend, format_backend, format_frontend}, logger_settings::DATETIME_FORMAT
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
    fn internal_log(&self, content: String, date_time: String, source: String, log_level: &LogLevel) {
        let is_error = log_level == &LogLevel::ERROR;
        println!(
            "{} [{}] {} | {}",
            color_backend(date_time, DATETIME_COLOR),
            format_backend(log_level.to_string(), log_level),
            format_backend(adjust_source_length(source, log_level), log_level),
            format_backend(content, log_level)
        );
    }

    /// Prints a result to the frontend console
    fn external_log(&self, content: String, date_time: String, source: String, log_level: &LogLevel) {
        let is_error = log_level == &LogLevel::ERROR;
        let frontend_datetime = color_frontend(date_time, DATETIME_COLOR);
        let frontend_loglevel = format_frontend(log_level.to_string(), log_level);
        let frontend_source = format_frontend(source, log_level);
        let frontend_content = format_frontend(content, log_level);

        let package = LogPackage{
            strings: vec![frontend_datetime.0, frontend_loglevel.0, frontend_source.0, frontend_content.0],
            styles: vec![frontend_datetime.1, frontend_loglevel.1, frontend_source.1, frontend_content.1],
            severity: log_level.to_owned()
        };

        if let Some(x) = &self.app_handle {
            x.emit_all(LOG_EVENT, package);
        }
    }

    fn stage(&self, content: &str, source: &str, visibility: LogVisibility, log_level: LogLevel) {

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
                &log_level,
            ),
            LogVisibility::Frontend => self.external_log(
                content.to_string(),
                date_time.to_string(),
                source.to_owned(),
                &log_level
            ),
            LogVisibility::Both => {
                self.internal_log(
                    content.to_string(),
                    date_time.to_string(),
                    source.to_owned(),
                    &log_level,
                );
                self.external_log(
                    content.to_string(),
                    date_time.to_string(),
                    source.to_owned(),
                    &log_level
                );
            }
        }
    }

    pub fn log_fatal(&self, content: &str, source: &str, visibility: LogVisibility) {
        self.stage(content, source, visibility,  LogLevel::FATAL)
    }

    pub fn log_error(&self, content: &str, source: &str, visibility: LogVisibility) {
        self.stage(content, source, visibility,  LogLevel::ERROR)
    }

    pub fn log_warn(&self, content: &str, source: &str, visibility: LogVisibility) {
        self.stage(content, source, visibility,  LogLevel::WARN)
    }

    pub fn log_info(&self, content: &str, source: &str, visibility: LogVisibility) {
        self.stage(content, source, visibility,  LogLevel::INFO)
    }

    pub fn log_debug(&self, content: &str, source: &str, visibility: LogVisibility) {
        self.stage(content, source, visibility,  LogLevel::DEBUG)
    }

    pub fn log_trace(&self, content: &str, source: &str, visibility: LogVisibility) {
        self.stage(content, source, visibility,  LogLevel::TRACE)
    }

    pub fn log(&self, content: &str, source: &str, visibility: LogVisibility, log_level: &str){
        self.stage(content, source, visibility, LogLevel::Custom(log_level.into()))
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
