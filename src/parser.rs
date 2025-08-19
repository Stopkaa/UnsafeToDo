use crate::commands::{all_commands, Command};
use crate::commands::{add::AddCommand, show::ShowCommand, remove::RemoveCommand, complete::CompleteCommand};


impl ParsedCommand {
    pub fn to_command(&self) -> Option<Box<dyn Command>> {
        all_commands()
            .into_iter()
            .find(|(name, _)| name == &self.command)
            .map(|(_, cmd)| cmd)
    }
}

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedCommand {
    pub command: String,
    pub positional: Option<String>,
    pub options: HashMap<String, String>,
}

pub fn parse_args(args: &[String]) -> Option<ParsedCommand> {
    let mut iter = args.iter();

    let command = iter.next()?.to_string();

    let mut positional = None;
    let mut options = HashMap::new();

    while let Some(arg) = iter.next() {
        if arg.starts_with('-') && arg.len() > 1 {
            let key = arg.trim_start_matches('-').to_string();
            if let Some(value) = iter.next() {
                options.insert(key, value.to_string());
            } else {
                options.insert(key, "".to_string());
            }
        } else if positional.is_none() {
            positional = Some(arg.to_string());
        } else {
            // Weitere freie Argumente ignorieren
        }
    }

    Some(ParsedCommand {
        command,
        positional,
        options,
    })
}
