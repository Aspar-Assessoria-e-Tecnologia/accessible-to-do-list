use crate::application::use_cases::create_task::CreateTaskUseCase;
use crate::infra::db::InMemoryTaskRepository;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    title: string,
    description: string,
}

pub async fn create_task(data: web::Json<CreateTaskRequest>) -> HttpResponse {
    let repository = InMemoryTaskRepository::new();
    let use_case = CreateTaskUseCase::new(repository);

    match use_case.execute(data.title.clone(), data.description.clone()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Ok")
}
