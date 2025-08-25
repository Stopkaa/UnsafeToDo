use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, ValueEnum)]
pub enum SortCriteria {
    Priority,
    PriorityReverse,
    #[default]
    CreatedDesc,
    CreatedAsc,
    DueDate,
    DueDateReverse,
    TitleAsc,
    TitleDesc,
    Status,
    StatusReverse,
}
