mod commands;
mod todo;
mod todo_list;
mod utils;
mod display;
mod sort_order;
mod sync;
mod cli;
mod priority;
//mod sync;
mod config;

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

    //config laden
    let conf = config::load_config()?;
    let data_dir = config::get_data_dir()?;
    let remote_url = conf.git_remote.as_deref(); //wenns remote repo gibt und in config eingetragen

    let git_repo = GitRepo::new(data_dir);
    //git_repo.demo_merge()?;//TODO nur zum testen vom merge
    git_repo.setup(remote_url)?;
    Ok(())
}
