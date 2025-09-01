use crate::todo::Todo;
use std::error::Error;

pub mod file_storage;

pub trait TodoStorage {
    fn save(&self, todo: &Todo) -> Result<(), Box<dyn Error>>;

    fn save_todos(&self, todos:&[Todo]) -> Result<(), Box<dyn Error>> {
        todos.iter().try_for_each(|todo| self.save(todo))
    }
}
