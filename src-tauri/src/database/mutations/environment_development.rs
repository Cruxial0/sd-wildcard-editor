use crate::database::mutations::mutation_selector::DatabaseMutation;

#[derive(Default)]
pub struct MutationEnvironmentDevelopment;

impl DatabaseMutation for MutationEnvironmentDevelopment {
    fn get_batch_command(&self) -> String {
        "
        CREATE TABLE IF NOT EXISTS Projects (
            ID INTEGER PRIMARY KEY AUTOINCREMENT, 
            Name VARCHAR(255) NOT NULL, 
            Description VARCHAR(255), 
            Wildcards TEXT, 
            Projects TEXT
        );
    
        CREATE TABLE IF NOT EXISTS Wildcards (
            ID INTEGER PRIMARY KEY AUTOINCREMENT,
            Name VARCHAR(255), 
            Path VARCHAR(255), 
            Lines TEXT
        );".to_owned()
    }
}
