pub(crate) use crate::argument::{ArgumentMeta};
use crate::parser::ParsedCommand;
use crate::commands::Command;
use crate::argument::{config_auto_sync_argument, config_git_remote_argument, config_show_argument};
use crate::config;

#[derive(Debug)]
pub struct ConfigCommand;

impl Command for ConfigCommand {
    fn execute(&self, parsed: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>> {
        for arg_meta in self.arguments() {
            if let Some(value) = parsed.options.get(&arg_meta.prefix) {
                match arg_meta.name.as_str() {
                    "auto_sync" => {
                        let auto_sync = value.parse::<bool>().unwrap_or_else( |_| {
                            println!("Warning: Invalid auto_sync value '{}', defaulting to 'false'", value);
                            false
                        });
                        config::set_auto_sync(auto_sync)?;
                    }
                    "git_remote_path" => {
                        config::set_git_remote(value.clone())?;
                    }
                    "config_show" => {
                        config::show_config()?;
                    }

                    _ => {
                        println!("Unknown argument: {} = {}", arg_meta.name, value);
                    }
                }
            }

        }
        Ok(())
    }

    fn arguments(&self) -> Vec<ArgumentMeta> {
        vec![config_git_remote_argument(), config_auto_sync_argument(), config_show_argument()]
    }

    fn description(&self) -> &'static str {
        "Change configuration"
    }
}
