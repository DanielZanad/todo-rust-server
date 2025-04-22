use crate::app::{entities::task::Model, repositories::task_repository::TaskRepository};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ListAllTasksResponse {
    tasks: Vec<Model>,
}

impl ListAllTasksResponse {
    pub fn new(tasks: Vec<Model>) -> Self {
        Self { tasks }
    }
}

pub struct ListAllTasks {
    task_repository: Arc<dyn TaskRepository>,
}

impl ListAllTasks {
    pub fn new(task_repository: Arc<dyn TaskRepository>) -> Self {
        Self { task_repository }
    }

    pub async fn execute(&self, db_conn: &DatabaseConnection) -> ListAllTasksResponse {
        let tasks = self.task_repository.list_all_tasks(db_conn).await;
        ListAllTasksResponse::new(tasks)
    }
}
