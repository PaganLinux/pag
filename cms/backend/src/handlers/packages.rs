// Handlery pakietów
use axum::{extract::{Path, Query, State}, http::StatusCode, Extension, Json};
use crate::db::DbPool;
use crate::models::*;
use crate::services::packages::PackageService;

pub async fn list_packages(
    State(pool): State<DbPool>,
    Query(filter): Query<PackageFilter>,
) -> Result<Json<PackageListResponse>, StatusCode> {
    PackageService::list(&pool, filter)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("List packages error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn get_package(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
) -> Result<Json<Package>, StatusCode> {
    PackageService::get_by_id(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn create_package(
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreatePackageRequest>,
) -> Result<(StatusCode, Json<Package>), StatusCode> {
    PackageService::create(&pool, req, claims.sub)
        .await
        .map(|p| (StatusCode::CREATED, Json(p)))
        .map_err(|e| {
            tracing::error!("Create package error: {}", e);
            StatusCode::BAD_REQUEST
        })
}

pub async fn update_package(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
    Json(req): Json<UpdatePackageRequest>,
) -> Result<Json<Package>, StatusCode> {
    PackageService::update(&pool, id, req)
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Update package error: {}", e);
            StatusCode::BAD_REQUEST
        })
}

pub async fn upload_pagbuild(
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<UploadPackageRequest>,
) -> Result<(StatusCode, Json<Package>), StatusCode> {
    // Jeśli podano zawartość PAGBUILD, zapisz tymczasowo i sparsuj
    if let Some(ref pagbuild_content) = req.pagbuild {
        let tmp_path = format!("/tmp/pagbuild_{}.txt", uuid::Uuid::new_v4());
        std::fs::write(&tmp_path, pagbuild_content)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let result = PackageService::create_from_pagbuild(&pool, &tmp_path, claims.sub)
            .await
            .map(|p| (StatusCode::CREATED, Json(p)))
            .map_err(|e| {
                tracing::error!("Upload PAGBUILD error: {}", e);
                StatusCode::BAD_REQUEST
            });

        let _ = std::fs::remove_file(&tmp_path);
        return result;
    }

    // Inaczej — standardowe tworzenie
    create_package(
        State(pool),
        Extension(claims),
        Json(CreatePackageRequest {
            name: req.name,
            version: req.version,
            release: req.release,
            description: req.description,
            arch: req.arch,
        }),
    )
    .await
}
