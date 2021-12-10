// 负责参数校验并返回数据
pub mod user;

use actix_web::{Responder, HttpResponse};

pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}
