use crate::utils;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn to_string(&self) -> String {
        match self {
            Priority::Low => String::from("Low"),
            Priority::Medium => String::from("Medium"),
            Priority::High => String::from("High"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: u32,
    title: String,
    description: Option<String>,
    finished: bool,
    priority: Priority,
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
            priority: Priority::Low,
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

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due) = &self.due_date {
            let today = chrono::Utc::now().date_naive();
            due < &today
        } else {
            false
        }
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_description(&self) -> String {
        self.description.clone().unwrap_or_default()
    }

    pub fn get_creation_date(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn get_due_date(&self) -> Option<NaiveDate> {
        self.due_date
    }

    pub fn get_priority(&self) -> &Priority {
        &self.priority
    }

    pub fn set_id(&mut self, id: u32){
        self.id = id;
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

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            if !line.trim().is_empty() {
                let mut todo: Todo = serde_json::from_str(&line)?;
                todo.set_id(i as u32);
                list.add(todo);
            }
        }
        Ok(list)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = utils::get_data_path();
        File::create(path)?;
        self.todos.iter().try_for_each(|todo| todo.save_to_file())
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
