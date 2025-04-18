use std::future::Future;
use std::pin::Pin;

use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};

use crate::app::entities::task::{ActiveModel, Model};
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
            let result = active_model.save(db_conn).await.unwrap();
            println!("{:?}", result);
        })
    }
}
