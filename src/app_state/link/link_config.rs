use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::hasher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkConfig {
    pub file_name: String,
    pub hashed_password: Option<String>,
    pub expiration_date: Option<DateTime<Utc>>,
}

impl LinkConfig {
    pub fn new(file_name: Option<String>, password: Option<String>, expiration_date: Option<DateTime<Utc>>) -> Self {
        let hashed_password = password.and_then(|pwd| hasher::hash(&pwd).ok());
        let file = file_name.unwrap_or(Uuid::new_v4().to_string());
        Self {
            file_name: file,
            hashed_password,
            expiration_date,
        }
    }

    pub fn try_access(&self, psw: &str) -> Result<(), ()> {
        if let Some(hashed_password) = &self.hashed_password {
            match hasher::verify_hash(psw, hashed_password) {
                true => Ok(()),
                false => Err(()),
            }
        } else {
            Ok(())
        }
    }
}
