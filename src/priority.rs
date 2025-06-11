use std::str::FromStr;
use serde::{Deserialize, Serialize};

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

// Implementierung von FromStr für Priority
impl FromStr for Priority {
    type Err = String; // Du kannst hier auch einen eigenen Error-Typ machen

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            _ => Err(format!("Invalid priority: {}", s)),
        }
    }
}