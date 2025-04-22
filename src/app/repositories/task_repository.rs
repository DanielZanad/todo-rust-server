use std::{future::Future, pin::Pin};

use sea_orm::DatabaseConnection;

use crate::app::entities::task::Model;

pub trait TaskRepository: Send + Sync {
    fn save<'a>(
        &'a self,
        task: Model,
        db_conn: &'a DatabaseConnection,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
    fn list_all_tasks<'a>(
        &'a self,
        db_conn: &'a DatabaseConnection,
    ) -> Pin<Box<dyn Future<Output = Vec<Model>> + Send + 'a>>;
}
