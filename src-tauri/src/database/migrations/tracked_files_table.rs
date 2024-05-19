use crate::database::migration_handler::DatabaseMigration;

pub struct MigrationTrackedFilesTable;
impl DatabaseMigration for MigrationTrackedFilesTable {
    fn get_batch_command(&self) -> String {
        "
        CREATE TABLE IF NOT EXISTS Workspace (
            id INTEGER NOT NULL UNIQUE,
            wildcards TEXT, 
            projects TEXT,
            PRIMARY KEY(id)
        );
        CREATE TABLE IF NOT EXISTS TrackedFiles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT,
            type INTEGER,
            content TEXT
        );
        ".into()
    }
}