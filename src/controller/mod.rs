use actix_web::{Responder, HttpResponse};

pub mod user;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("H")
}