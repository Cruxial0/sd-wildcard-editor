use crate::database::migration_handler::DatabaseMigration;

pub struct MigrationTrackedFilesTable;
impl DatabaseMigration for MigrationTrackedFilesTable {
    fn get_batch_command(&self) -> String {
        "
        CREATE TABLE IF NOT EXISTS TrackedFiles (
            path TEXT,
            type INTEGER,
            content TEXT
        );
        ".into()
    }
}