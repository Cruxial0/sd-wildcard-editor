use crate::database::migration_handler::DatabaseMigration;

#[derive(Default)]
pub struct MigrationMergeDefinitions;

impl DatabaseMigration for MigrationMergeDefinitions {
    fn get_batch_command(&self) -> String {
        "CREATE TABLE IF NOT EXISTS MergeDefinitions (
            id VARCHAR(128), 
            definition TEXT,
            PRIMARY KEY(id)
        );".to_owned()
    }
}