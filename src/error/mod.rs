#[warn(dead_code)]
use std::fmt::{Display, Result as fmtResult};

use crate::domain::vo::RespVO;
use actix_http::{http::StatusCode, http::header};
use actix_web::{HttpResponse, ResponseError, body::AnyBody};

pub type Result<T> = actix_web::Result<T, ErrorResponse>;

#[derive(Debug)]
pub enum ErrorResponse {
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    RequestTimeout,
    PayloadTooLarge,
    UnsupportedMediaType,
    Locked,
    TooManyRequests,
    InternalServerError,
    CustomError(i64, String),
}


impl Display for ErrorResponse {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> fmtResult {
        Ok(())
    }
}

impl ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        let mut resp: RespVO<String> = RespVO::new();
        match self {
            ErrorResponse::BadRequest => {
                resp.code = Some(400);
                resp.msg = Some("Invalid Parameter!".to_string());
            },
            ErrorResponse::Unauthorized => {
                resp.code = Some(401);
                resp.msg = Some("Unauthorized!".to_string());
            },
            ErrorResponse::PaymentRequired => {
                resp.code = Some(402);
                resp.msg = Some("Payment Required!".to_string());
            },
            ErrorResponse::Forbidden => {
                resp.code = Some(403);
                resp.msg = Some("Forbidden!".to_string());
            },
            ErrorResponse::NotFound => {
                resp.code = Some(404);
                resp.msg = Some("404 Not Found!".to_string());
            },
            ErrorResponse::MethodNotAllowed => {
                resp.code = Some(405);
                resp.msg = Some("Method Not Allowed!".to_string());
            },
            ErrorResponse::NotAcceptable => {
                resp.code = Some(406);
                resp.msg = Some("Not Acceptable!".to_string());
            },
            ErrorResponse::RequestTimeout => {
                resp.code = Some(408);
                resp.msg = Some("Request Timeout!".to_string());
            },
            ErrorResponse::PayloadTooLarge => {
                resp.code = Some(413);
                resp.msg = Some("Payload TooLarge!".to_string());
            },
            ErrorResponse::UnsupportedMediaType => {
                resp.code = Some(415);
                resp.msg = Some("Unsupported Media Type!".to_string());
            },
            ErrorResponse::Locked => {
                resp.code = Some(423);
                resp.msg = Some("Locked!".to_string());
            },
            ErrorResponse::TooManyRequests => {
                resp.code = Some(429);
                resp.msg = Some("TooMany Requests!".to_string());
            },
            ErrorResponse::InternalServerError => {
                resp.code = Some(500);
                resp.msg = Some("Internal Server Error!".to_string());
            }
            ErrorResponse::CustomError(code, msg) => {
                resp.code = Some(code.to_owned());
                resp.msg = Some(msg.to_owned());
            }
        }
        let resp = resp.to_string();
        let mut res = HttpResponse::new(StatusCode::OK);
        res.headers_mut().insert(
            header::CONTENT_TYPE, 
            header::HeaderValue::from_static("application/json; charset=utf-8"),    
        );
        res.set_body(AnyBody::from(resp))
    }
}
