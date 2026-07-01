use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::json;

use crate::auth::{self, AuthExtractor};
use crate::db;
use crate::models::*;
use crate::state::AppState;

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user = db::get_user_by_username(&state.pool, &req.username)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "DB error"}))))?;

    match user {
        Some(u) if auth::verify_password(&req.password, &u.password_hash).unwrap_or(false) => {
            let token = auth::create_user_session(&state.pool, u.id, state.auth_config.token_expiry_days)
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Session error"}))))?;

            Ok(Json(json!({
                "token": token,
                "user": {"id": u.id, "username": u.username, "role": u.role}
            })))
        }
        _ => Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"})))),
    }
}

pub async fn logout(
    AuthExtractor(user): AuthExtractor,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM sessions WHERE user_id = ?")
        .bind(user.id)
        .execute(&state.pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "DB error"}))))?;

    Ok(Json(json!({"message": "Logged out"})))
}

pub async fn me(
    AuthExtractor(user): AuthExtractor,
) -> Json<serde_json::Value> {
    Json(json!({"id": user.id, "username": user.username, "role": user.role}))
}
