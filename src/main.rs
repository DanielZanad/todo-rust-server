<<<<<<< HEAD
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use infra::database::configuration::get_configuration;
use sea_orm::Database;
=======
use std::ops::Add;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use infra::database::sea_orm::configuration::get_configuration;
>>>>>>> origin/database_migration

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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(
                database_settings.database_connection.clone(),
            ))
            .service(hello)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
