use clap::{Parser, Subcommand};

use crate::commands::add::{AddCommand, AddArgs};

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
}

impl Cli {
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            Commands::Add(args) => AddCommand::execute(args)
        }
    }
}


