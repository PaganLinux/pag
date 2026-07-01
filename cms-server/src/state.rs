use sqlx::SqlitePool;
use std::sync::Arc;

use crate::build_queue::BuildQueue;
use crate::config::{AuthConfig, CmsConfig, ForgejoConfig};

/// Unified application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub build_queue: Arc<BuildQueue>,
    pub auth_config: AuthConfig,
    pub forgejo_config: ForgejoConfig,
    pub build_config: crate::config::BuildConfig,
}

impl AppState {
    pub fn from_config(pool: SqlitePool, config: &CmsConfig, build_queue: Arc<BuildQueue>) -> Self {
        Self {
            pool,
            build_queue,
            auth_config: config.auth.clone(),
            forgejo_config: config.forgejo.clone(),
            build_config: config.build.clone(),
        }
    }
}
