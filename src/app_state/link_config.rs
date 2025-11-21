use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkConfig {
    file: String,
    description: Option<String>,
    hashed_password: Option<String>,
    limited_uses: Option<usize>,
    expiration_date: Option<DateTime<Utc>>,
    allow_download: bool,
    allow_upload: bool,
    max_size_mb: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LinkRoute (String);
