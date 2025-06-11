use crate::argument::ArgumentMeta;
use crate::parser::ParsedCommand;
use crate::priority::Priority;
use crate::todo::TodoBuilder;
use crate::commands::Command;
use std::str::FromStr;

#[derive(Debug)]
pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self, parsed: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>> {
        let title = parsed.positional
            .as_ref()
            .ok_or_else(|| "Todo title not specified")?;

        let mut builder = TodoBuilder::new().title(title.clone());

        for arg_meta in self.arguments() {
            if let Some(value) = parsed.options.get(&arg_meta.prefix) {
                match arg_meta.name.as_str() {
                    "priority" => {
                        let priority = Priority::from_str(value).unwrap_or_else(|_| {
                            println!("Warnung: Ungültige Priorität '{}', Standard 'Low' verwendet", value);
                            Priority::Low
                        });
                        builder = builder.priority(priority);
                    }
                    _ => {
                        println!("Unbekanntes Argument: {} = {}", arg_meta.name, value);
                    }
                }
            }
        }

        let todo = builder.build()?; // ? gibt fehler direkt zurueck
        todo.save_to_file()?;

        Ok(())
    }

    fn arguments(&self) -> Vec<ArgumentMeta> {
        vec![priority_argument()]
    }
}

pub fn priority_argument() -> ArgumentMeta {
    ArgumentMeta {
        name: "priority".to_string(),
        prefix: "p".to_string(),
        help: "Optional priority, e.g. -p high".to_string(),
    }
}


