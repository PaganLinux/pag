use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::json;

use crate::auth::AuthExtractor;
use crate::db;
use crate::models::*;
use crate::state::AppState;

pub async fn get_settings(
    AuthExtractor(_user): AuthExtractor,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let settings = db::get_all_settings(&state.pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;
    Ok(Json(serde_json::to_value(settings).unwrap()))
}

pub async fn update_settings(
    AuthExtractor(_user): AuthExtractor,
    State(state): State<AppState>,
    Json(req): Json<UpdateSettingsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    for entry in &req.settings {
        db::update_setting(&state.pool, &entry.key, &entry.value)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;
    }
    Ok(Json(json!({"message": "Settings updated"})))
}
