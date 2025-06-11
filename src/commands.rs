use crate::argument::ArgumentMeta;
use crate::parser::ParsedCommand;

pub mod add;
pub mod show;
pub mod remove;
pub mod complete;
pub mod update;

pub trait Command {
    fn execute(&self, parsed: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>>;

    fn arguments(&self) -> Vec<ArgumentMeta> {
        Vec::new()
    }
}

