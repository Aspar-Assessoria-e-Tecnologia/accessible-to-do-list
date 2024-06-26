use actix_web::web;
use crate::handlers::handlers::{get_tasks, add_task};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .route("", web::get().to(get_tasks))
            .route("", web::post().to(add_task))
    );
}
