// Handlery repozytoriów
use axum::{extract::{Path, State}, http::StatusCode, Json};
use crate::config::Config;
use crate::db::DbPool;
use crate::models::*;
use crate::services::repos::RepoService;

pub async fn list_repos(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Repo>>, StatusCode> {
    RepoService::list(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_repo(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
) -> Result<Json<Repo>, StatusCode> {
    RepoService::get_by_id(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn create_repo(
    State(pool): State<DbPool>,
    Json(req): Json<CreateRepoRequest>,
) -> Result<(StatusCode, Json<Repo>), StatusCode> {
    let config = Config::from_env();
    RepoService::create(&pool, req, &config)
        .await
        .map(|r| (StatusCode::CREATED, Json(r)))
        .map_err(|e| {
            tracing::error!("Create repo error: {}", e);
            StatusCode::BAD_REQUEST
        })
}

pub async fn handle_webhook(
    State(pool): State<DbPool>,
    Json(payload): Json<WebhookPayload>,
) -> StatusCode {
    match RepoService::handle_webhook(&pool, &payload).await {
        Ok(()) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Webhook error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
