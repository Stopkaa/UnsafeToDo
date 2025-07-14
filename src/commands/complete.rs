use crate::parser::ParsedCommand;
use crate::{commands::Command, todo::TodoList};

#[derive(Debug)]
pub struct CompleteCommand;

impl Command for CompleteCommand {
    fn execute(&self, parsed: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>> {
        let id = parsed.positional
            .as_deref()
            .ok_or("No ID provided")?;
        
        let id: usize = id.parse()
            .map_err(|_| format!("Invalid ID format: '{}'", id))?;
        
        let mut todo_list = TodoList::load()
            .map_err(|e| format!("Failed to load todo list: {}", e))?;
        
        let todo = todo_list.get_todo_mut(id)
            .ok_or_else(|| format!("Todo with ID {} not found", id))?;
        
        todo.complete(true);
        todo_list.save()
            .map_err(|e| format!("Failed to save todo list: {}", e))?;
        
        println!("Todo with ID {} completed", id);
        Ok(())
    }
    
    fn description(&self) -> &'static str {
        "Marks a todo as completed"
    }
}
