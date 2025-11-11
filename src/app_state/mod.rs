use tokio::sync::RwLock;

use crate::app_state::{app_config::AppConfig, app_data::AppData};

pub(crate) mod app_config;
pub(crate) mod app_data;
pub(crate) mod link_config;

#[derive(Debug)]
pub struct AppState {
    pub config: RwLock<AppConfig>,
    pub data: RwLock<AppData>,
}

impl AppState {
    pub fn load() -> AppState {
        AppState {
            config: RwLock::new(AppConfig::load()),
            data: RwLock::new(AppData::load()),
        }
    }
}
