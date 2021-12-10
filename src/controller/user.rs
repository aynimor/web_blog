use actix_web::{HttpResponse};
use crate::error::{Result, ErrorResponse};

pub async fn create_user() -> Result<HttpResponse> {
    Err(ErrorResponse::InternalServerError)
}