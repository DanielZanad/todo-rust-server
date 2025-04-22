use std::sync::Arc;

use actix_web::{get, http::StatusCode, post, web, Error, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    app::{
        repositories::task_repository::TaskRepository,
        use_cases::{
            insert_task::{InsertTask, InsertTaskRequest},
            list_all_tasks::ListAllTasks,
        },
    },
    infra::database::sea_orm::sea_orm_repository::SeaOrmRepository,
};

#[derive(Deserialize)]
struct Task {
    name: String,
    content: String,
    state: String,
}

#[post("/task")]
pub async fn insert_task(db: web::Data<DatabaseConnection>, task: web::Json<Task>) -> HttpResponse {
    let task = InsertTaskRequest::new(task.name.clone(), task.content.clone(), task.state.clone());
    let task_repository = Arc::new(SeaOrmRepository);

    let use_case = InsertTask::new(task_repository);
    use_case.execute(task, &db).await;

    HttpResponse::Ok().status(StatusCode::OK).finish()
}

#[get("/tasks")]
pub async fn list_all_tasks(db: web::Data<DatabaseConnection>) -> Result<impl Responder, Error> {
    let task_repository = Arc::new(SeaOrmRepository);

    let use_case = ListAllTasks::new(task_repository);
    let tasks = use_case.execute(&db).await;

    Ok(web::Json(tasks))
}

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// extract `Info` using serde
#[post("/info")]
pub async fn index(info: web::Json<Info>) -> Result<String, Error> {
    Ok(format!("Welcome {}!", info.username))
}
