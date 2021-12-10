use std::sync::Arc;

use argon2::{self, Config, Variant, ThreadMode, Version};
use rbatis::object_id::ObjectId;
use rbatis::rbatis::Rbatis;

use crate::domain::User;
use crate::mapper::user;
use crate::domain::dto::user::{UserAddDTO, UserLoginDTO};
use crate::error::{Result, ErrorResponse};
use crate::domain::vo::user::UserVO;
use crate::utils;

pub struct UserService {
    rb: Arc<Rbatis>,
}

// service 部分
impl UserService {
    pub async fn create_user_service(&self, arg: &UserAddDTO) -> Result<UserVO> {
        if arg.account.is_none()
            || arg.account.as_ref().unwrap().is_empty()
            || arg.nickname.is_none()
            || arg.nickname.as_ref().unwrap().is_empty()
        {
            return Err(ErrorResponse::BadRequest);
        }
        let old_user = user::find_by_account(
            &mut self.rb.as_executor(), 
            arg.account.as_ref().unwrap_or(&"".to_string())
            ).await;

        if old_user.is_ok() {
            return Err(ErrorResponse::CustomError(405, format!(
                "用户账户:{}已存在!",
                arg.account.as_ref().unwrap()
            )));
        }
        let password = arg.password.clone().unwrap_or_default();
        if password.is_empty() || password.len() < 6 {
            return Err(ErrorResponse::BadRequest)
        }
        let id = ObjectId::new();
        let salt = utils::random_u8();
        let user = User {
            id: id.to_string().into(),
            account: arg.account.clone(),
            nickname: arg.nickname.clone(),
            sign: arg.nickname.clone(),
            password: self.encrypt_password(password, salt.as_slice()),
            state: ,
            del: ,
            create_time: ,
            updated_time: ,

        };
        if let Ok(id) = user::create_user(
            &mut self.rb.as_executor(), &user
        ).await {

            Ok(UserVO::from(User))
        } else {
            return Err(ErrorResponse::InternalServerError)
        }
    }

    pub async fn user_login_service(&self, arg: &UserLoginDTO) -> Result<UserVO> {
        if arg.account.is_none()
            || arg.account.as_ref().unwrap().is_empty()
            || arg.password.is_none()
            || arg.password.as_ref().unwrap().is_empty()
        {
            return Err(ErrorResponse::BadRequest);
        }

        let user = user::find_by_account(
            &mut self.rb.as_executor(), 
            arg.account.as_ref().unwrap_or(&"".to_string())
            ).await;
        if let Ok(user) = user {
            if !self.validate_password(
                arg.password.as_ref().unwrap_or(&"".to_string()), 
            user.password.as_ref().unwrap_or(&"".to_string())
            ) {
                return Err(ErrorResponse::Unauthorized)
            }
            Ok(UserVO::from(user))
        } else {
            Err(ErrorResponse::NotFound)
        }
    }
}

// 工具实现
impl UserService {
    pub fn new(rb: Arc<Rbatis>) -> Self {
        UserService {
            rb,
        }
    }

    pub fn validate_password(&self, password: &String, envrypt_password: &String) -> bool {
        argon2::verify_encoded(envrypt_password.as_str(), password.as_bytes()).is_ok()
    }

    pub fn encrypt_password(&self, password: String, salt: &[u8]) -> Option<String> {
        let config = Config {
            variant: Variant::Argon2i,
            version: Version::Version13,
            mem_cost: 65536,
            time_cost: 10,
            lanes: 4,
            thread_mode: ThreadMode::Parallel,
            secret: &[],
            ad: &[],
            hash_length: 32
        };
        if let Ok(encry) = argon2::hash_encoded(password.as_bytes(), salt, &config) {
            Some(encry)
        } else {
            None
        }
    }
}