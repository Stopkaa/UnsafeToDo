use crate::{commands::Command, todo::TodoList};
use std::str::FromStr;
use chrono::NaiveDate;
use crate::argument::{finished_argument, title_argument, ArgumentMeta};
use crate::commands::add::{description_argument, due_date_argument, priority_argument};
use crate::parser::ParsedCommand;
use crate::priority::Priority;

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
            for arg_meta in self.arguments() {
                if let Some(value) = parsed.options.get(&arg_meta.prefix) {
                    match arg_meta.name.as_str() {
                        "priority" => {
                            let priority = Priority::from_str(value).unwrap_or_else(|_| {
                                println!("Warning: Invalid priority '{}', defaulting to 'Low'", value);
                                Priority::Low
                            });
                            todo.set_priority(priority);
                        }
                        "due_date" => {
                            match NaiveDate::parse_from_str(value, "%d.%m.%Y") {
                                Ok(date) => {
                                    todo.set_due_date(date);
                                }
                                Err(err) => {
                                    println!("Warning: Invalid due date '{}': {}", value, err);
                                }
                            }
                        }
                        "title" => {
                            todo.set_title(value.clone());
                        }
                        "description" => {
                            todo.set_description(value.clone());
                        }
                        "finished" => {
                            let finished = value.parse::<bool>().unwrap_or_else( |_| {
                                println!("Warning: Invalid finished value '{}', defaulting to 'false'", value);
                                false
                            });
                            todo.set_finished(finished);
                        }
                        _ => {
                            println!("Unknown argument: {} = {}", arg_meta.name, value);
                        }
                    }
                }
            }
            todo_list.save()?;
            println!("Todo with ID: {} updated", id);
        }
        else {
            return Err(format!("Todo with ID: {} not found", id).into());
        }
        Ok(())
    }

    fn arguments(&self) -> Vec<ArgumentMeta> {
        vec![priority_argument(), due_date_argument(), description_argument(), finished_argument(), title_argument()]
    }


    fn description(&self) -> &'static str {
        "Updates one todo with given index"
    }
}

