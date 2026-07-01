use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ─── Package Submission (from Forgejo/community) ──────────

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PackageSubmission {
    pub id: i64,
    pub forgejo_pr_id: i64,
    pub forgejo_pr_url: String,
    pub package_name: String,
    pub package_version: String,
    pub description: String,
    pub submitter: String,
    pub build_script: String,
    pub status: SubmissionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(rename_all = "lowercase")]
pub enum SubmissionStatus {
    Pending,
    UnderReview,
    Approved,
    Rejected,
    Building,
    Built,
    Failed,
    Published,
}

impl std::fmt::Display for SubmissionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::UnderReview => write!(f, "under_review"),
            Self::Approved => write!(f, "approved"),
            Self::Rejected => write!(f, "rejected"),
            Self::Building => write!(f, "building"),
            Self::Built => write!(f, "built"),
            Self::Failed => write!(f, "failed"),
            Self::Published => write!(f, "published"),
        }
    }
}

// ─── Build Job ────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BuildJob {
    pub id: i64,
    pub job_uuid: String,
    pub submission_id: Option<i64>,
    pub package_name: String,
    pub package_version: String,
    pub build_script: String,
    pub status: BuildStatus,
    pub log_output: String,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub exit_code: Option<i32>,
    pub artifact_path: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(rename_all = "lowercase")]
pub enum BuildStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl std::fmt::Display for BuildStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Queued => write!(f, "queued"),
            Self::Running => write!(f, "running"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

// ─── CMS User ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsUser {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Moderator,
    Viewer,
}

// ─── Session ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: i64,
    pub session_token: String,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

// ─── CMS Settings ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsSetting {
    pub key: String,
    pub value: String,
    pub updated_at: DateTime<Utc>,
}

// ─── Dashboard Stats ──────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_packages: i64,
    pub pending_submissions: i64,
    pub active_builds: i64,
    pub completed_builds_today: i64,
    pub failed_builds_today: i64,
    pub published_packages: i64,
    pub disk_usage_mb: u64,
}

// ─── API Request/Response types ───────────────────────────

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub role: UserRole,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubmissionRequest {
    pub status: Option<SubmissionStatus>,
    pub build_script: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBuildRequest {
    pub package_name: String,
    pub package_version: String,
    pub build_script: String,
    pub submission_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsRequest {
    pub settings: Vec<SettingEntry>,
}

#[derive(Debug, Deserialize)]
pub struct SettingEntry {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgejoWebhookPayload {
    pub action: Option<String>,
    pub number: Option<i64>,
    pub pull_request: Option<ForgejoPR>,
    pub repository: Option<ForgejoRepo>,
}

#[derive(Debug, Deserialize)]
pub struct ForgejoPR {
    pub id: i64,
    pub title: Option<String>,
    pub body: Option<String>,
    pub html_url: Option<String>,
    pub user: Option<ForgejoUser>,
    pub head: Option<ForgejoBranch>,
}

#[derive(Debug, Deserialize)]
pub struct ForgejoBranch {
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub sha: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgejoUser {
    pub login: String,
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ForgejoRepo {
    pub full_name: String,
    pub name: String,
}
