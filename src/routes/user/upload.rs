use std::{fs, sync::Arc};

use axum::extract::{Multipart, Path, State};
use serde::Deserialize;

use crate::{app_state::AppState, extractors::user_auth_extractor::UserAuthSessionExtractor};

#[derive(Deserialize)]
pub struct FilePath {
    file_url: String
}

pub async fn upload(
    _: UserAuthSessionExtractor,
    State(state): State<Arc<AppState>>,
    Path(path): Path<FilePath>,
    mut multipart: Multipart,
) {
    let url = path.file_url;
    let name;
    {
        let app_data = state.data.read().await;
        name = app_data.get_link_filename(&url).unwrap().to_string();
    }
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(original_name) = field.file_name() {
            state.data.write().await.save_file_original_name(&name, original_name);
        }
        let data = field.bytes().await.unwrap();
        fs::write(&name, data).unwrap();
    }

    println!("Finished uploading");
}
