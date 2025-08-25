use crate::display::display_todo_list;
use crate::todo_list::TodoList;
use clap::Args;

#[derive(Debug)]
pub struct ShowCommand;

#[derive(Args)]
pub struct ShowArgs {}

impl ShowCommand {
    pub fn execute(_: ShowArgs) -> Result<(), Box<dyn std::error::Error>> {
        let todos = TodoList::load()?;
        display_todo_list(&todos);
        Ok(())
    }
}
