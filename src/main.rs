mod commands;
mod parser;
mod todo;
mod utils;
mod display;
mod argument;

mod priority;
//mod sync;
mod config;
use parser::parse_args;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    config::load_config();
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

fn print_help() {
    println!("Help:");
    println!("  unsafe_todo add \"TODO\"");
    println!("  unsafe_todo show");
    println!("  unsafe_todo remove <index>");
}
