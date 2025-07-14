use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SortOrder {
    Priority(Option<Box<SortOrder>>),
    
    PriorityReverse(Option<Box<SortOrder>>),
    
    CreatedDesc(Option<Box<SortOrder>>),
    
    CreatedAsc(Option<Box<SortOrder>>),
    
    DueDate(Option<Box<SortOrder>>),
    
    DueDateReverse(Option<Box<SortOrder>>),
    
    TitleAsc(Option<Box<SortOrder>>),
    
    TitleDesc(Option<Box<SortOrder>>),
    
    Status(Option<Box<SortOrder>>),
    
    StatusReverse(Option<Box<SortOrder>>),
}

impl SortOrder {
    pub fn all_single() -> Vec<SortOrder> {
        vec![
            SortOrder::Priority(None),
            SortOrder::PriorityReverse(None),
            SortOrder::CreatedDesc(None),
            SortOrder::CreatedAsc(None),
            SortOrder::DueDate(None),
            SortOrder::DueDateReverse(None),
            SortOrder::TitleAsc(None),
            SortOrder::TitleDesc(None),
            SortOrder::Status(None),
            SortOrder::StatusReverse(None),
        ]
    }

   pub fn from_string(s: &str) -> Option<SortOrder> {
        let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
        Self::build_chain(&parts)
    }

    fn build_chain(parts: &[&str]) -> Option<SortOrder> {
        if parts.is_empty() {
            return None;
        }

        let current = Self::parse_single_criterion(parts[0])?;
        
        if parts.len() == 1 {
            Some(current)
        } else {
            let next_chain = Self::build_chain(&parts[1..])?;
            Some(Self::with_next_criterion(current, next_chain))
        }
    }

    fn parse_single_criterion(s: &str) -> Option<SortOrder> {
        match s.to_lowercase().as_str() {
            "priority" | "prio" => Some(SortOrder::Priority(None)),
            "priority-reverse" | "prio-reverse" | "prio-rev" => Some(SortOrder::PriorityReverse(None)),
            "created" | "created-desc" | "newest" => Some(SortOrder::CreatedDesc(None)),
            "created-asc" | "oldest" => Some(SortOrder::CreatedAsc(None)),
            "due" | "due-date" => Some(SortOrder::DueDate(None)),
            "due-reverse" | "due-rev" => Some(SortOrder::DueDateReverse(None)),
            "title" | "title-asc" | "alphabetical" => Some(SortOrder::TitleAsc(None)),
            "title-desc" | "title-reverse" => Some(SortOrder::TitleDesc(None)),
            "status" => Some(SortOrder::Status(None)),
            "status-reverse" | "status-rev" => Some(SortOrder::StatusReverse(None)),
            _ => None,
        }
    }

    fn with_next_criterion(current: SortOrder, next: SortOrder) -> SortOrder {
        match current {
            SortOrder::Priority(_) => SortOrder::Priority(Some(Box::new(next))),
            SortOrder::PriorityReverse(_) => SortOrder::PriorityReverse(Some(Box::new(next))),
            SortOrder::CreatedDesc(_) => SortOrder::CreatedDesc(Some(Box::new(next))),
            SortOrder::CreatedAsc(_) => SortOrder::CreatedAsc(Some(Box::new(next))),
            SortOrder::DueDate(_) => SortOrder::DueDate(Some(Box::new(next))),
            SortOrder::DueDateReverse(_) => SortOrder::DueDateReverse(Some(Box::new(next))),
            SortOrder::TitleAsc(_) => SortOrder::TitleAsc(Some(Box::new(next))),
            SortOrder::TitleDesc(_) => SortOrder::TitleDesc(Some(Box::new(next))),
            SortOrder::Status(_) => SortOrder::Status(Some(Box::new(next))),
            SortOrder::StatusReverse(_) => SortOrder::StatusReverse(Some(Box::new(next))),
        }
    }

    pub fn description(&self) -> String {
        let current_desc = self.current_description();
        
        if let Some(next) = self.get_next() {
            format!("{} → {}", current_desc, next.description())
        } else {
            current_desc
        }
    }

    pub fn current_description(&self) -> String {
        match self {
            SortOrder::Priority(_) => "Priority (High → Medium → Low)".to_string(),
            SortOrder::PriorityReverse(_) => "Priority (Low → Medium → High)".to_string(),
            SortOrder::CreatedDesc(_) => "Creation date (newest first)".to_string(),
            SortOrder::CreatedAsc(_) => "Creation date (oldest first)".to_string(),
            SortOrder::DueDate(_) => "Due date (earliest first)".to_string(),
            SortOrder::DueDateReverse(_) => "Due date (latest first)".to_string(),
            SortOrder::TitleAsc(_) => "Title (A → Z)".to_string(),
            SortOrder::TitleDesc(_) => "Title (Z → A)".to_string(),
            SortOrder::Status(_) => "Status (unfinished first)".to_string(),
            SortOrder::StatusReverse(_) => "Status (finished first)".to_string(),
        }
    }

    pub fn short_description(&self) -> String {
        match self {
            SortOrder::Priority(_) => "Priority↓".to_string(),
            SortOrder::PriorityReverse(_) => "Priority↑".to_string(),
            SortOrder::CreatedDesc(_) => "Newest".to_string(),
            SortOrder::CreatedAsc(_) => "Oldest".to_string(),
            SortOrder::DueDate(_) => "Due↑".to_string(),
            SortOrder::DueDateReverse(_) => "Due↓".to_string(),
            SortOrder::TitleAsc(_) => "Title↑".to_string(),
            SortOrder::TitleDesc(_) => "Title↓".to_string(),
            SortOrder::Status(_) => "Unfinished first".to_string(),
            SortOrder::StatusReverse(_) => "Finished first".to_string(),
        }
    }

    pub fn get_next(&self) -> Option<&SortOrder> {
        match self {
            SortOrder::Priority(next) => next.as_deref(),
            SortOrder::PriorityReverse(next) => next.as_deref(),
            SortOrder::CreatedDesc(next) => next.as_deref(),
            SortOrder::CreatedAsc(next) => next.as_deref(),
            SortOrder::DueDate(next) => next.as_deref(),
            SortOrder::DueDateReverse(next) => next.as_deref(),
            SortOrder::TitleAsc(next) => next.as_deref(),
            SortOrder::TitleDesc(next) => next.as_deref(),
            SortOrder::Status(next) => next.as_deref(),
            SortOrder::StatusReverse(next) => next.as_deref(),
        }
    }

   pub fn without_chain(&self) -> SortOrder {
        match self {
            SortOrder::Priority(_) => SortOrder::Priority(None),
            SortOrder::PriorityReverse(_) => SortOrder::PriorityReverse(None),
            SortOrder::CreatedDesc(_) => SortOrder::CreatedDesc(None),
            SortOrder::CreatedAsc(_) => SortOrder::CreatedAsc(None),
            SortOrder::DueDate(_) => SortOrder::DueDate(None),
            SortOrder::DueDateReverse(_) => SortOrder::DueDateReverse(None),
            SortOrder::TitleAsc(_) => SortOrder::TitleAsc(None),
            SortOrder::TitleDesc(_) => SortOrder::TitleDesc(None),
            SortOrder::Status(_) => SortOrder::Status(None),
            SortOrder::StatusReverse(_) => SortOrder::StatusReverse(None),
        }
    }
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::CreatedDesc(None)
    }
}

impl fmt::Display for SortOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
