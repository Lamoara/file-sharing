use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::hasher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkConfig {
    pub hashed_password: Option<String>,
    pub expiration_date: Option<DateTime<Utc>>,
}

impl LinkConfig {
    pub fn new(password: Option<String>, expiration_date: Option<DateTime<Utc>>) -> Self {
        let hashed_password = password.and_then(|pwd| hasher::hash(&pwd).ok());
        Self {
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
