use std::sync::Arc;

use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::get,
};
use dotenvy::var;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::app_state::AppState;

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
        .route("/", get("Hello admin"))
        .layer(admin_cors_layer);

    let users_router = Router::new()
        .route("/", get("Hello user"))
        .layer(users_cors_layer);

    Ok(Router::new()
        .nest("/admin", admin_router)
        .nest("/users", users_router)
        .layer(trace_layer)
        .with_state(app_state))
}
