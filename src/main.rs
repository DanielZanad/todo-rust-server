use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use infra::{database::sea_orm::configuration::get_configuration, http::task_controller};
use sea_orm::Database;

mod app;
mod infra;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hey there")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_settings = get_configuration()
        .await
        .expect("failed to connect to database");

    let conn = database_settings.database_connection.unwrap();

    // let database_url = "postgres://docker:docker@localhost:5432/todo?currentSchema=public";
    // let conn = Database::connect(database_url)
    //     .await
    //     .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .service(task_controller::insert_task)
            .service(task_controller::index)
            .service(hello)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
