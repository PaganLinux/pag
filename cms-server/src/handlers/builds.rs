use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::auth::AuthExtractor;
use crate::build_queue::BuildTask;
use crate::db;
use crate::models::*;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListQuery {
    pub status: Option<String>,
}

pub async fn list_builds(
    AuthExtractor(_user): AuthExtractor,
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let builds = db::get_build_jobs(&state.pool, query.status.as_deref())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    Ok(Json(serde_json::to_value(builds).unwrap()))
}

pub async fn get_build(
    AuthExtractor(_user): AuthExtractor,
    State(state): State<AppState>,
    Path(uuid): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let build = db::get_build_job_by_uuid(&state.pool, &uuid)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    match build {
        Some(b) => Ok(Json(serde_json::to_value(b).unwrap())),
        None => Err((StatusCode::NOT_FOUND, Json(json!({"error": "Not found"})))),
    }
}

pub async fn create_build(
    AuthExtractor(_user): AuthExtractor,
    State(state): State<AppState>,
    Json(req): Json<CreateBuildRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let job_uuid = uuid::Uuid::new_v4().to_string();
    let task = BuildTask {
        job_uuid: job_uuid.clone(),
        submission_id: req.submission_id,
        package_name: req.package_name,
        package_version: req.package_version,
        build_script: req.build_script,
    };
    state.build_queue.enqueue(task).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;
    Ok(Json(json!({"message": "Build enqueued", "job_uuid": job_uuid})))
}

pub async fn approve_and_build(
    AuthExtractor(_user): AuthExtractor,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let sub = db::get_submission_by_id(&state.pool, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"}))))?;

    db::update_submission_status(&state.pool, id, "approved")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let job_uuid = uuid::Uuid::new_v4().to_string();
    let task = BuildTask {
        job_uuid: job_uuid.clone(),
        submission_id: Some(id),
        package_name: sub.package_name,
        package_version: sub.package_version,
        build_script: sub.build_script,
    };
    state.build_queue.enqueue(task).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;
    Ok(Json(json!({"message": "Approved and enqueued", "job_uuid": job_uuid})))
}

pub async fn cancel_build(
    AuthExtractor(_user): AuthExtractor,
    State(state): State<AppState>,
    Path(uuid): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    db::update_build_job_status(&state.pool, &uuid, "cancelled")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    Ok(Json(json!({"message": "Cancelled"})))
}
