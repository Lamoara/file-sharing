use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    hashed_admin_username: String,
    hashed_password: String,
}

impl AppConfig {
    pub fn load() -> AppConfig {
        //TODO Load file

        AppConfig {
            hashed_admin_username: "".to_string(),
            hashed_password: "".to_string(),
        }
    }

    pub fn change_username() {}
    pub fn change_password() {}
    pub fn validate_credentials() {}
}
