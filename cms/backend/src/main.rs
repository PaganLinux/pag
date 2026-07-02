// PaganCMS — Backend API
// Centralny serwer zarządzania dla paganlinux.eu
mod config;
mod db;
mod models;
mod services;
mod middleware;
mod handlers;

use axum::{
    middleware as axum_middleware,
    routing::{get, post, put, delete},
    Router,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicjalizacja loggera
    tracing_subscriber::fmt::init();

    // Wczytaj konfigurację
    let config = config::Config::from_env();

    // Inicjalizacja bazy danych
    let pool = db::init_db(&config).await?;
    tracing::info!("Database initialized");

    // Warstwa CORS
    let cors = middleware::cors::cors_layer();

    // ─── Publiczne endpointy (bez auth) ────────────────
    let public_routes = Router::new()
        .route("/api/v1/auth/register", post(handlers::auth::register))
        .route("/api/v1/auth/login", post(handlers::auth::login))
        .route("/api/v1/stats", get(handlers::builds::stats))
        .route("/api/v1/webhook/gitea", post(handlers::repos::handle_webhook))
        // Publiczne GET-y
        .route("/api/v1/packages", get(handlers::packages::list_packages))
        .route("/api/v1/packages/{id}", get(handlers::packages::get_package))
        .route("/api/v1/builds", get(handlers::builds::list_builds))
        .route("/api/v1/builds/{id}", get(handlers::builds::get_build))
        .route("/api/v1/builds/package/{package_id}", get(handlers::builds::list_builds_for_package))
        .route("/api/v1/ports", get(handlers::ports::list_ports))
        .route("/api/v1/ports/{id}", get(handlers::ports::get_port))
        .route("/api/v1/repos", get(handlers::repos::list_repos))
        .route("/api/v1/repos/{id}", get(handlers::repos::get_repo))
        .with_state(pool.clone());

    // ─── Chronione endpointy (JWT) ─────────────────────
    let protected_routes = Router::new()
        .route("/api/v1/auth/me", get(handlers::auth::me))
        .route("/api/v1/packages", post(handlers::packages::create_package))
        .route("/api/v1/packages/{id}", put(handlers::packages::update_package))
        .route("/api/v1/packages/upload", post(handlers::packages::upload_pagbuild))
        .route("/api/v1/builds", post(handlers::builds::create_build))
        .route("/api/v1/builds/{id}/status", put(handlers::builds::update_build_status))
        .route("/api/v1/ports", post(handlers::ports::create_port))
        .route("/api/v1/ports/{id}", put(handlers::ports::update_port))
        .route("/api/v1/ports/{id}", delete(handlers::ports::delete_port))
        .route("/api/v1/repos", post(handlers::repos::create_repo))
        .layer(axum_middleware::from_fn(middleware::auth::auth_middleware))
        .with_state(pool.clone());

    // Połącz wszystko
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Start serwera
    let addr = config.server_addr.clone();
    tracing::info!("PaganCMS API starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
