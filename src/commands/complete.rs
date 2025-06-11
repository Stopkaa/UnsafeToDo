use crate::{commands::Command, todo::TodoList};
use crate::parser::ParsedCommand;

#[derive(Debug)]
pub struct CompleteCommand;

impl Command for CompleteCommand {
    fn execute(&self, parsed: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>> {
        let id_str = parsed.positional
            .as_ref()
            .ok_or_else(|| "No Index proposed")?;

        let id = id_str.parse::<usize>()?;
        let mut todo_list = TodoList::load().unwrap();
        if let Some(todo ) = todo_list.get_todo_mut(id) {
            todo.complete(true);
            todo_list.save()?;
            println!("Todo with ID: {} finished", id);
        }
        else {
            return Err(format!("Todo with ID: {} not found", id).into());
        }
        Ok(())
    }

    fn description(&self) -> &'static str {
        "Marks a todo as completed"
    }
}
