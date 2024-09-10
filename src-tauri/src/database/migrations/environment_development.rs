use crate::database::migration_handler::DatabaseMigration;

#[derive(Default)]
pub struct MigrationEnvironmentDevelopment;

impl DatabaseMigration for MigrationEnvironmentDevelopment {
    fn get_batch_command(&self) -> String {
        "
        CREATE TABLE IF NOT EXISTS Subjects (
            uuid VARCHAR(64),
            name NVARCHAR(255) NOT NULL,
            path TEXT,
            description TEXT,
            wildcards TEXT,
            subjects TEXT,
            mergeDefs TEXT,
            PRIMARY KEY(uuid)
        );
    
        CREATE TABLE IF NOT EXISTS Wildcards (
            uuid VARCHAR(64),
            name NVARCHAR(255),
            path TEXT,
            PRIMARY KEY(uuid)
        );"
        .to_owned()
    }
}
