mod commands;
mod parser;
mod todo;
mod utils;

use parser::parse_command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match parse_command(&args) {
        Some(cmd) => {
            if let Err(e) = cmd.execute(&args[1..]) {
                eprintln!("{}", e);
            }
        }
        None => {
            eprintln!("Unknown or missing command.");
            print_help();
        }
    }
    Ok(())
}

fn print_help() {
    println!("Help:");
    println!("  unsafe_todo add \"TODO\"");
    println!("  unsafe_todo show");
    println!("  unsafe_todo remove <index>");
}
