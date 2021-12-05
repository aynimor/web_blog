use serde::{Deserialize, Serialize};

// 创建用户
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserAddDTO {
    pub account: Option<String>,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub profile: Option<String>,
}