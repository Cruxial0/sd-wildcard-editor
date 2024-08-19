use crate::database::migration_handler::DatabaseMigration;

#[derive(Default)]
pub struct MigrationEnvironmentDevelopment;

impl DatabaseMigration for MigrationEnvironmentDevelopment {
    fn get_batch_command(&self) -> String {
        "
        CREATE TABLE IF NOT EXISTS Subjects (
            id VARCHAR(128), 
            name VARCHAR(255) NOT NULL, 
            description VARCHAR(255), 
            wildcards TEXT, 
            subjects TEXT,
            PRIMARY KEY(id)
        );
    
        CREATE TABLE IF NOT EXISTS Wildcards (
            id VARCHAR(128),
            name VARCHAR(255), 
            path VARCHAR(255), 
            lines TEXT,
            PRIMARY KEY(id)
        );"
        .to_owned()
    }
}
