use super::{Task, TaskId};

pub trait TaskRepository {
    type Error: std::error::Error;
    fn find_all(&self) -> Vec<Task>;
    fn find_by_id(&self, id: TaskId) -> Result<Option<Task>,Self::Error>;
    fn save(&self, task: Task) -> Result<Task, Self::Error>;
    fn delete(&self, id: TaskId) -> Result<(), Self::Error>;
    fn find_latest(&self)-> Result<Option<Task>, Self::Error>;
}