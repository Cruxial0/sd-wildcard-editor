use crate::database::migration_handler::DatabaseMigration;

pub struct MutationBehaviourSettings;

impl DatabaseMigration for MutationBehaviourSettings {
    fn get_batch_command(&self) -> String {
        "
        CREATE TABLE IF NOT EXISTS AppSettings (
            id INTEGER NOT NULL UNIQUE,
            trackedDirectories TEXT,
            selectedStyle INTEGER,
            PRIMARY KEY(id)
        );
        "
        .to_owned()
    }
}
