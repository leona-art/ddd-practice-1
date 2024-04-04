use super::{repository::TaskRepository, TaskId};

pub struct TaskService<R>
where R: TaskRepository {
    task_repository: R,
}

impl<R> TaskService<R>
where R: TaskRepository {
    pub fn new(task_repository: R) -> Self {
        TaskService {
            task_repository,
        }
    }
    pub fn generate_id(&self) -> TaskId {
        let task=self.task_repository.find_latest().unwrap();
        match task {
            Some(task) => TaskId(task.id.0 + 1),
            None => TaskId::from(0),
        }
    }
}