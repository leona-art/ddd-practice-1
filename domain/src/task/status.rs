use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
    Archived,
}

impl Default for Status {
    fn default() -> Self {
        Status::NotStarted
    }
}