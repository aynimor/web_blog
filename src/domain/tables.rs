#[warn(unused_must_use)]

use rbatis::{DateTimeNative, crud_table, impl_field_name_method};

#[crud_table]
#[derive(Clone, Debug)]
pub struct User {
    pub id: Option<String>,
    pub account: Option<String>,
    pub nickname: Option<String>,
    pub sign: Option<String>,
    pub password: Option<String>,
    pub state: Option<i32>,
    pub del: Option<i32>,
    pub create_time: Option<DateTimeNative>,
    pub updated_time: Option<DateTimeNative>,
}

impl_field_name_method!(User{id, account, password, nickname, sign, state, del, create_time, updated_time});


#[crud_table]
#[derive(Clone, Debug)]
pub struct Article {
    pub id: Option<String>,
    pub title: Option<String>,
    pub profile: Option<String>,
    pub view: Option<u64>,
    pub state: Option<i32>,
    pub del: Option<i32>,
    pub created_time: Option<DateTimeNative>,
    pub updated_time: Option<DateTimeNative>,
}

impl_field_name_method!(Article{id, title, profile, view, state, del, created_time, updated_time});


#[crud_table]
#[derive(Clone, Debug)]
pub struct About {
    pub id: Option<String>,
    pub title: Option<String>,
    pub profile: Option<String>,
    pub del: Option<i32>,
    pub created_time: Option<DateTimeNative>,
    pub updated_time: Option<DateTimeNative>,
}

impl_field_name_method!(About{id, title, profile, del, created_time, updated_time});


#[crud_table]
#[derive(Clone, Debug)]
pub struct Section {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub del: Option<i32>,
    pub created_time: Option<DateTimeNative>,
    pub updated_time: Option<DateTimeNative>,
}

impl_field_name_method!(Section{id, name, description, del, created_time, updated_time});


#[crud_table]
#[derive(Clone, Debug)]
pub struct FriendLink {
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub del: Option<i32>,
    pub created_time: Option<DateTimeNative>,
    pub updated_time: Option<DateTimeNative>,
}
