use clap::{Parser, Subcommand};

use crate::commands::{
    add::{AddArgs, AddCommand},
    complete::{CompleteArgs, CompleteCommand},
    remove::{RemoveArgs, RemoveCommand},
    show::{ShowArgs, ShowCommand},
    sort::{SortArgs, SortCommand},
    update::{UpdateArgs, UpdateCommand},
};

#[derive(Parser)]
#[command(name = "utodo")]
#[command(version = "0.1")]
#[command(about = "CLI Todo Tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(AddArgs),
    Remove(RemoveArgs),
    Complete(CompleteArgs),
    Show(ShowArgs),
    Sort(SortArgs),
    Update(UpdateArgs),
}

impl Cli {
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            Commands::Add(args) => AddCommand::execute(args),
            Commands::Remove(args) => RemoveCommand::execute(args),
            Commands::Complete(args) => CompleteCommand::execute(args),
            Commands::Show(args) => ShowCommand::execute(args),
            Commands::Sort(args) => SortCommand::execute(args),
            Commands::Update(args) => UpdateCommand::execute(args),
        }
    }
}
