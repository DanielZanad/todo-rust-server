use std::future::Future;
use std::pin::Pin;

use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel};

use crate::app::entities::task::Model;
use crate::app::repositories::task_repository::TaskRepository;
pub struct SeaOrmRepository;

impl TaskRepository for SeaOrmRepository {
    fn save<'a>(
        &'a self,
        task: Model,
        db_conn: &'a DatabaseConnection,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            let active_model = task.into_active_model();
            println!("sea orm repository");
            let result = active_model.insert(db_conn).await.unwrap();
            println!("{:?}", result);
        })
    }

    fn list_all_tasks<'a>(
        &'a self,
        db_conn: &'a DatabaseConnection,
    ) -> Pin<Box<dyn Future<Output = Vec<Model>> + Send + 'a>> {
        Box::pin(async move {
            let tasks = crate::app::entities::task::Entity::find()
                .all(db_conn)
                .await
                .unwrap_or_else(|_| vec![]);
            tasks
        })
    }
}
