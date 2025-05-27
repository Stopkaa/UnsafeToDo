use core::panic;
use crate::{commands::Command, todo::Todo};

#[derive(Debug)]
pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(todo) = args.get(0) {
            Todo::new(todo.clone()).save_to_file()?;
        }
        else {
            panic!("Todo not specified");
        }
        Ok(())
    }
}
