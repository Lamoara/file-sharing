use axum::{extract::Path, response::IntoResponse};
use serde::Deserialize;

use crate::extractors::user_auth_extractor::UserAuthSessionExtractor;

pub(crate) mod login;

#[derive(Debug, Deserialize)]
pub struct FilePath {
    file_url: String,
}

pub async fn load(_: UserAuthSessionExtractor, Path(path): Path<FilePath>) -> impl IntoResponse {}
