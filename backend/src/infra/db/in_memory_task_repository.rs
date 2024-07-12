use crate::domain::entities::Task;
use crate::domain::repositories::TaskRepository;
use std::sync::{Arc, Mutex};

pub struct InMemoryTaskRepository {
    tasks: Arc<Mutex<Vec<Task>>>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl TaskRepository for InMemoryTaskRepository {
    fn create(&self, task: Task) -> Result<(), String> {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(task);
        Ok(())
    }

    fn get_all(&self) -> Result<Vec<Task>, String> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks.clone())
    }
}
