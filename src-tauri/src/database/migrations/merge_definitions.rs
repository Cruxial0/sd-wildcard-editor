use crate::database::migration_handler::DatabaseMigration;

#[derive(Default)]
pub struct MigrationMergeDefinitions;

impl DatabaseMigration for MigrationMergeDefinitions {
    fn get_batch_command(&self) -> String {
        "CREATE TABLE IF NOT EXISTS MergeDefinitions (
            uuid VARCHAR(64),
            name VARCHAR(64),
            definition TEXT,
            PRIMARY KEY(uuid)
        );".to_owned()
    }
}