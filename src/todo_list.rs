use crate::sort_order::SortCriteria;
use crate::sync::GitRepo;
use crate::{config, todo::Todo};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoList {
    pub todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { todos: Vec::new() }
    }

    pub fn load() -> io::Result<Self> {
        let path = config::get_data_path();
        if !path.exists() {
            return Ok(TodoList::new());
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut list = Self::new();

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            if !line.trim().is_empty() {
                let todo = Todo::from_json_line(&line, i as u32)?;
                list.add(todo);
            }
        }

        Ok(list)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = config::get_data_path();
        File::create(path)?;
        self.todos.iter().try_for_each(|todo| todo.save_to_file())?;

        let data_dir = config::get_data_dir()?;
        let repo = GitRepo::new(data_dir);

        if let Err(e) = repo.sync_file("todos.json") {
            //TODO "todos.json" auch config
            eprintln!("Git sync failed: {}", e);
        }

        Ok(())
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

    /// Sort todos by the given sort order
    pub fn sort_by_order(&mut self, sort_order: &Vec<SortCriteria>) {
        self.todos.sort_by(|a, b| {
            for criteria in sort_order {
                let ordering = a.compare(b, criteria);
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
            Ordering::Equal
        });
    }
}

pub fn todos_from_json_lines(lines: &[String]) -> Vec<Todo> {
    lines
        .iter()
        .filter_map(|line| Todo::from_json_line(line, 0).ok())
        .collect()
}
