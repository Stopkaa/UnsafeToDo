use crate::commands::Command;
use crate::display::display_todo_list;
use crate::todo::TodoList;
use dirs::data_local_dir;
use std::fs;
use std::path::PathBuf;
use crate::parser::ParsedCommand;

#[derive(Debug)]
pub struct ShowCommand;

impl Command for ShowCommand {
    fn execute(&self, parsed: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>> {
        //show_todo()
        show_todo_pretty()
    }

    fn description(&self) -> &'static str {
        "Shows all todos in the list"
    }
}

pub fn show_todo() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("unsafe_todo");
    fs::create_dir_all(&path).ok();
    path.push("todos.txt");

    let content = fs::read_to_string(path)?;
    println!("{}", content);
    Ok(())
}

pub fn show_todo_pretty() -> Result<(), Box<dyn std::error::Error>> {
    let todos = TodoList::load()?;
    display_todo_list(&todos);
    Ok(())
}
