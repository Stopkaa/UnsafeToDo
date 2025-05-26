use crate::commands::Command;
use std::fs;
use serde_cbor::Value;

#[derive(Debug)]
pub struct ShowCommand;

impl Command for ShowCommand {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        show_todo();
        Ok(())
    }
}

pub fn show_todo() -> Result<(), Box<dyn std::error::Error>> {
    let todos_as_bytes = fs::read("todos.txt").expect("Nothing to read");
    let cbor_value: Value = serde_cbor::from_slice(&todos_as_bytes)?;

    let json_string = serde_json::to_string_pretty(&cbor_value)?;
    println!("CBOR als JSON:");
    println!("{}", json_string);
    Ok(())
}
