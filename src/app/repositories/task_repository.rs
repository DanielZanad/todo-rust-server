use crate::app::entities::task::Task;

pub trait TaskRepository {
    fn save(task: Task) {}
}
