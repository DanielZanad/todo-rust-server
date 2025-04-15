use crate::app::entities::task::Task;
use crate::app::repositories::task_repository::TaskRepository;
pub struct SeaOrmRepository {}

impl TaskRepository for SeaOrmRepository {
    fn save(task: Task) {}
}
