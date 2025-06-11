use serde::de::value;

use crate::{commands::Command, todo::TodoList};
use std::collections::HashMap;
use std::error::Error;
#[derive(Debug)]
pub struct UpdateCommand;

impl Command for UpdateCommand {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if args.is_empty() {
            return Err("No arguments proposed".into());
        }

        let id = args[0].parse::<i32>()?;
        let mut todo_list = TodoList::load().unwrap();
        if let Some(todo ) = todo_list.get_todo_mut(id as usize) {
            for arg in args.iter().skip(1) {
            }
            todo.complete(true);
            todo_list.save()?;
            println!("Todo with ID: {} finished", id);
        }
        else {
            return Err(format!("Todo with ID: {} not found", id).into());
        }
        Ok(())
    }
}

pub struct ArgParser;

impl ArgParser {
    pub fn parse_args_to_map(&self, args: &[String]) -> Result<HashMap<String, String>, Box<dyn Error>> {
        Ok(HashMap::new()) //TODO
    }   
    
}
