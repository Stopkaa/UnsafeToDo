use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{self,Write, BufRead, BufReader};
use crate::utils;

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

    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let as_json = serde_json::to_string(self)?;
        let path = utils::get_data_path();
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(file, "{}", as_json)?;
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

   pub fn load() -> io::Result<Self> {
        let path = utils::get_data_path();
        if !path.exists() {
            return Ok(TodoList::new());
        }
        let mut list = Self::new();
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let todo: Todo = serde_json::from_str(&line)?;
            list.todos.push(todo);
        }
        Ok(list)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = utils::get_data_path();
        File::create(path)?;
        self.todos
            .iter()
            .try_for_each(|todo| todo.save_to_file())
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
