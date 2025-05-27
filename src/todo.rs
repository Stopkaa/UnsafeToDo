use chrono::{DateTime, NaiveDate, Utc};
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{self, PathBuf};
use dirs::data_local_dir;


#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: u32,
    title: String,
    description: Option<String>,
    finished: bool,
    priority: u32,
    created_at: DateTime<Utc>,
    due_date: Option<NaiveDate>,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            id: 0,
            title,
            description: None,
            finished: false,
            priority: 0,
            created_at: Utc::now(),
            due_date: None,
        }
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut as_cbor = serde_cbor::to_vec(self)?;
        //as_cbor.push(b'\n');
        let mut file = OpenOptions::new()
            .create(true) 
            .append(true)
            .open(path)?;
        
        file.write_all(&as_cbor)?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { todos: Vec::new() }
    }

    fn get_data_path() -> PathBuf {
        let mut path = data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("unsafe_todo");
        fs::create_dir_all(&path).ok();
        path.push("todos.cbor");
        path
    }

    pub fn load() -> io::Result<Self> {
        let path = Self::get_data_path();
        if !path.exists() {
            return Ok(TodoList::new());
        }
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let list = serde_cbor::from_slice(&buffer).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(list)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_data_path();
        let path = path.to_str().unwrap_or("todo.txt");
        self.todos.iter().try_for_each(|todo| todo.save_to_file(&path))
    }

    pub fn add(&mut self, task: Todo) {
        self.todos.push(task);
    }

    pub fn remove(&mut self, index: usize) -> Option<Todo> {
        if index < self.todos.len() {
            Some(self.todos.remove(index))
        } else {
            None
        }
    }
}

