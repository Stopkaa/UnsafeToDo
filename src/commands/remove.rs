use crate::commands::Command;
use crate::parser::ParsedCommand;
use crate::todo::TodoList;
pub struct RemoveCommand;
//TODO remove mit id statt index
impl Command for RemoveCommand {
    fn execute(&self, parsed: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>> {
        let mut todo_list = TodoList::load().unwrap();
        if let Some(index_str) = &parsed.positional {
            if let Ok(index) = index_str.parse::<usize>() {
                if let Some(removed) = todo_list.remove(index - 1) {
                    todo_list.save()?;
                    println!("removed TODO: {:?}", removed);
                } else {
                    println!("Error: Wrong Index.");
                }
            } else {
                println!("Error: '{}' no valid number", index_str);
            }
        } else {
            println!("Error: No index specified.");
        }
        Ok(())
    }

    fn description(&self) -> &'static str {
        "Removes one todo with given index"
    }
}
