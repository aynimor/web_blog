#[warn(unused_must_use)]
#[warn(unused_macros)]
#[warn(dead_code)]
// use log::info;
mod service;
mod config;
mod dao;
mod domain;
mod error;
mod controller;

use std::sync::Arc;
use actix_web::{web, HttpServer, App};
use log::info;
use crate::controller::user;
use crate::service::ServiceContext;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = ServiceContext::new("/home/aynimor/program/web_blog/application.yml").await;
    //日志追加器
    config::log::init_log(&context)?;
    let data = web::Data::new(Arc::new(context));
    let server_url = data.config.server_url.clone();
    info!("Start HTTP server: {}", server_url);
    HttpServer::new(
        move || 
            App::new()
            .service(web::resource("/index").route(web::get().to(controller::index)))
            .service(web::resource("/user").app_data(data.clone()).route(web::get().to(user::create_user)))
    ).bind(server_url)?
    .run()
    .await?;
    Ok(())
}
