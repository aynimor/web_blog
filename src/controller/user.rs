use actix_web::{Responder, HttpResponse};

pub async fn create_user() -> impl Responder {
    HttpResponse::Ok().body("Hello World".to_string())
}