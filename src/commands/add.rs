use crate::priority::Priority;
use crate::todo::TodoBuilder;
use crate::todo_list::TodoList;
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
