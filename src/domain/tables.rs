use rbatis::{DateTimeNative, crud_table, impl_field_name_method};

#[crud_table]
#[derive(Clone, Debug)]
pub struct User {
    pub id: Option<String>,
    pub account: Option<String>,
    pub nickname: Option<String>,
    pub profile: Option<String>,
    pub password: Option<String>,
    pub state: Option<i32>,
    pub del: Option<i32>,
    pub create_date: Option<DateTimeNative>,
}
impl_field_name_method!(User{id, account, password, nickname, state, del, create_date});
