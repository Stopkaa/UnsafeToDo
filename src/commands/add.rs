use crate::argument::ArgumentMeta;
use crate::parser::ParsedCommand;
use crate::priority::Priority;
use crate::todo::TodoBuilder;
use crate::commands::Command;
use std::str::FromStr;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug)]
pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, parsed: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>> {
        let title = parsed.positional
            .as_ref()
            .ok_or_else(|| "Todo title not specified")?;

        let mut builder = TodoBuilder::new().title(title.clone());

        for arg_meta in self.arguments() {
            if let Some(value) = parsed.options.get(&arg_meta.prefix) {
                match arg_meta.name.as_str() {
                    "priority" => {
                        let priority = Priority::from_str(value).unwrap_or_else(|_| {
                            println!("Warning: Invalid priority '{}', defaulting to 'Low'", value);
                            Priority::Low
                        });
                        builder = builder.priority(priority);
                    }
                    "due_date" => {
                        match NaiveDate::parse_from_str(value, "%d.%m.%Y") {
                            Ok(date) => {
                                builder = builder.due_date(date);
                            }
                            Err(err) => {
                                println!("Warning: Invalid due date '{}': {}", value, err);
                            }
                        }
                    }
                    "description" => {
                        builder = builder.description(value.clone());
                    }
                    _ => {
                        println!("Unknown argument: {} = {}", arg_meta.name, value);
                    }
                }
            }

        }

        let todo = builder.build()?; // ? gibt fehler direkt zurueck
        todo.save_to_file()?;

        Ok(())
    }

    fn arguments(&self) -> Vec<ArgumentMeta> {
        vec![priority_argument(), due_date_argument(), description_argument()]
    }

    fn description(&self) -> &'static str {
        "Adds a new todo item with optional priority, due date, and description."
    }
}

pub fn priority_argument() -> ArgumentMeta {
    ArgumentMeta {
        name: "priority".to_string(),
        prefix: "p".to_string(),
        help: "Optional priority, e.g. -p high".to_string(),
    }
}

pub fn due_date_argument() -> ArgumentMeta {
    ArgumentMeta {
        name: "due_date".to_string(),
        prefix: "d".to_string(),
        help: "Optional due date, e.g. -d DD.MM.YYYY".to_string(),
    }
}

pub fn description_argument() -> ArgumentMeta {
    ArgumentMeta {
        name: "description".to_string(),
        prefix: "m".to_string(),
        help: "m for memo, optional description, e.g. -m \"Task details\"".to_string(),
    }
}


