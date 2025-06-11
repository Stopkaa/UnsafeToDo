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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_with_positional_and_options() {
        let args = vec![
            "add".to_string(),
            "aufgabe1".to_string(),
            "-p".to_string(),
            "high".to_string(),
            "-d".to_string(),
            "05-05-2026".to_string(),
        ];

        let expected = ParsedCommand {
            command: "add".to_string(),
            positional: Some("aufgabe1".to_string()),
            options: {
                let mut map = HashMap::new();
                map.insert("p".to_string(), "high".to_string());
                map.insert("d".to_string(), "05-05-2026".to_string());
                map
            },
        };

        let result = parse_args(&args).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_args_without_positional() {
        let args = vec![
            "show".to_string(),
            "-p".to_string(),
            "medium".to_string(),
        ];

        let expected = ParsedCommand {
            command: "show".to_string(),
            positional: None,
            options: {
                let mut map = HashMap::new();
                map.insert("p".to_string(), "medium".to_string());
                map
            },
        };

        let result = parse_args(&args).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_args_only_command() {
        let args = vec!["list".to_string()];

        let expected = ParsedCommand {
            command: "list".to_string(),
            positional: None,
            options: HashMap::new(),
        };

        let result = parse_args(&args).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_args_no_command() {
        let args: Vec<String> = vec![];

        let result = parse_args(&args);
        assert!(result.is_none());
    }
}
