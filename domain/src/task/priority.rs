use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone,PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Low
    }
}

