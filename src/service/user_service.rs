use crate::domain::dto::user::UserAddDTO;
use crate::dao::user::UserModal;
use crate::error::{Result, Error, CustomFrom};

pub struct UserService {
    user: UserModal,
}

impl UserService {
    pub fn new(user: UserModal) -> Self {
        Self {
            user
        }
    }

    pub async fn create_user(&self, arg: &UserAddDTO) -> Result<u64> {
        if arg.account.is_none()
            || arg.account.as_ref().unwrap().is_empty()
            || arg.nickname.is_none()
            || arg.nickname.as_ref().unwrap().is_empty()
        {
            return Err(Error::from_res(400, "用户名和姓名不能为空!"));
        }
        let old_user = self.user
            .find_by_account(arg.account.as_ref().unwrap_or(&"".to_string()))
            .await?;
        if old_user.is_some() {
            return Err(Error::from_res(405, format!(
                "用户账户:{}已存在!",
                arg.account.as_ref().unwrap()
            )));
        }
        Ok(0)
    }
}
