use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Json},
};
use serde_json::json;
use sqlx::SqlitePool;

use crate::db;
use crate::models::*;

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Hash: {}", e))?
        .to_string())
}

pub fn verify_password(password: &str, hash: &str) -> anyhow::Result<bool> {
    let parsed = PasswordHash::new(hash).map_err(|e| anyhow::anyhow!("Parse: {}", e))?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
}

pub fn generate_session_token() -> String {
    hex::encode(rand::random::<[u8; 32]>())
}

pub async fn create_user_session(
    pool: &SqlitePool,
    user_id: i64,
    expiry_days: u32,
) -> anyhow::Result<String> {
    let token = generate_session_token();
    let expires_at = chrono::Utc::now() + chrono::Duration::days(expiry_days as i64);
    db::create_session(pool, &token, user_id, expires_at).await?;
    db::update_last_login(pool, user_id).await?;
    Ok(token)
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: i64,
    pub username: String,
    pub role: UserRole,
}

// Simple extractor that reads from request extensions
// The actual auth logic is done via middleware or explicit token check
#[derive(Debug, Clone)]
pub struct AuthExtractor(pub AuthUser);

impl<S> FromRequestParts<S> for AuthExtractor
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Try to get user from request extensions (set by middleware)
        if let Some(user) = parts.extensions.get::<AuthUser>() {
            return Ok(AuthExtractor(user.clone()));
        }

        // Fallback: extract token and validate directly
        let pool = parts.extensions.get::<SqlitePool>()
            .ok_or(AuthError::Internal)?;

        let token = extract_token(parts)?;
        validate_token(pool, &token).await
    }
}

fn extract_token(parts: &Parts) -> Result<String, AuthError> {
    if let Some(auth) = parts.headers.get("Authorization") {
        let auth_str = auth.to_str().map_err(|_| AuthError::InvalidToken)?;
        return Ok(auth_str.strip_prefix("Bearer ").unwrap_or(auth_str).to_string());
    }
    if let Some(cookie) = parts.headers.get("Cookie") {
        let cookie_str = cookie.to_str().map_err(|_| AuthError::InvalidToken)?;
        for part in cookie_str.split(';') {
            if let Some((key, value)) = part.trim().split_once('=') {
                if key.trim() == "pagan_cms_session" {
                    return Ok(value.trim().to_string());
                }
            }
        }
    }
    Err(AuthError::NoToken)
}

async fn validate_token(pool: &SqlitePool, token: &str) -> Result<AuthExtractor, AuthError> {
    let session = db::get_session(pool, token)
        .await
        .map_err(|_| AuthError::Internal)?;

    match session {
        Some(s) => {
            let user = sqlx::query_as::<_, CmsUser>("SELECT * FROM cms_users WHERE id = ?")
                .bind(s.user_id)
                .fetch_optional(pool)
                .await
                .map_err(|_| AuthError::Internal)?
                .ok_or(AuthError::InvalidToken)?;

            Ok(AuthExtractor(AuthUser {
                id: user.id,
                username: user.username,
                role: user.role,
            }))
        }
        None => Err(AuthError::ExpiredToken),
    }
}

#[derive(Debug)]
pub enum AuthError {
    NoToken,
    InvalidToken,
    ExpiredToken,
    Internal,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            AuthError::NoToken | AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Session expired"),
            AuthError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}
