use std::{fs, sync::Arc};

use axum::{body::Body, extract::{Multipart, Path, State}, http::{HeaderMap, HeaderValue, StatusCode, header}, response::{IntoResponse, Response}};
use serde::Deserialize;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use crate::{app_state::AppState, extractors::user_auth_extractor::UserAuthSessionExtractor};

#[derive(Deserialize)]
pub struct FilePath {
    file_url: String
}

pub async fn download(
    _: UserAuthSessionExtractor,
    State(state): State<Arc<AppState>>,
    Path(path): Path<FilePath>,
) -> Response {
    let url = path.file_url;
    let app_data = state.data.read().await;
    let name = app_data.get_link_filename(&url).unwrap();
    let original_name = app_data.get_original_name(name).unwrap_or("downloadedFile");

    let file = match File::open(name).await {
        Ok(file) => file,
        Err(err) => return (StatusCode::NOT_FOUND, format!("File not found: {}", err)).into_response(),
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headers= HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/toml"));

    let content_disposition_value = HeaderValue::from_str(&format!("attachment; filename={original_name}")).unwrap();
    headers.insert(header::CONTENT_DISPOSITION, content_disposition_value);

    (headers, body).into_response()
}
