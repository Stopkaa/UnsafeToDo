use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use crate::sort_order::SortOrder;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Path where todos.json is stored
    data_path: PathBuf,
    /// Whether auto-sync is enabled
    auto_sync_enabled: bool,
    // Sort order
    sort_order: SortOrder,
}

impl Config {
    /// Get the path to the config file
    fn config_file_path() -> Result<PathBuf, Box<dyn Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?;
        
        let app_config_dir = config_dir.join("unsafeToDo");
        
        // Ensure config directory exists
        fs::create_dir_all(&app_config_dir)?;
        
        Ok(app_config_dir.join("config.json"))
    }
    
    /// Get default data directory (XDG standard)
    fn default_data_path() -> Result<PathBuf, Box<dyn Error>> {
        let data_dir = dirs::data_local_dir()
            .ok_or("Could not find local data directory")?;
        
        let app_data_dir = data_dir.join("unsafeToDo");
        
        // Ensure data directory exists
        fs::create_dir_all(&app_data_dir)?;
        
        Ok(app_data_dir)
    }
    
     fn default() -> Result<Self, Box<dyn Error>> {
        Ok(Config {
            data_path: Self::default_data_path()?,
            auto_sync_enabled: false,
            sort_order: SortOrder::default(),
        })
    }
    
    /// Load config from file, create with defaults if doesn't exist
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config_path = Self::config_file_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&content)?;
            
            // Ensure data directory exists
            fs::create_dir_all(&config.data_path)?;
            
            Ok(config)
        } else {
            // Create default config and save it
            let default_config = Self::default()?;
            default_config.save()?;
            
            println!("📄 Created default config at: {}", config_path.display());
            println!("📁 Default data directory: {}", default_config.data_path.display());
            
            Ok(default_config)
        }
    }
    
    /// Save config to file
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let config_path = Self::config_file_path()?;
        
        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Ensure data directory exists
        fs::create_dir_all(&self.data_path)?;
        
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, json)?;
        Ok(())
    }
    
    /// Get the full path to todos.json
    pub fn get_todos_file_path(&self) -> PathBuf {
        self.data_path.join("todos.json")
    }
    
    /// Set data path and save config
    pub fn set_data_path(&mut self, new_path: PathBuf) -> Result<(), Box<dyn Error>> {
        // Ensure the new directory exists
        fs::create_dir_all(&new_path)?;
        
        self.data_path = new_path;
        self.save()?;
        Ok(())
    }
    
    /// Set auto-sync enabled/disabled and save config
    pub fn set_auto_sync(&mut self, enabled: bool) -> Result<(), Box<dyn Error>> {
        self.auto_sync_enabled = enabled;
        self.save()?;
        Ok(())
    }
    
    /// Move todos from old path to new path
    pub fn migrate_todos(&self, old_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let old_todos_file = old_path.join("todos.json");
        let new_todos_file = self.get_todos_file_path();
        
        if old_todos_file.exists() && !new_todos_file.exists() {
            fs::copy(&old_todos_file, &new_todos_file)?;
            println!("📋 Migrated todos from {} to {}", 
                old_todos_file.display(), new_todos_file.display());
            
            // Optional: remove old file
            // fs::remove_file(&old_todos_file)?;
        }
        
        Ok(())
    }

    /// Get current sort order
    pub fn get_sort_order(&self) -> &SortOrder {
        &self.sort_order
    }

    /// Set sort order and save config
    pub fn set_sort_order(&mut self, sort_order: SortOrder) -> Result<(), Box<dyn Error>> {
        self.sort_order = sort_order;
        self.save()?;
        Ok(())
    }
    
    /// Show current configuration
    pub fn show(&self) -> Result<(), Box<dyn Error>> {
        println!("📊 Configuration:");
        println!("   Config file: {}", Self::config_file_path()?.display());
        println!("   Data path: {}", self.data_path.display());
        println!("   Todos file: {}", self.get_todos_file_path().display());
        println!("   Auto-sync: {}", if self.auto_sync_enabled { "✅ Enabled" } else { "❌ Disabled" });
        
                // Show if todos file exists
        let todos_file = self.get_todos_file_path();
        if todos_file.exists() {
            println!("   Todos file: ✅ Exists");
        } else {
            println!("   Todos file: ❌ Not found (will be created)");
        }
        
        Ok(())
    }
    
    /// Validate configuration (basic checks only)
    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        // Check if data path exists
        if !self.data_path.exists() {
            return Err(format!("Data path does not exist: {}", self.data_path.display()).into());
        }
        
        // Check if data path is writable
        let test_file = self.data_path.join(".write_test");
        match fs::write(&test_file, "test") {
            Ok(_) => {
                fs::remove_file(&test_file).ok(); // Clean up
            }
            Err(_) => {
                return Err(format!("Data path is not writable: {}", self.data_path.display()).into());
            }
        }
        
        Ok(())
    }
    
    /// Get config file location for display purposes
    pub fn get_config_file_path() -> Result<PathBuf, Box<dyn Error>> {
        Self::config_file_path()
    }
}

