use std::collections::HashMap;

use itertools::Itertools;
use tauri::{AppHandle, Error};

use crate::{logging::logger::LogVisibility, state::ServiceAccess};

use super::operations::{db_common, tables::DatabaseTable};

#[derive(Default, Clone)]
pub struct DatabaseSession {
    handle: Option<AppHandle>,
    start: HashMap<DatabaseTable, u32>,
    pub claimed_ids: HashMap<DatabaseTable, Vec<u32>>
}

impl DatabaseSession {
    /// Gets an unclaimed id.
    /// 
    /// If you plan on using this id, claim it using [`Self::claim_id()`], or use [`Self::get_and_claim_id()`] instead.
    /// 
    /// Returns [`tauri::Error::AssetNotFound`] error if [`DatabaseSession`] was not initialized.
    pub fn get_unclaimed_id(&mut self, table: DatabaseTable) -> Result<u32, Error> {
        if self.handle.is_none() {
            println!("DatabaseSession has not been initialized!");
            return Err(Error::AssetNotFound("AppHandle not Initialized".into()));
        }

        let handle = self.handle.as_ref().unwrap();
        let logger = *handle.get_logger();

        if !self.start.keys().contains(&table) {
            let start_id = match db_common::get_unique_id(handle, &table) {
                Ok(id) => {
                    logger.log_debug(&format!("Found id: {} (Table: {})", &id, &table.to_str()), "GetUnclaimedId", LogVisibility::Backend);
                    id
                },
                Err(e) => {
                    logger.log_error(&format!("Unexpected database error: {}", e), "GetUnclaimedId", LogVisibility::Backend);
                    0
                },
            };
            self.start.insert(table.clone(), start_id);
        }

        if let Some(claimed_ids) = self.claimed_ids.get(&table){
            let mut unclaimed_id: u32 = 0;
            while claimed_ids.contains(&unclaimed_id) { unclaimed_id += 1; }
            Ok(unclaimed_id)
        } 
        else { 
            self.claimed_ids.insert(table.clone(), Vec::new());
            Ok(*self.start.get(&table).unwrap()) 
        }
    }

    /// Claims an id
    /// 
    /// Returns [`tauri::Error::Io`] ([`std::io:ErrorKind::AlreadyExists`]) if the id was already claimed
    /// 
    /// Returns [`tauri::Error::AssetNotFound`] if the table key does not exist
    pub fn claim_id(&mut self, id: &u32, table: DatabaseTable) -> Result<(), Error> {
        match self.claimed_ids.get_mut(&table){
            Some(ids) => match ids.contains(id) {
                    true => Err(Error::Io(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Id was already claimed!"))),
                    false => {
                        self.handle.as_ref().unwrap().logger(|lgr| lgr.log_info(&format!("Claimed id: {} (Table: {})", id, table.to_str()), "ClaimId", LogVisibility::Backend));
                        Ok(ids.push(*id))
                    },
                },
            None => {
                Err(Error::AssetNotFound(format!("ID collection for table '{}' is uninitialized!", table.to_str())))
            },
        } 
    }

    pub fn get_and_claim_id(&mut self, table: DatabaseTable) -> Result<u32, Error> {
        match self.get_unclaimed_id(table.clone()) {
            Ok(id) => {
                match self.claim_id(&id, table) {
                    Ok(_) => Ok(id),
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        } 
    }

    pub fn initialize(handle: &AppHandle) -> DatabaseSession {
        let owned_handle = handle.clone();

        DatabaseSession {
            handle: Some(owned_handle),
            start: HashMap::new(),
            claimed_ids: HashMap::new()
        }
    }
}