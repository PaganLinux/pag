use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Package {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub release: String,
    pub description: Option<String>,
    pub arch: String,
    pub maintainer_id: Option<i64>,
    pub build_status: String,
    pub pkg_url: Option<String>,
    pub pkg_size: Option<i64>,
    pub checksum_sha: Option<String>,
    pub checksum_blake3: Option<String>,
    pub gpg_signature: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePackageRequest {
    pub name: String,
    pub version: String,
    pub release: Option<String>,
    pub description: Option<String>,
    pub arch: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePackageRequest {
    pub version: Option<String>,
    pub release: Option<String>,
    pub description: Option<String>,
    pub build_status: Option<String>,
    pub pkg_url: Option<String>,
    pub pkg_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PackageFilter {
    pub arch: Option<String>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PackageListResponse {
    pub packages: Vec<Package>,
    pub total: i64,
    pub page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize)]
pub struct UploadPackageRequest {
    pub name: String,
    pub version: String,
    pub release: Option<String>,
    pub description: Option<String>,
    pub arch: Option<String>,
    pub pagbuild: Option<String>, // zawartość pliku PAGBUILD
}
