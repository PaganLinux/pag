// Konfiguracja aplikacji
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub server_addr: String,
    pub gitea_api_url: String,
    pub gitea_token: String,
    pub cors_origin: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:data/pagancms.db?mode=rwc".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "paganlinux-cms-secret-change-in-production".to_string()),
            jwt_expiry_hours: env::var("JWT_EXPIRY_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
            server_addr: env::var("SERVER_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
            gitea_api_url: env::var("GITEA_API_URL")
                .unwrap_or_else(|_| "https://git.paganlinux.eu/api/v1".to_string()),
            gitea_token: env::var("GITEA_TOKEN")
                .unwrap_or_default(),
            cors_origin: env::var("CORS_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
        }
    }
}
