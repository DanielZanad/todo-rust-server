use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};

use crate::app::entities::task::Model;
use crate::app::repositories::task_repository::TaskRepository;
pub struct SeaOrmRepository {}

impl TaskRepository for SeaOrmRepository {
    fn save(&self, task: Model, db_conn: DatabaseConnection) {
        let active_model = task.into_active_model();
        active_model.insert(&db_conn);
    }
}
