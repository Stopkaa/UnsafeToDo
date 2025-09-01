use crate::storage::TodoStorage;
use crate::todo::Todo;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

pub struct TodoFileStorage {
    data_path: PathBuf,
}

impl TodoFileStorage {
    pub fn new(data_path: PathBuf) -> Self {
        Self { data_path }
    }
}

impl TodoStorage for TodoFileStorage {
    fn save(&self, todo: &Todo) -> Result<(), Box<dyn Error>> {
        let as_json = serde_json::to_string(todo)?;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.data_path)?;
        writeln!(file, "{}", as_json)?;
        Ok(())
    }
}
