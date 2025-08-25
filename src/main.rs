mod cli;
mod commands;
mod config;
mod display;
mod priority;
mod sort_order;
mod sync;
mod todo;
mod todo_list;

use crate::cli::Cli;
use crate::sync::GitRepo;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = config::load_config()?;
    let data_dir = config::get_data_dir()?;
    let remote_url = conf.git_remote.as_deref();
    let git_repo = GitRepo::new(data_dir);
    git_repo.setup(remote_url)?;
    let cli = Cli::parse();
    if let Err(e) = cli.execute() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}
