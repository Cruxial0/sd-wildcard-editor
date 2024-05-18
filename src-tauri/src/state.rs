// Inspired by https://github.com/RandomEngy/tauri-sqlite

use rusqlite::Connection;
use tauri::{AppHandle, State, Manager};

use crate::{database::database_session::DatabaseSession, logging::logger::{self, Logger}};

#[derive(Default)]
pub struct AppState {
    pub db: std::sync::Mutex<Option<Connection>>,
    pub logger: std::sync::Mutex<Option<Logger>>,
    pub session: std::sync::Mutex<Option<DatabaseSession>>
}

pub trait ServiceAccess {
    fn db<'a, F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&Connection) -> TResult;
    fn db_mut<'a, F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&mut Connection) -> TResult;

    fn db_session<'a, F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&mut DatabaseSession) -> TResult;

    fn logger<'a, F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&Logger) -> TResult;
    fn get_logger(&self) -> Box<Logger>;
}

impl ServiceAccess for AppHandle {
    fn db<'a, F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&Connection) -> TResult {
        let app_state: State<AppState> = self.state();
        let db_connection_guard = app_state.db.lock().unwrap();
        let db = db_connection_guard.as_ref().unwrap();

        operation(db)
    }

    fn db_mut<'a, F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&mut Connection) -> TResult {
        let app_state: State<AppState> = self.state();
        let mut db_connection_guard = app_state.db.lock().unwrap();
        let db = db_connection_guard.as_mut().unwrap();

        operation(db)
    }
    
    fn logger<'a, F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&Logger) -> TResult {
        let app_state: State<AppState> = self.state();
        let logger_guard = app_state.logger.lock().unwrap();
        let logger = logger_guard.as_ref().unwrap();

        operation(logger)
    }
    
    fn get_logger(&self) -> Box<Logger> {
        let app_state: State<AppState> = self.state();
        let logger_guard = app_state.logger.lock().unwrap();
        let logger = logger_guard.as_ref().unwrap();

        Box::new(logger.clone())
    }
    
    fn db_session<'a, F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&mut DatabaseSession) -> TResult {
        let app_state: State<AppState> = self.state();
        let mut session_guard = app_state.session.lock().unwrap();
        let session = session_guard.as_mut().unwrap();

        operation(session)
    }
}