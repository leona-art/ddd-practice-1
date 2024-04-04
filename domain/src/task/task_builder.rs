use super::{priority::Priority, repository::TaskRepository, service::TaskService, status::Status, Task, TaskDescription, TaskTitle};
use chrono::prelude::*;
pub struct TaskBuilder {
    task: Task,
}

impl TaskBuilder {
    pub fn new<R:TaskRepository>(service:TaskService<R>) -> Self {
        TaskBuilder {
            task: Task{
                id: service.generate_id(),
                title: Default::default(),
                description: Default::default(),
                status: Default::default(),
                priority: Default::default(),
                deadline: Default::default(),
                created_at: Local::now(),
                updated_at: Default::default(),
            },
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.task.title = TaskTitle::new(title).unwrap();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.task.description = TaskDescription::new(description).unwrap();
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.task.status = status;
        self
    }
    pub fn priority(mut self, priority: Priority) -> Self {
        self.task.priority = priority;
        self
    }

    pub fn build(mut self) -> Task {
        self.task.updated_at =Local::now();
        self.task
    }
}

impl From<Task> for TaskBuilder {
    fn from(task: Task) -> Self {
        TaskBuilder {
            task,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::task::TaskId;

    use super::*;
    
    struct MockTaskRepository;
    impl TaskRepository for MockTaskRepository {
        type Error = std::io::Error;
        fn find_all(&self) -> Vec<Task> {
            vec![]
        }
        fn find_by_id(&self, id: TaskId) -> Result<Option<Task>, Self::Error> {
            Ok(None)
        }
        fn save(&self, task: Task) -> Result<Task, Self::Error> {
            Ok(task)
        }
        fn delete(&self, id: TaskId) -> Result<(), Self::Error> {
            Ok(())
        }
        fn find_latest(&self) -> Result<Option<Task>, Self::Error> {
            Ok(None)
        }
    }

    #[test]
    fn test_task_builder() {
        let repo=MockTaskRepository{};
        let task_builder = TaskBuilder::new(TaskService::new(repo));
        let task = task_builder
            .title("New Task")
            .description("Description")
            .status(Status::InProgress)
            .priority(Priority::Low)
            .build();
        assert_eq!(task.title.0, "New Task");
        assert_eq!(task.description.0, "Description");
        assert_eq!(task.status, Status::InProgress);
        assert_eq!(task.priority, Priority::Low);
    }
}