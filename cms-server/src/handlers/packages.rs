use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::auth::AuthExtractor;
use crate::db;
use crate::models::*;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListQuery {
    pub status: Option<String>,
}

pub async fn list_submissions(
    AuthExtractor(_user): AuthExtractor,
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let submissions = db::get_submissions(&state.pool, query.status.as_deref())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    Ok(Json(serde_json::to_value(submissions).unwrap()))
}

pub async fn get_submission(
    AuthExtractor(_user): AuthExtractor,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let sub = db::get_submission_by_id(&state.pool, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    match sub {
        Some(s) => Ok(Json(serde_json::to_value(s).unwrap())),
        None => Err((StatusCode::NOT_FOUND, Json(json!({"error": "Not found"})))),
    }
}

pub async fn update_submission(
    AuthExtractor(_user): AuthExtractor,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateSubmissionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if let Some(status) = &req.status {
        db::update_submission_status(&state.pool, id, &status.to_string())
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    }
    if let Some(script) = &req.build_script {
        db::update_submission_script(&state.pool, id, script)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    }
    Ok(Json(json!({"message": "Updated"})))
}
