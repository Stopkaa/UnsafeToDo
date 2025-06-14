use crate::utils;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use crate::priority::Priority;

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

    pub fn complete(&mut self, complete: bool) {
        self.finished = complete;
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


// --------- Builder ------------

pub struct TodoBuilder {
    id: u32,
    title: Option<String>,
    description: Option<String>,
    finished: bool,
    priority: Priority,
    created_at: DateTime<Utc>,
    due_date: Option<NaiveDate>,
}

impl TodoBuilder {
    pub fn new() -> Self {
        Self {
            id: 0,
            title: None,
            description: None,
            finished: false,
            priority: Priority::Low,
            created_at: Utc::now(),
            due_date: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn finished(mut self, finished: bool) -> Self {
        self.finished = finished;
        self
    }

    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    pub fn due_date(mut self, due_date: NaiveDate) -> Self {
        self.due_date = Some(due_date);
        self
    }

    pub fn build(self) -> Result<Todo, String> {
        if let Some(title) = self.title {
            Ok(Todo {
                id: self.id,
                title,
                description: self.description,
                finished: self.finished,
                priority: self.priority,
                created_at: self.created_at,
                due_date: self.due_date,
            })
        } else {
            Err("Title is required".into())
        }
    }
}
//-------------------------

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

    pub fn get_todo(&self, id: usize) -> Option<&Todo> {
        self.todos.get(id)
    }
    
    pub fn get_todo_mut(&mut self, id: usize) -> Option<&mut Todo> {
        self.todos.get_mut(id)
    }
}
