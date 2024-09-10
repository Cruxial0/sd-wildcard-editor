use crate::database::migration_handler::DatabaseMigration;

pub struct MigrationBehaviourSettings;

impl DatabaseMigration for MigrationBehaviourSettings {
    fn get_batch_command(&self) -> String {
        "
        CREATE TABLE IF NOT EXISTS AppSettings (
            uuid VARCHAR(64) NOT NULL UNIQUE,
            selectedStyle INTEGER,
            PRIMARY KEY(uuid)
        );"
        .to_owned()
    }
}
