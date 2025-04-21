use std::sync::Arc;

use actix_web::{http::StatusCode, post, web, Error, HttpResponse};
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};
use serde::Deserialize;

use crate::{
    app::{
        entities::task::{ActiveModel, Model},
        repositories::task_repository,
        use_cases::insert_task::{InsertTask, InsertTaskRequest},
    },
    infra::database::sea_orm::sea_orm_repository::{self, SeaOrmRepository},
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

#[derive(Deserialize)]
struct Info {
    username: String,
}

/// extract `Info` using serde
#[post("/info")]
pub async fn index(info: web::Json<Info>) -> Result<String, Error> {
    Ok(format!("Welcome {}!", info.username))
}
