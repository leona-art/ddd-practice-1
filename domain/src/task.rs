pub mod priority;
pub mod repository;
pub mod service;
pub mod status;
pub mod task_builder;

use chrono::prelude::*;
use priority::Priority;
use serde::{Deserialize, Serialize};
use status::Status;

#[derive(Debug, Serialize, Deserialize, Clone,PartialEq, Eq)]
pub struct Task {
    pub id: TaskId,
    pub title: TaskTitle,
    pub description: TaskDescription,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deadline: DeadLine,
    pub status: Status,
    pub priority: Priority,
}

#[derive(Debug, Serialize, Deserialize, Clone,PartialEq, Eq,PartialOrd, Ord)]
pub struct TaskId(i32);

impl From<i32> for TaskId {
    fn from(id: i32) -> Self {
        TaskId(id)
    }
}
impl From<usize> for TaskId {
    fn from(id: usize) -> Self {
        TaskId(id as i32)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone,PartialEq, Eq)]
pub struct TaskTitle(String);

impl TaskTitle {
    pub(self) fn new(title: &str) -> Result<Self, String> {
        if title.is_empty() {
            Err("Title cannot be empty".to_string())
        } else {
            Ok(TaskTitle(title.to_string()))
        }
    }
}
impl Default for TaskTitle {
    fn default() -> Self {
        TaskTitle("New Task".to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone,PartialEq, Eq)]
pub struct TaskDescription(String);
impl TaskDescription {
    pub(self) fn new(description: &str) -> Result<Self, String> {
        if description.is_empty() {
            Err("Description cannot be empty".to_string())
        } else {
            Ok(TaskDescription(description.to_string()))
        }
    }
}
impl Default for TaskDescription {
    fn default() -> Self {
        TaskDescription("".to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone,PartialEq, Eq)]
pub enum DeadLine {
    Deadline(DateTime<Local>),
    None,
}

impl Default for DeadLine {
    fn default() -> Self {
        DeadLine::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_title() {
        let title = TaskTitle::new("New Task");
        assert_eq!(title.unwrap().0, "New Task");
    }

    #[test]
    fn test_task_description() {
        let description = TaskDescription::new("Description");
        assert_eq!(description.unwrap().0, "Description");
    }

    #[test]
    fn test_task_id_from_i32() {
        let id = TaskId::from(1);
        assert_eq!(id.0, 1);
    }

    #[test]
    fn test_task_id_from_usize() {
        let id = TaskId::from(1);
        assert_eq!(id.0, 1);
    }

    #[test]
    fn test_task_deadline() {
        let deadline = DeadLine::Deadline(Local::now());
        match deadline {
            DeadLine::Deadline(_) => assert!(true),
            DeadLine::None => assert!(false),
        }
    }
}