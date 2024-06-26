use actix_web::HttpResponse;

pub async fn get_tasks() -> HttpResponse {
    HttpResponse::Ok().json("Get tasks")
}

pub async fn add_task() -> HttpResponse {
    HttpResponse::Ok().json("Add task")
}