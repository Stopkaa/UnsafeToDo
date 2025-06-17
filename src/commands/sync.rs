use crate::commands::Command;
use crate::config;
use crate::git_sync;
use std::error::Error;
pub struct SyncCommand;

impl Command for SyncCommand {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if args.is_empty() {
            // sync ohne Argumente: Einfach synchronisieren
            self.sync()
        } else if args.len() == 2 && args[0] == "--auto" {
            // sync --auto on/off
            match args[1].as_str() {
                "on" => self.enable_auto_sync(),
                "off" => self.disable_auto_sync(),
                _ => Err("Usage: sync --auto on|off".into()),
            }
        } else {
            Err("Usage: sync  or  sync --auto on|off".into())
        }
    }
}
impl SyncCommand {
    /// Einfache Synchronisation: Pull + Push
    fn sync(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Synchronizing todos...");

        // Get data directory from config module
        let data_dir = config::get_data_dir()?;

        // Check if it's a git repository using git module
        if !git_sync::is_git_repository(&data_dir) {
            return Err(format!(
                "Data directory is not a git repository: {}\nPlease initialize git first: cd {} && git init",
                data_dir.display(),
                data_dir.display()
            ).into());
        }

        // Perform sync using git module
        git_sync::sync_repository(&data_dir)?;

        println!("✅ Sync completed successfully!");
        Ok(())
    }

    /// Auto-sync aktivieren
    fn enable_auto_sync(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Use config module to enable auto-sync (includes validation)
        config::enable_auto_sync()?;

        println!("📝 All todo changes will be automatically synchronized");
        Ok(())
    }

    /// Auto-sync deaktivieren
    fn disable_auto_sync(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Use config module to disable auto-sync
        config::disable_auto_sync()?;

        println!("📝 Use 'sync' command to manually synchronize");
        Ok(())
    }

    /// Show sync status (combines config and git info)
    pub fn show_status(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Sync Status:");

        // Get config info
        let config = config::load_config()?;
        println!("   Data directory: {}", config.data_path.display());
        println!("   Todos file: {}", config.get_todos_file_path().display());
        println!(
            "   Auto-sync: {}",
            if config.auto_sync_enabled {
                "✅ Enabled"
            } else {
                "❌ Disabled"
            }
        );

        // Check git status using git module
        if git_sync::is_git_repository(&config.data_path) {
            println!("   Git repository: ✅ Found");
        } else {
            println!("   Git repository: ❌ Not found");
            if config.auto_sync_enabled {
                println!("   ⚠️  Auto-sync is enabled but no git repository found!");
            }
        }

        Ok(())
    }
}
/// Public function for auto-sync (used by other todo commands)
pub fn auto_sync_if_enabled() -> Result<(), Box<dyn Error>> {
    config::auto_sync_if_enabled()
}

/// Public function to check if auto-sync is enabled
pub fn is_auto_sync_enabled() -> bool {
    config::is_auto_sync_enabled()
}
