use std::sync::Arc;

use axum::{
    Router, extract::DefaultBodyLimit, http::{HeaderValue, Method, header}, routing::{get, post}
};
use dotenvy::var;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    app_state::AppState,
    routes::{
        admin::{
            self,
            dashboard::{create_download_link, create_upload_link, dashboard},
        },
        user::{self, download::download, load, upload::upload},
    },
};

pub fn app() -> anyhow::Result<Router> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let trace_layer = TraceLayer::new_for_http();

    let origins_admin: Vec<HeaderValue> = var("CORS_ORIGIN_ADMIN")
        .unwrap_or_default()
        .split(",")
        .map(|e| e.trim().parse::<HeaderValue>())
        .collect::<Result<_, _>>()?;

    let origins_users: Vec<HeaderValue> = var("CORS_ORIGIN_USERS")
        .unwrap_or_default()
        .split(",")
        .map(|e| e.trim().parse::<HeaderValue>())
        .collect::<Result<_, _>>()?;

    let admin_cors_layer = CorsLayer::new()
        .allow_origin(origins_admin)
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
        ]);

    let users_cors_layer = CorsLayer::new()
        .allow_origin(origins_users)
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
        ]);

    let app_state = Arc::new(AppState::load()?);

    let admin_router = Router::new()
        .route("/", get(admin::login::login_page))
        .route("/login", post(admin::login::login_form))
        .route("/dashboard", get(dashboard))
        .route("/dashboard/create/upload-link", post(create_upload_link))
        .route(
            "/dashboard/create/download-link",
            post(create_download_link),
        )
        .layer(admin_cors_layer);

    let request_limit = DefaultBodyLimit::disable(); //TODO Make this dynamic with link config

    let users_router = Router::new()
        .route("/{file_url}", get(load))
        .route(
            "/{file_url}/login",
            get(user::login::login_page).post(user::login::login_form),
        )
        .route("/{file_url}/upload", post(upload))
        .route("/{file_url}/download", get(download))
        .layer(request_limit)
        .layer(users_cors_layer);

    Ok(Router::new()
        .nest("/admin", admin_router)
        .nest("/user", users_router)
        .layer(trace_layer)
        .with_state(app_state))
}
