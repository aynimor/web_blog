pub mod user;

use actix_web::body::AnyBody;
use actix_web::{HttpResponse, Responder};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};


/// http接口返回模型结构，提供基础的 code，msg，data 等json数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: Option<i64>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone
{
    pub fn new() -> Self {
        RespVO { code: None, msg: None, data: None }
    }

    pub fn resp_json(&self) -> impl Responder {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Cache-Control", "no-cache"))
            .insert_header(("Content-Type", "text/json;charset=UTF-8"))
            .body(AnyBody::from(self.to_string()));
    }

    pub fn from(code: i64, msg: String, data: Option<T>) -> Self {
        RespVO { code: Some(code), msg: Some(msg), data: data }
    }
}

impl<T> ToString for RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}