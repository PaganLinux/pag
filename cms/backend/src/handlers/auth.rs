// Handlery autoryzacji
use axum::{extract::State, http::StatusCode, Extension, Json};
use crate::db::DbPool;
use crate::models::*;
use crate::services::auth::AuthService;

pub async fn register(
    State(pool): State<DbPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    AuthService::register(&pool, req)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Register error: {}", e);
            StatusCode::BAD_REQUEST
        })
}

pub async fn login(
    State(pool): State<DbPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    AuthService::login(&pool, req)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Login error: {}", e);
            StatusCode::UNAUTHORIZED
        })
}

pub async fn me(
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserPublic>, StatusCode> {
    let user = AuthService::get_user_by_id(&pool, claims.sub)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(UserPublic::from)
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}
