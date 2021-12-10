use rbatis::{html_sql, executor::RbatisExecutor, rb_html, push_index};
use crate::domain::User;


#[html_sql("mapper/user.html")]
async fn find_by_account(rb: &mut RbatisExecutor<'_,'_>, account: &str) -> User { todo!() }


#[html_sql("mapper/user.html")]
async fn create_user(rb: &mut RbatisExecutor<'_,'_>, user: &User) -> i64 { todo!() }
