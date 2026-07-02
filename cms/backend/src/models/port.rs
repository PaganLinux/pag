use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Port {
    pub id: i64,
    pub name: String,
    pub category: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub maintainer_id: Option<i64>,
    pub pagbuild_path: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePortRequest {
    pub name: String,
    pub category: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub pagbuild_path: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePortRequest {
    pub description: Option<String>,
    pub version: Option<String>,
    pub pagbuild_path: Option<String>,
    pub status: Option<String>,
}
