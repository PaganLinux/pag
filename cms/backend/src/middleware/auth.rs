// Middleware autoryzacji JWT
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use crate::config::Config;
use crate::services::auth::AuthService;

/// Ekstraktuje token z nagłówka Authorization
pub fn extract_token(req: &Request) -> Option<String> {
    req.headers()
        .get("Authorization")?
        .to_str()
        .ok()
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(String::from)
}

/// Middleware autoryzacji — wymaga poprawnego tokenu JWT
pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let config = Config::from_env();

    let token = extract_token(&req).ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = AuthService::validate_token(&config, &token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Wstrzyknij claims do request extensions
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

/// Middleware wymagający roli admin
pub async fn admin_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = req
        .extensions()
        .get::<crate::models::Claims>()
        .cloned()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