// Public API functions for easy use

/// Load configuration (creates default if doesn't exist)
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    Config::load()
}

/// Get the path to todos.json based on current config
pub fn get_data_path() -> PathBuf {
    match Config::load() {
        Ok(config) => config.get_todos_file_path(),
        Err(_) => {
            // Fallback to old behavior if config fails
            let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
            path.push("unsafe_todo");
            fs::create_dir_all(&path).ok();
            path.push("todos.json");
            path
        }
    }
}

/// Get the data directory (without todos.json)
pub fn get_data_dir() -> Result<PathBuf, Box<dyn Error>> {
    let config = Config::load()?;
    Ok(config.data_path)
}

/// Show current configuration
pub fn show_config() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?;
    config.show()
}

/// Set data path where todos are stored
pub fn set_data_path(new_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut config = Config::load()?;
    let old_path = config.data_path.clone();
    
    config.set_data_path(new_path.clone())?;
    
    // Try to migrate existing todos
    config.migrate_todos(&old_path)?;
    
    println!("✅ Data path updated to: {}", new_path.display());
    println!("📁 Todos file: {}", config.get_todos_file_path().display());
    
    Ok(())
}

/// Validate current configuration
pub fn validate_config() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?;
    config.validate()?;
    println!("✅ Configuration is valid");
    Ok(())
}

/// Initialize data directory and config
pub fn init_config() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?; // This creates default if needed
    config.validate()?;
    
    // Create todos.json if it doesn't exist
    let todos_file = config.get_todos_file_path();
    if !todos_file.exists() {
        fs::write(&todos_file, "[]")?; // Empty JSON array
        println!("📄 Created empty todos file: {}", todos_file.display());
    }
    
    println!("✅ Configuration initialized successfully");
    Ok(())
}

/// Reset configuration to defaults
pub fn reset_config() -> Result<(), Box<dyn Error>> {
    let config_path = Config::get_config_file_path()?;
    
    if config_path.exists() {
        fs::remove_file(&config_path)?;
        println!("🗑️  Removed existing config file");
    }
    
    let new_config = Config::load()?; // Creates new default config
    println!("✅ Configuration reset to defaults");
    new_config.show()
}

/// Get current sort order from config
pub fn get_sort_order() -> Result<SortOrder, Box<dyn Error>> {
    let config = Config::load()?;
    Ok(config.sort_order.clone())
}

/// Set sort order in config
pub fn set_sort_order(sort_order: SortOrder) -> Result<(), Box<dyn Error>> {
    let mut config = Config::load()?;
    config.set_sort_order(sort_order.clone())?;
    println!("✅ Sort order updated to: {}", sort_order);
    Ok(())
}
