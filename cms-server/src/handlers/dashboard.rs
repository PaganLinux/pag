use axum::{extract::State, response::Json};
use serde_json::json;

use crate::auth::AuthExtractor;
use crate::db;
use crate::state::AppState;

pub async fn get_stats(
    AuthExtractor(_user): AuthExtractor,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let stats = db::get_dashboard_stats(&state.pool).await.map_err(|e| {
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;
    Ok(Json(serde_json::to_value(stats).unwrap()))
}
