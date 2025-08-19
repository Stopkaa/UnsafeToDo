pub use crate::argument::{description_argument, due_date_argument, priority_argument, ArgumentMeta};
use crate::parser::ParsedCommand;
use crate::priority::Priority;
use crate::todo::{TodoBuilder, TodoList};
use crate::commands::Command;
use std::env::args;
use std::str::FromStr;
use chrono::NaiveDate;
use clap::Args;

#[derive(Debug)]
pub struct AddCommand {}

#[derive(Args)]
pub struct AddArgs {
    #[arg(short, long)]
    title: String,

    #[arg(long)]
    description: Option<String>,

    #[arg(long, short, value_enum)]
    priority: Option<Priority>,

    #[arg(long, value_parser = parse_date_string,
        help = "Date in format dd.mm.YYYY or d.m.YYYY"
    )]
    due_date: Option<NaiveDate>,
}

impl Command for AddCommand {
    fn execute(&self, parsed: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>> {
        let title = parsed.positional
            .as_ref()
            .ok_or("Todo title not specified")?;

        let mut todo_list = TodoList::load().unwrap();
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
        todo_list.add(todo);
        todo_list.save()?;

        Ok(())
    }


    fn arguments(&self) -> Vec<ArgumentMeta> {
        vec![priority_argument(), due_date_argument(), description_argument()]
    }

    fn description(&self) -> &'static str {
        "Adds a new todo item with optional priority, due date, and description."
    }
}

impl AddCommand {
    pub fn execute(args: AddArgs) -> Result<(), Box<dyn std::error::Error>> {
        let mut todo_list = TodoList::load().unwrap();
        let todo = TodoBuilder::new()
            .title(args.title.clone())
            .due_date(args.due_date)
            .description(args.description)
            .priority(args.priority)
            .build()?;
        todo_list.add(todo);
        todo_list.save()?;
        Ok(())
    }
}

fn parse_date_string(date_as_str: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(date_as_str, "%d.%m.%Y")
        .map_err(|_| "Invalid date format. See help for further information".to_string())
}
