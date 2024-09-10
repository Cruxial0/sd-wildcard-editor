use crate::database::migration_handler::DatabaseMigration;

pub struct MigrationTrackedFilesTable;
impl DatabaseMigration for MigrationTrackedFilesTable {
    fn get_batch_command(&self) -> String {
        "
        CREATE TABLE IF NOT EXISTS Workspace (
            uuid VARCHAR(64) NOT NULL UNIQUE,
            wildcards TEXT,
            subjects TEXT,
            PRIMARY KEY(uuid)
        );
        CREATE TABLE IF NOT EXISTS TrackedFiles (
            uuid VARCHAR(64) NOT NULL UNIQUE,
            data VARCHAR(256),
            path TEXT
        );
        ".into()
    }
}