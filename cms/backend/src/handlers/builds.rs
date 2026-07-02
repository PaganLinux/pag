// Handlery buildów
use axum::{extract::{Path, State}, http::StatusCode, Extension, Json};
use crate::db::DbPool;
use crate::models::*;
use crate::services::builds::BuildService;

pub async fn list_builds(
    State(pool): State<DbPool>,
) -> Result<Json<BuildListResponse>, StatusCode> {
    BuildService::list(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_build(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
) -> Result<Json<Build>, StatusCode> {
    BuildService::get_by_id(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn list_builds_for_package(
    State(pool): State<DbPool>,
    Path(package_id): Path<i64>,
) -> Result<Json<Vec<Build>>, StatusCode> {
    BuildService::list_for_package(&pool, package_id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn create_build(
    State(pool): State<DbPool>,
    Extension(_claims): Extension<Claims>,
    Json(req): Json<CreateBuildRequest>,
) -> Result<(StatusCode, Json<Build>), StatusCode> {
    BuildService::create(&pool, req)
        .await
        .map(|b| (StatusCode::CREATED, Json(b)))
        .map_err(|e| {
            tracing::error!("Create build error: {}", e);
            StatusCode::BAD_REQUEST
        })
}

pub async fn update_build_status(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<Build>, StatusCode> {
    let status = body["status"].as_str().unwrap_or("running");
    let log_path = body["log_path"].as_str();

    BuildService::update_status(&pool, id, status, log_path)
        .await
        .map(Json)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

pub async fn stats(
    State(pool): State<DbPool>,
) -> Result<Json<StatsResponse>, StatusCode> {
    BuildService::stats(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
