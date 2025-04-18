use sea_orm::DatabaseConnection;

use crate::app::entities::task::Model;

pub trait TaskRepository {
    fn save(&self, task: Model, db_conn: DatabaseConnection) {}
}
