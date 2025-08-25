use crate::config;
use crate::sort_order::SortCriteria;
use crate::todo_list::TodoList;
use clap::Args;

pub struct SortCommand;

#[derive(Args)]
pub struct SortArgs {
    #[arg(value_enum)]
    criterias: Vec<SortCriteria>,
}

impl SortCommand {
    pub fn execute(args: SortArgs) -> Result<(), Box<dyn std::error::Error>> {
        let mut todo_list = TodoList::load()?;
        todo_list.sort_by_order(&args.criterias);
        config::set_sort_order(args.criterias)?;
        todo_list.save()?;
        Ok(())
    }
}
