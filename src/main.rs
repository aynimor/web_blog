use log::info;
use salvo::prelude::*;

mod service;
mod config;
use crate::service::ServiceContext;

#[fn_handler]
async fn index(res: &mut Response) {
    info!("access index");
    res.render_plain_text("Hello World");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //日志追加器
    let context: ServiceContext = ServiceContext::new("/home/aynimor/test/rustlang/salvo_blog/application.yml").await;
    crate::config::log::init_log(&context)?;
    info!(" - Local:   http://{}", context.config.server_url);
    let router = Router::new()
        .push(Router::new().get(index));

    Server::new(TcpListener::bind(context.config.server_url.as_str()))
        .try_serve(router).await.unwrap();
    Ok(())
}
