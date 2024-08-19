use crate::database::migration_handler::DatabaseMigration;

pub struct MigrationTrackedFilesTable;
impl DatabaseMigration for MigrationTrackedFilesTable {
    fn get_batch_command(&self) -> String {
        "
        CREATE TABLE IF NOT EXISTS Workspace (
            id INTEGER NOT NULL UNIQUE,
            wildcards TEXT, 
            subjects TEXT,
            PRIMARY KEY(id)
        );
        CREATE TABLE IF NOT EXISTS TrackedFiles (
            id INTEGER NOT NULL UNIQUE,
            files TEXT
        );
        ".into()
    }
}