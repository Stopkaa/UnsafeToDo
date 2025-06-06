use crate::commands::Command;
use crate::commands::{add::AddCommand, show::ShowCommand, remove::RemoveCommand, complete::CompleteCommand};

pub fn parse_command(args: &[String]) -> Option<Box<dyn Command>> {
    match args.get(0).map(|s| s.as_str()) {
        Some("add") => Some(Box::new(AddCommand)),
        Some("show") => Some(Box::new(ShowCommand)),
        Some("remove") => Some(Box::new(RemoveCommand)),
        Some("complete") => Some(Box::new(CompleteCommand)),
        _ => None,
    }
}
