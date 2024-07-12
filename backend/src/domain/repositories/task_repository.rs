use crate::domain::entities::task::Task;

pub trait TaskRepository {
    fn create(&self, task: Task) -> Result<(), String>;
    fn get_all(&self) -> Result<Vec<Task>, String>;
}
