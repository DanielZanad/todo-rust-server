use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::app::{entities::task::Model, repositories::task_repository::TaskRepository};

pub struct InsertTaskRequest {
    pub name: String,
    pub content: String,
    pub state: String,
}

impl InsertTaskRequest {
    pub fn new(name: String, content: String, state: String) -> Self {
        Self {
            name,
            content,
            state,
        }
    }
}

pub struct InsertTask {
    task_repository: Box<dyn TaskRepository>,
}

impl InsertTask {
    pub fn new(task_repository: Box<dyn TaskRepository>) -> Self {
        Self { task_repository }
    }
    pub fn execute(&self, request: InsertTaskRequest, db_conn: DatabaseConnection) {
        let task = Model::new(request.name, request.content, request.state);

        self.task_repository.save(task, db_conn);
    }
}
