use crate::commands::Command;
use std::fs;
use dirs::data_local_dir;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ShowCommand;

impl Command for ShowCommand {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        show_todo()
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
