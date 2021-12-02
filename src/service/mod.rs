

use rbatis::rbatis::Rbatis;
use rbatis::core::db::DBPoolOptions;
use crate::config::app_config::ApplicationConfig;

async fn init_rbatis(config: &ApplicationConfig) -> Rbatis {
    let rbatis = Rbatis::new();
    let opt = DBPoolOptions::new();
    rbatis.link_opt(config.database_url.as_str(), opt).await.unwrap();
    rbatis
}


pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rb: Rbatis,
}

impl ServiceContext {
    pub async fn new(config_path: &str) -> Self {
        let config = ApplicationConfig::new(config_path);
        let rb = init_rbatis(&config).await;
        ServiceContext {
            config,
            rb
        }
    }
}
