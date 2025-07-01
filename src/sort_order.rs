use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SortOrder {
    /// Sort by priority (High -> Medium -> Low), with optional next criterion
    Priority(Option<Box<SortOrder>>),
    /// Sort by priority (Low -> Medium -> High), with optional next criterion
    PriorityReverse(Option<Box<SortOrder>>),
    /// Sort by creation date (newest first), with optional next criterion
    CreatedDesc(Option<Box<SortOrder>>),
    /// Sort by creation date (oldest first), with optional next criterion
    CreatedAsc(Option<Box<SortOrder>>),
    /// Sort by due date (earliest first, no due date last), with optional next criterion
    DueDate(Option<Box<SortOrder>>),
    /// Sort by due date (latest first, no due date last), with optional next criterion
    DueDateReverse(Option<Box<SortOrder>>),
    /// Sort by title alphabetically (A-Z), with optional next criterion
    TitleAsc(Option<Box<SortOrder>>),
    /// Sort by title alphabetically (Z-A), with optional next criterion
    TitleDesc(Option<Box<SortOrder>>),
    /// Sort by status (unfinished first, then finished), with optional next criterion
    Status(Option<Box<SortOrder>>),
    /// Sort by status (finished first, then unfinished), with optional next criterion
    StatusReverse(Option<Box<SortOrder>>),
}

impl SortOrder {
    /// Get all available single sort orders (without chaining)
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

    /// Parse sort order from string - supports comma-separated multi-criteria
    /// Examples: "due-date,priority", "status,due-date,priority"
    pub fn from_string(s: &str) -> Option<SortOrder> {
        let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
        Self::build_chain(&parts)
    }

    /// Build a chain of sort orders from a list of criterion strings
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

    /// Parse a single sort criterion (without chaining)
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

    /// Add a next criterion to any sort order
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

    /// Builder pattern methods for fluent API
    pub fn priority() -> Self {
        SortOrder::Priority(None)
    }

    pub fn priority_reverse() -> Self {
        SortOrder::PriorityReverse(None)
    }

    pub fn due_date() -> Self {
        SortOrder::DueDate(None)
    }

    pub fn due_date_reverse() -> Self {
        SortOrder::DueDateReverse(None)
    }

    pub fn title() -> Self {
        SortOrder::TitleAsc(None)
    }

    pub fn title_reverse() -> Self {
        SortOrder::TitleDesc(None)
    }

    pub fn status() -> Self {
        SortOrder::Status(None)
    }

    pub fn status_reverse() -> Self {
        SortOrder::StatusReverse(None)
    }

    pub fn created() -> Self {
        SortOrder::CreatedDesc(None)
    }

    pub fn created_reverse() -> Self {
        SortOrder::CreatedAsc(None)
    }

    /// Chain another sort criterion
    pub fn then(self, next: SortOrder) -> Self {
        Self::with_next_criterion(self, next)
    }

    /// Get description of the sort order
    pub fn description(&self) -> String {
        let current_desc = self.current_description();
        
        if let Some(next) = self.get_next() {
            format!("{} → {}", current_desc, next.description())
        } else {
            current_desc
        }
    }

    /// Get description of only the current criterion
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

    /// Get short description for display
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

    /// Check if this sort order has chained criteria
    pub fn is_multi(&self) -> bool {
        self.get_next().is_some()
    }

    /// Get the next criterion in the chain
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

    /// Get all criteria in the chain as a flat list
    pub fn get_all_criteria(&self) -> Vec<&SortOrder> {
        let mut criteria = vec![self];
        let mut current = self;
        
        while let Some(next) = current.get_next() {
            criteria.push(next);
            current = next;
        }
        
        criteria
    }

    /// Get the primary criterion without any chaining
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