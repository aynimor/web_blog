pub mod user_service;

use std::sync::Arc;
use crate::config::app_config::ApplicationConfig;
use crate::service::user_service::UserService;

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub user_service: UserService,
}


impl ServiceContext {
    pub async fn new(config_path: &str) -> Self {
        let config = ApplicationConfig::new(config_path);
        let rb = crate::dao::init_rbatis(&config).await;
        let service_rb = Arc::new(rb);
        let user_service: UserService = UserService::new(service_rb.clone());

        ServiceContext {
            config,
            user_service,
        }
    }
}
