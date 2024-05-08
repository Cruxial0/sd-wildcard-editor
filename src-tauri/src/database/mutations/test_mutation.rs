use super::mutation_selector::DatabaseMutation;

#[derive(Default)]
pub struct TestMutation;

impl DatabaseMutation for TestMutation {
    fn get_batch_command(&self) -> String {
        "ALTER TABLE Projects ADD test INTEGER".to_owned()
    }
}
