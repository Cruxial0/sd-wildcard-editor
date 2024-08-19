use crate::database::migration_handler::DatabaseMigration;

pub struct MigrationBehaviourSettings;

impl DatabaseMigration for MigrationBehaviourSettings {
    fn get_batch_command(&self) -> String {
        "
        CREATE TABLE IF NOT EXISTS AppSettings (
            id VARCHAR(128) NOT NULL UNIQUE,
            trackedDirectories TEXT,
            selectedStyle INTEGER,
            PRIMARY KEY(id)
        );
        "
        .to_owned()
    }
}
