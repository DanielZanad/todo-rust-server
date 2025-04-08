use actix_web::{get, App, HttpResponse, HttpServer, Responder};

mod app;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hey there")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
