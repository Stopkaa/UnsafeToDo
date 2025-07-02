use crate::commands::Command;
use crate::parser::ParsedCommand;
use crate::todo::TodoList;
pub struct RemoveCommand;

impl Command for RemoveCommand {
    fn execute(&self, parsed: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>> {
        let mut todo_list = TodoList::load().unwrap();
        let id = parsed.positional.as_deref().ok_or("No ID provided")?;
        let id = id.parse()
            .map_err(|_| format!("Invalid ID format: '{}'", id))?;

        let removed = todo_list.remove(id)
            .ok_or_else(|| format!("Todo with ID {} not found", id))?;
        
        todo_list.save()?;
        println!("removed TODO: {:?}", removed);
        Ok(())
    }

    fn description(&self) -> &'static str {
        "Removes one todo with given index"
    }
}
