use std::sync::Arc;

use axum::{
    RequestPartsExt,
    extract::{FromRequestParts, Path},
    http::{StatusCode, request::Parts},
    response::Response,
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use uuid::Uuid;

use crate::app_state::AppState;

pub struct UserAuthSessionExtractor;

#[derive(Debug, Deserialize)]
struct Params {
    file_url: String,
}

impl FromRequestParts<Arc<AppState>> for UserAuthSessionExtractor {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let Path(params): Path<Params> = parts.extract::<Path<Params>>().await.unwrap();
        let current_link = params.file_url;

        let cookies = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| redirect_to_login(&current_link))?;

        let session_cookie = cookies
            .get("session")
            .ok_or_else(|| redirect_to_login(&current_link))?;

        let session_id = Uuid::parse_str(session_cookie.value())
            .map_err(|_| redirect_to_login(&current_link))?;

        let allowed_link = state
            .sessions
            .read()
            .await
            .verify_user(&session_id)
            .unwrap_or_default();

        if allowed_link == current_link {
            return Ok(UserAuthSessionExtractor);
        }

        if state
            .data
            .read()
            .await
            .try_access(&current_link, "")
            .is_ok()
        {
            return Ok(UserAuthSessionExtractor);
        }

        Err(redirect_to_login(&current_link))
    }
}

fn redirect_to_login(url: &str) -> Response {
    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header("Location", format!("{url}/login"))
        .body("".into())
        .unwrap()
}
