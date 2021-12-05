use std::sync::Arc;
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;
use crate::domain::tables::User;
use crate::error::Result;

pub struct UserModal {
    rb: Arc<Rbatis>,
}

impl std::ops::Deref for UserModal {
    type Target = Arc<Rbatis>;

    fn deref(&self) -> &Self::Target {
        &self.rb
    }
}

impl UserModal {
    pub fn new(rb: Arc<Rbatis>) -> Self {
        UserModal {
            rb,
        }
    }

    pub async fn find_by_account(&self, account: &str) -> Result<Option<User>> {
        let wrapper = self.rb.new_wrapper().eq(User::account(), account);
        return Ok(self.rb.fetch_by_wrapper(wrapper).await.unwrap());
    }
}