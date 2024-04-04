use std::{io::{Read, Write}, sync::Mutex};

use domain::task::{repository::TaskRepository, Task};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileTaskRepositoryError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

pub struct FileTaskRepository{
    file_path: String,
    tasks: Mutex<Vec<Task>>,
}

impl FileTaskRepository{
    pub fn new(file_path: &str) -> Result<Self, FileTaskRepositoryError> {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .map_err(FileTaskRepositoryError::Io)?;

        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        let tasks: Vec<Task> = serde_json::from_str(&buf).unwrap();
        Ok(FileTaskRepository {
            file_path: file_path.to_string(),
            tasks: Mutex::new(tasks),
        })
    }
}

impl Drop for FileTaskRepository {
    fn drop(&mut self) {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.file_path)
            .unwrap();
        let tasks = serde_json::to_string(&self.tasks).unwrap();
        file.write_all(tasks.as_bytes()).unwrap();
    }
    
}

impl TaskRepository for FileTaskRepository{
    type Error=FileTaskRepositoryError;

    fn find_all(&self) -> Vec<Task> {
        self.tasks.lock().unwrap().clone()
    }

    fn find_by_id(&self, id: domain::task::TaskId) -> Result<Option<Task>,Self::Error> {
        let tasks=self.tasks.lock().unwrap();
        let task = tasks.iter().find(|task| task.id == id);
        Ok(task.cloned())
    }

    fn save(&self, task: Task) -> Result<Task, Self::Error> {
        let mut tasks = self.tasks.lock().unwrap();
        let index = tasks.iter().position(|t| t.id == task.id);
        match index {
            Some(index) => {
                tasks[index] = task.clone();
                Ok(task)
            }
            None => {
                tasks.push(task.clone());
                Ok(task)
            }
        }
    }

    fn delete(&self, id: domain::task::TaskId) -> Result<(), Self::Error> {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(index) = tasks.iter().position(|task| task.id == id){
            tasks.remove(index);
        };
        Ok(())
    }

    fn find_latest(&self)-> Result<Option<Task>, Self::Error> {
        let tasks=self.tasks.lock().unwrap();
        let task=tasks.iter().reduce(|a, b| if a.id > b.id { a } else { b });
        Ok(task.cloned())
    }
}


