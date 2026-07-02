use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Repo {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub owner: String,
    pub description: Option<String>,
    pub gitea_id: Option<i64>,
    pub clone_url: Option<String>,
    pub webhook_url: Option<String>,
    pub active: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRepoRequest {
    pub name: String,
    pub description: Option<String>,
    pub owner: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookPayload {
    pub r#ref: Option<String>,
    pub after: Option<String>,
    pub repository: Option<WebhookRepo>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookRepo {
    pub full_name: Option<String>,
    pub clone_url: Option<String>,
}
