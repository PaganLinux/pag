mod auth;
mod build_queue;
mod config;
mod db;
mod handlers;
mod models;
mod state;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

use crate::build_queue::BuildQueue;
use crate::config::CmsConfig;
use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,pag_cms=debug".into()),
        )
        .init();

    tracing::info!("🚀 PaganLinux CMS Server starting...");

    let config = CmsConfig::load()?;
    tracing::info!("Config loaded, binding to {}:{}", config.server.bind, config.server.port);

    let pool = db::init_pool(&config.database.path).await?;
    tracing::info!("Database initialized at {}", config.database.path);

    let admin_exists = db::get_user_by_username(&pool, &config.auth.admin_username)
        .await?
        .is_some();

    if !admin_exists {
        let hash = auth::hash_password(
            &std::env::var("PAG_CMS_ADMIN_PASSWORD")
                .unwrap_or_else(|_| "admin123".into()),
        )?;
        db::create_user(&pool, &config.auth.admin_username, &hash, "admin").await?;
        tracing::info!("Default admin user created (change password immediately!)");
    }

    let build_queue = BuildQueue::new(pool.clone(), config.build.max_concurrent_builds);
    tracing::info!("Build queue initialized (max concurrent: {})", config.build.max_concurrent_builds);

    let state = AppState::from_config(pool, &config, build_queue);

    let app = Router::new()
        // Auth
        .route("/api/v1/auth/login", axum::routing::post(handlers::auth_handlers::login))
        .route("/api/v1/auth/logout", axum::routing::post(handlers::auth_handlers::logout))
        .route("/api/v1/auth/me", axum::routing::get(handlers::auth_handlers::me))
        // Dashboard
        .route("/api/v1/dashboard/stats", axum::routing::get(handlers::dashboard::get_stats))
        // Submissions
        .route("/api/v1/submissions", axum::routing::get(handlers::packages::list_submissions))
        .route("/api/v1/submissions/{id}", axum::routing::get(handlers::packages::get_submission).patch(handlers::packages::update_submission))
        .route("/api/v1/submissions/{id}/approve-build", axum::routing::post(handlers::builds::approve_and_build))
        // Builds
        .route("/api/v1/builds", axum::routing::get(handlers::builds::list_builds).post(handlers::builds::create_build))
        .route("/api/v1/builds/{uuid}", axum::routing::get(handlers::builds::get_build))
        .route("/api/v1/builds/{uuid}/cancel", axum::routing::post(handlers::builds::cancel_build))
        // WebSocket
        .route("/api/v1/builds/{uuid}/ws", axum::routing::get(handlers::ws::ws_handler))
        // Settings
        .route("/api/v1/settings", axum::routing::get(handlers::settings::get_settings).put(handlers::settings::update_settings))
        // Webhooks (public)
        .route("/api/v1/hooks/forgejo", axum::routing::post(handlers::webhooks::forgejo_webhook))
        // Health
        .route("/api/v1/health", get(health_check))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("{}:{}", config.server.bind, config.server.port);
    tracing::info!("CMS API listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(serde_json::json!({
        "status": "ok",
        "service": "pag-cms-server",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
