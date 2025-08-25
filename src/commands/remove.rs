use crate::todo_list::TodoList;
use clap::Args;

#[derive(Debug)]
pub struct RemoveCommand;

#[derive(Args)]
pub struct RemoveArgs {
    id: usize,
}

impl RemoveCommand {
    pub fn execute(args: RemoveArgs) -> Result<(), Box<dyn std::error::Error>> {
        let mut todo_list = TodoList::load().unwrap();
        let removed = todo_list
            .remove(args.id)
            .ok_or_else(|| format!("Todo with ID {} not found", args.id))?;

        todo_list.save()?;
        println!("removed TODO: {:?}", removed);
        Ok(())
    }
}
