// use log::info;
mod service;
mod config;
mod dao;

use std::sync::Arc;

use actix_web::{web, HttpServer, App, Route};
use rbatis::{crud_table, impl_field_name_method, crud::CRUD};
use serde::{Deserialize, Serialize};

use service::ServiceContext;

#[crud_table]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct BizActivity {
    id: i32,
    name: Option<String>
}
impl_field_name_method!(BizActivity{id,name});


async fn index(ctx: web::Data<Arc<ServiceContext>>) -> String {
    let wrapper = ctx.rb.new_wrapper().eq(BizActivity::id(), &1);
    let bz: Result<Option<BizActivity>, rbatis::Error> = ctx.rb.fetch_by_wrapper(wrapper).await;
    println!("{:?}", bz);
    String::from(ctx.config.database_url.as_str())
}

async fn index2(ctx: web::Data<Arc<ServiceContext>>) -> String {
    String::from(ctx.config.database_url.as_str())
}


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = ServiceContext::new("/home/aynimor/test/rustlang/salvo_blog/application.yml").await;
    //日志追加器
    config::log::init_log(&context)?;
    let data = web::Data::new(Arc::new(context));
    HttpServer::new(
        move || 
            App::new()
            .service(web::resource("/hey").app_data(data.clone()).route(web::get().to(index)))
            .service(web::resource("/hey2").route(web::get().to(index2)))
    ).bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
