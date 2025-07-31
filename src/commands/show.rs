use crate::commands::Command;
use crate::display::{display_todo_vector};
use crate::todo::TodoList;
use crate::parser::ParsedCommand;

#[derive(Debug)]
pub struct ShowCommand;

impl Command for ShowCommand {
    fn execute(&self, _: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>> {
        show_todo_pretty()
    }

    fn description(&self) -> &'static str {
        "Shows all todos in the list"
    }
}

pub fn show_todo_pretty() -> Result<(), Box<dyn std::error::Error>> {
    let todos = TodoList::load()?;
    display_todo_vector(&todos.todos_as_vec());
    Ok(())
}
