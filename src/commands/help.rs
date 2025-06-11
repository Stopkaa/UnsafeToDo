use crate::commands::{all_commands, Command};
use crate::parser::ParsedCommand;

pub struct HelpCommand;

impl HelpCommand {
    pub fn new() -> Self {
        HelpCommand
    }
}
impl Command for HelpCommand {
    fn execute(&self, _parsed: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>> {
        println!("Available commands:");
        for (name, cmd) in all_commands() {
            println!("  {}: {}", name, cmd.description());
            for arg in cmd.arguments() {
                println!("    -{}: {}", arg.prefix, arg.help);
            }
        }
        Ok(())
    }

    fn description(&self) -> &'static str {
        "Show help about all commands"
    }
}