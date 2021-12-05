use crate::error::Error;
use actix_web::{HttpResponse, Responder};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub const CODE_SUCCESS: u64 = 0;
pub const CODE_FAIL: u64 = 4000;

/// http接口返回模型结构，提供基础的 code，msg，data 等json数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: Option<u64>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &Result<T, Error>) -> Self {
        match arg {
            Ok(data) => {
                return Self {
                    code: Some(CODE_SUCCESS),
                    msg: None,
                    data: Some(data.clone()),
                }
            },
            Err(Error::E(code, msg)) => {
                return Self {
                    code: Some(code.to_owned()),
                    msg: Some(msg.to_owned()),
                    data: None,
                }
            }
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            code: Some(CODE_SUCCESS),
            msg: None,
            data: Some(arg.clone()),
        }
    }

    pub fn from_error(arg: &Error) -> Self {
        let Error::E(code, msg) = arg.clone();
        Self {
            code: Some(code.clone()),
            msg: Some(msg.clone()),
            data: None,
        }
    }

    pub fn from_error_info(info: &str) -> Self {
        Self {
            code: Some(CODE_FAIL),
            msg: Some(info.to_string()),
            data: None,
        }
    }

    pub fn resp_json(&self) -> impl Responder {
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Cache-Control", "no-cache"))
            .insert_header(("Content-Type", "text/json;charset=UTF-8"))
            .body(self.to_string());
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