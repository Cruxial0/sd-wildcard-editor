use rusqlite::Transaction;
use std::collections::HashMap;

use crate::logging::logger::{LogVisibility, Logger};

use super::migrations::environment_development::MigrationEnvironmentDevelopment;
use super::migrations::mutation_behaviour_settings::MigrationBehaviourSettings;
use super::migrations::tracked_files_table::MigrationTrackedFilesTable;
use crate::database::migrations::merge_definitions::MigrationMergeDefinitions;

static LOG_SOURCE: &str = "DatabaseMigration";

lazy_static! {
    static ref MIGRATIONS: HashMap<u32, Box<dyn DatabaseMigration + Sync>> = {
        let mut m: HashMap<u32, Box<dyn DatabaseMigration + Sync>> = HashMap::new();

        m.insert(1, to_migration(MigrationEnvironmentDevelopment));
        m.insert(2, to_migration(MigrationBehaviourSettings));
        m.insert(3, to_migration(MigrationTrackedFilesTable));
        m.insert(4, to_migration(MigrationMergeDefinitions));
        m
    };
}

fn to_migration<'a>(
    item: impl DatabaseMigration + Sync + 'a,
) -> Box<dyn DatabaseMigration + Sync + 'a> {
    Box::new(item)
}

/// Applies all mutations from the current version and down, starting from the first and ending at the latest.
pub fn apply_migrations(tx: &mut Transaction, version: u32, logger: &Logger) {
    let mut command = "".to_owned();

    for i in 1..version + 1 {
        let mutation = MIGRATIONS.get(&i).expect("Database Migration should exist");
        command.push_str(&format!("{}{}", "\n", mutation.get_batch_command()));
    }

    match tx.execute_batch(&command) {
        Ok(_) => logger.log_info(
            "Successfully applied database migrations",
            LOG_SOURCE,
            LogVisibility::Backend,
        ),
        Err(x) => logger.log_error(
            &format!(
                "An error occured while applying database migrations: {:?}",
                x
            ),
            LOG_SOURCE,
            LogVisibility::Backend,
        ),
    }
}

pub trait DatabaseMigration {
    /// Use this to declare the setup command
    fn get_batch_command(&self) -> String;
}
