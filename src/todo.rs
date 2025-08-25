use crate::config;
use crate::priority::Priority;
use crate::sort_order::SortCriteria;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::io::{self, Write};


#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub fn from_json_line(line: &str, id: u32) -> io::Result<Todo> {
        let mut todo: Todo = serde_json::from_str(line)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        todo.set_id(id);
        Ok(todo)
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let as_json = serde_json::to_string(self)?;
        let path = config::get_data_path();

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;

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

    pub fn get_finished(&self) -> bool {
        self.finished
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    pub fn set_due_date(&mut self, due_date: NaiveDate) {
        self.due_date = Some(due_date);
    }

    pub fn set_finished(&mut self, finished: bool) {
        self.finished = finished;
    }

    pub fn set_creation_date(&mut self, creation_date: DateTime<Utc>) {
        self.created_at = creation_date;
    }

    /// Compare todos by the given sort order (supports chained criteria)
    pub fn compare(&self, other: &Todo, sort_order: &SortCriteria) -> Ordering {
        self.compare_single_criterion(other, sort_order)
    }

    /// Compare todos by a single criterion
    fn compare_single_criterion(&self, other: &Todo, sort_order: &SortCriteria) -> Ordering {
        match sort_order {
            SortCriteria::Priority => {
                // High > Medium > Low
                let self_priority = self.priority.priority_value();
                let other_priority = other.priority.priority_value();
                other_priority.cmp(&self_priority) // Reverse for High->Low order
            }
            SortCriteria::PriorityReverse => {
                // Low > Medium > High
                let self_priority = self.priority.priority_value();
                let other_priority = other.priority.priority_value();
                self_priority.cmp(&other_priority)
            }
            SortCriteria::CreatedDesc => {
                // Newest first
                other.created_at.cmp(&self.created_at)
            }
            SortCriteria::CreatedAsc => {
                // Oldest first
                self.created_at.cmp(&other.created_at)
            }
            SortCriteria::DueDate => {
                // Earliest due date first, no due date last
                match (&self.due_date, &other.due_date) {
                    (Some(self_due), Some(other_due)) => self_due.cmp(other_due),
                    (Some(_), None) => Ordering::Less,
                    (None, Some(_)) => Ordering::Greater,
                    (None, None) => Ordering::Equal,
                }
            }
            SortCriteria::DueDateReverse => {
                // Latest due date first, no due date last
                match (&self.due_date, &other.due_date) {
                    (Some(self_due), Some(other_due)) => other_due.cmp(self_due),
                    (Some(_), None) => Ordering::Less,
                    (None, Some(_)) => Ordering::Greater,
                    (None, None) => Ordering::Equal,
                }
            }
            SortCriteria::TitleAsc => {
                // A-Z
                self.title.to_lowercase().cmp(&other.title.to_lowercase())
            }
            SortCriteria::TitleDesc => {
                // Z-A
                other.title.to_lowercase().cmp(&self.title.to_lowercase())
            }
            SortCriteria::Status => {
                // Unfinished first
                match (self.finished, other.finished) {
                    (false, true) => Ordering::Less,
                    (true, false) => Ordering::Greater,
                    _ => Ordering::Equal,
                }
            }
            SortCriteria::StatusReverse => {
                // Finished first
                match (self.finished, other.finished) {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    _ => Ordering::Equal,
                }
            }
        }
    }
}

pub struct TodoBuilder {
    id: u32,
    title: String,
    description: Option<String>,
    finished: Option<bool>,
    priority: Option<Priority>,
    due_date: Option<NaiveDate>,
}

impl TodoBuilder {
    pub fn new() -> Self {
        Self {
            id: 0,
            title: String::new(),
            description: None,
            finished: None,
            priority: None,
            due_date: None,
        }
    }

    pub fn id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn description(mut self, description: impl Into<Option<String>>) -> Self {
        self.description = description.into();
        self
    }

    pub fn finished(mut self, finished: impl Into<Option<bool>>) -> Self {
        self.finished = finished.into();
        self
    }

    pub fn priority(mut self, priority: impl Into<Option<Priority>>) -> Self {
        self.priority = priority.into();
        self
    }

    pub fn due_date(mut self, due_date: impl Into<Option<NaiveDate>>) -> Self {
        self.due_date = due_date.into();
        self
    }

    pub fn build(self) -> Result<Todo, String> {
        Ok(Todo {
            id: self.id,
            title: self.title,
            description: self.description,
            finished: self.finished.unwrap_or_default(),
            priority: self.priority.unwrap_or_default(),
            created_at: Utc::now(),
            due_date: self.due_date,
        })
    }
}
