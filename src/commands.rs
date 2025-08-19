pub mod add;
pub mod show;
pub mod remove;
pub mod complete;
pub mod update;
pub mod help;
pub mod sort;
mod config_command;

use crate::argument::ArgumentMeta;
use crate::commands::add::AddCommand;
use crate::commands::complete::CompleteCommand;
use crate::commands::config_command::ConfigCommand;
use crate::commands::help::HelpCommand;
use crate::commands::remove::RemoveCommand;
use crate::commands::show::ShowCommand;
use crate::commands::update::UpdateCommand;
use crate::parser::ParsedCommand;
use crate::commands::sort::SortCommand;

pub trait Command {
    fn execute(&self, parsed: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>>;

    fn arguments(&self) -> Vec<ArgumentMeta> {
        Vec::new()
    }

    fn description(&self) -> &'static str { //string literal referenz, fest im speicher, niemals temporaer
        "No description provided."
    }
}

pub(crate) fn all_commands() -> Vec<(&'static str, Box<dyn Command>)> {
    vec![
        //("add", Box::new(AddCommand)),
        ("show", Box::new(ShowCommand)),
        ("remove", Box::new(RemoveCommand)),
        ("complete", Box::new(CompleteCommand)),
        ("update", Box::new(UpdateCommand)),
        ("sort", Box::new(SortCommand)),
        ("help", Box::new(HelpCommand)),
        ("config", Box::new(ConfigCommand)),
    ]
}

