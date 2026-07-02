use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Build {
    pub id: i64,
    pub package_id: i64,
    pub job_id: String,
    pub status: String,
    pub arch: String,
    pub log_path: Option<String>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBuildRequest {
    pub package_id: i64,
    pub arch: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BuildListResponse {
    pub builds: Vec<BuildWithPackage>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BuildWithPackage {
    pub id: i64,
    pub package_id: i64,
    pub job_id: String,
    pub status: String,
    pub arch: String,
    pub log_path: Option<String>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub created_at: String,
    pub package_name: String,
    pub package_version: String,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub total_packages: i64,
    pub total_ports: i64,
    pub total_builds: i64,
    pub builds_today: i64,
    pub successful_builds: i64,
    pub failed_builds: i64,
}
