use serde::de::value;

use crate::{commands::Command, todo::TodoList};
use std::collections::HashMap;
use std::error::Error;
use crate::parser::ParsedCommand;

#[derive(Debug)]
pub struct UpdateCommand;

impl Command for UpdateCommand {
    fn execute(&self, parsed: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>> {
        let id_str = parsed.positional
            .as_ref()
            .ok_or_else(|| "No Index proposed")?;

        let id = id_str.parse::<i32>()?;
        let mut todo_list = TodoList::load().unwrap();
        if let Some(todo ) = todo_list.get_todo_mut(id as usize) {
            //for arg in args.iter().skip(1) {
            //}
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
        "Updates one todo with given index"
    }
}

pub struct ArgParser;

impl ArgParser {
    pub fn parse_args_to_map(&self, args: &[String]) -> Result<HashMap<String, String>, Box<dyn Error>> {
        Ok(HashMap::new()) //TODO
    }   
    
}
