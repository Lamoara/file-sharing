use tokio::sync::RwLock;

use crate::app_state::{
    app_config::AppConfig,
    app_data::AppData,
    sessions::{Id, Sessions},
};

pub(crate) mod app_config;
pub(crate) mod app_data;
pub(crate) mod link_config;
pub(crate) mod sessions;

#[derive(Debug)]
pub struct AppState {
    pub sessions: RwLock<Sessions>,
    pub config: RwLock<AppConfig>,
    pub data: RwLock<AppData>,
}

impl AppState {
    pub fn load() -> anyhow::Result<AppState> {
        Ok(AppState {
            config: RwLock::new(AppConfig::load()?),
            data: RwLock::new(AppData::load()?),
            sessions: RwLock::new(Sessions::default()),
        })
    }
}

impl AppState {
    pub async fn try_login(&self, username: &str, password: &str) -> anyhow::Result<Id> {
        let config_lock = self.config.read().await;
        let verified = config_lock.validate_credentials(username, password);
        if verified {
            let mut sessions_lock = self.sessions.write().await;
            Ok(sessions_lock.login_admin())
        } else {
            Err(anyhow::Error::msg("Invalid credentials"))
        }
    }
}
