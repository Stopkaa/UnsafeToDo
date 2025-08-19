mod commands;
mod parser;
mod todo;
mod utils;
mod display;
mod argument;
mod sort_order;
mod sync;
mod cli;
mod priority;
//mod sync;
mod config;
use parser::parse_args;

use crate::sync::GitRepo;
use crate::cli::Cli;
use clap::Parser;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    if let Err(e) = cli.execute() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    return Ok(());
    let args: Vec<String> = std::env::args().skip(1).collect();

    //config laden
    let conf = config::load_config()?;
    let data_dir = config::get_data_dir()?;
    let remote_url = conf.git_remote.as_deref(); //wenns remote repo gibt und in config eingetragen

    let git_repo = GitRepo::new(data_dir);
    //git_repo.demo_merge()?;//TODO nur zum testen vom merge
    git_repo.setup(remote_url)?;
    
    if let Some(parsed) = parse_args(&args) {
        if let Some(command) = parsed.to_command() {
            command.execute(&parsed)?;
        } else {
            println!("Unknown command: {}, use help for more details", parsed.command);
        }
    } else {
        println!("No command provided, use help for more details");
    }

    Ok(())
}
