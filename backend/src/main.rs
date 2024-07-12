mod domain;
mod application;
mod infra;

use actix_web::{App, HttpServer, web};
use infra::web::handlers::{create_task, health_check};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/tasks", web::post().to(create_task))
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
