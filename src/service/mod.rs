use rbatis::rbatis::Rbatis;
pub use crate::config::app_config::ApplicationConfig;

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rb: Rbatis,
}


impl ServiceContext {
    pub async fn new(config_path: &str) -> Self {
        let config = ApplicationConfig::new(config_path);
        let rb = crate::dao::init_rbatis(&config).await;
        ServiceContext {
            config,
            rb,
        }
    }
}
