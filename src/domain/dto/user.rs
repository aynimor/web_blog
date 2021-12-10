use serde::{Deserialize, Serialize};

// 创建用户
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserAddDTO {
    pub account: Option<String>,
    pub nickname: Option<String>,
    pub sign: Option<String>,
    pub password: Option<String>,
}

// 用户登录
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserLoginDTO {
    pub account: Option<String>,
    pub password: Option<String>,
}
