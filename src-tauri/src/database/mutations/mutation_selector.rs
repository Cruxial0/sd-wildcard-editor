use crate::database::mutations::test_mutation::TestMutation;
use rusqlite::Transaction;
use std::collections::HashMap;
use crate::logging::logger;

use super::environment_development::MutationEnvironmentDevelopment;

static LOG_SOURCE: &str = "DatabaseMigration";

lazy_static! {
    static ref MUTATIONS: HashMap<u32, Box<dyn DatabaseMutation + Sync>> = {
        let mut m: HashMap<u32, Box<dyn DatabaseMutation + Sync>> = HashMap::new();
        let v1 = Box::new(MutationEnvironmentDevelopment);
        let v2 = Box::new(TestMutation);

        let mut1: Box<dyn DatabaseMutation + Sync> = Box::new(*v1);
        let mut2: Box<dyn DatabaseMutation + Sync> = Box::new(*v2);

        m.insert(1, mut1);
        m.insert(2, mut2);
        m
    };
}

/// Applies all mutations from the current version and down, starting from the first and ending at the latest.
pub fn apply_mutations(tx: &mut Transaction, version: u32) {
    let mut command = "".to_owned();

    for i in 1..version + 1 {
        let mutation = MUTATIONS.get(&i).expect("Database Mutation should exist");
        command.push_str(&format!("{}{}", "\n", mutation.get_batch_command()));
    }

    match tx.execute_batch(&command) {
        Ok(_) => logger::log("Successfully applied database migrations", LOG_SOURCE, logger::LogVisibility::Backend),
        Err(x) => logger::log(&format!("An error occured while applying database migrations: {:?}", x), LOG_SOURCE, logger::LogVisibility::Backend)
    }
}

pub trait DatabaseMutation {
    /// Use this to declare the setup command
    fn get_batch_command(&self) -> String;
}
