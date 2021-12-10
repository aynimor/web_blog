#[warn(unused_must_use)]
#[warn(unused_macros)]
#[warn(dead_code)]
// use log::info;
mod mapper;
mod service;
mod config;
mod dao;
mod domain;
mod error;
mod controller;
mod utils;
mod constants;

extern crate argon2;

use std::sync::Arc;
use std::env;
use actix_web::{web, HttpServer, App};
use log::{info, debug};
use crate::controller::user;
use crate::service::ServiceContext;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv().ok();
    let mut config_url = String::new();
    match env::var_os("BLOG_CONFIG_PATH") {
        Some(url) => {
            if let Some(url) = url.to_str() {
                config_url.push_str(url)
            } else {
                eprintln!("Not found config env: `BLOG_CONFIG_PATH`, You can run with 'export BLOG_CONFIG_PATH=$path' before app start, and makesure filepath exist!");
                return Ok(());
            }
        },
        None => {
            eprintln!("Not found config env: `BLOG_CONFIG_PATH`, You can run with 'export BLOG_CONFIG_PATH=$path' before app start!");
            return Ok(());
        }
    }
    let context = ServiceContext::new(config_url.as_str()).await;
    // 日志追加器
    config::log::init_log(&context);
    let data = web::Data::new(Arc::new(context));
    let server_url = data.config.server_url.clone();
    info!("Start HTTP server: {}", server_url);

    let app = move || {
        debug!("Constructing the App");

        App::new()
            .app_data(data.clone())
            .service(web::resource("/index").route(web::get().to(controller::ping)))
            .service(web::resource("/user").app_data(data.clone()).route(web::get().to(user::create_user)))
    };

    HttpServer::new(app).bind(server_url)?.run().await
}
