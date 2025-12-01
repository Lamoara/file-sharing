use std::fs;

use axum::extract::Multipart;

pub async fn upload_fn(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        fs::write("file_test", data).unwrap();
    }
}
