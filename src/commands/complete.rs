use crate::todo_list::TodoList;
use clap::Args;

#[derive(Debug)]
pub struct CompleteCommand;

#[derive(Args)]
pub struct CompleteArgs {
    id: usize,
}

impl CompleteCommand {
    pub fn execute(args: CompleteArgs) -> Result<(), Box<dyn std::error::Error>> {
        let mut todo_list =
            TodoList::load().map_err(|e| format!("Failed to load todo list: {}", e))?;

        let todo = todo_list
            .get_todo_mut(args.id)
            .ok_or_else(|| format!("Todo with ID {} not found", args.id))?;

        todo.complete(true);
        todo_list
            .save()
            .map_err(|e| format!("Failed to save todo list: {}", e))?;

        println!("Todo with ID {} completed", args.id);
        Ok(())
    }
}
