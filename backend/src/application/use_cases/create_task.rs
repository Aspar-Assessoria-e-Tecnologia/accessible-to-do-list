use crate::domain::entities::Task;
use crate::domain::repositories::TaskRepository;

pub struct CreateTaskUseCase<R: TaskRepository> {
    repository: R,
}

impl<R: TaskRepository> CreateTaskUseCase<R> {
    pub fn new(repositoryL: R) -> Self {
        Self {
            repository
        }
    }

    pub fn execute(&self, title: string, description: string) -> Result<Task, String> {
        let task = Task::new(title, description);
        self.repository.save(task.clone())?;
        Ok(task)
    }
}