use crate::priority::Priority;
use crate::todo_list::TodoList;
use chrono::NaiveDate;
use clap::Args;

#[derive(Debug)]
pub struct UpdateCommand;

#[derive(Args)]
pub struct UpdateArgs {
    #[arg(long)]
    id: usize,

    #[arg(short, long)]
    title: Option<String>,

    #[arg(long)]
    description: Option<String>,

    #[arg(long, short, value_enum)]
    priority: Option<Priority>,

    #[arg(long, value_parser = parse_date_string,
        help = "Date in format dd.mm.YYYY or d.m.YYYY"
    )]
    due_date: Option<NaiveDate>,

    #[arg(short, long)]
    finished: Option<bool>,
}

fn parse_date_string(date_as_str: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(date_as_str, "%d.%m.%Y")
        .map_err(|_| "Invalid date format. See help for further information".to_string())
}

impl UpdateCommand {
    pub fn execute(args: UpdateArgs) -> Result<(), Box<dyn std::error::Error>> {
        let mut todo_list = TodoList::load().unwrap();
        if let Some(todo) = todo_list.get_todo_mut(args.id) {
            if let Some(title) = args.title {
                todo.set_title(title.clone());
            }
            if let Some(priority) = args.priority {
                todo.set_priority(priority);
            }
            if let Some(due_date) = args.due_date {
                todo.set_due_date(due_date);
            }
            if let Some(description) = args.description {
                todo.set_description(description);
            }
            if let Some(finished) = args.finished {
                todo.set_finished(finished);
            }
            todo_list.save()?;
            println!("Todo with ID: {} updated", args.id);
        } else {
            return Err(format!("Todo with ID: {} not found", args.id).into());
        }
        Ok(())
    }
}
