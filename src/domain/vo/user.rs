use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};
use crate::domain::User;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserVO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub nickname: Option<String>,
    pub sign: Option<String>,
    pub create_time: Option<DateTimeNative>,
    pub updated_time: Option<DateTimeNative>,
}

impl From<User> for UserVO {
    fn from(arg: User) -> Self {
        Self {
            id: arg.id,
            account: arg.account,
            nickname: arg.nickname,
            sign: arg.sign,
            create_time: arg.create_time,
            updated_time: arg.updated_time,
        }
    }
}