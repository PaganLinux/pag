// Handlery portów
use axum::{extract::{Path, State}, http::StatusCode, Extension, Json};
use crate::db::DbPool;
use crate::models::*;
use crate::services::ports::PortService;

pub async fn list_ports(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Port>>, StatusCode> {
    PortService::list(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_port(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
) -> Result<Json<Port>, StatusCode> {
    PortService::get_by_id(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn create_port(
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreatePortRequest>,
) -> Result<(StatusCode, Json<Port>), StatusCode> {
    PortService::create(&pool, req, claims.sub)
        .await
        .map(|p| (StatusCode::CREATED, Json(p)))
        .map_err(|e| {
            tracing::error!("Create port error: {}", e);
            StatusCode::BAD_REQUEST
        })
}

pub async fn update_port(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
    Json(req): Json<UpdatePortRequest>,
) -> Result<Json<Port>, StatusCode> {
    PortService::update(&pool, id, req)
        .await
        .map(Json)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

pub async fn delete_port(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    PortService::delete(&pool, id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
