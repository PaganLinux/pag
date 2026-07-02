// Serwis pakietów — CRUD + parsowanie PAGBUILD
use crate::db::DbPool;
use crate::models::*;

pub struct PackageService;

impl PackageService {
    /// Lista pakietów z filtrami i paginacją
    pub async fn list(
        pool: &DbPool,
        _filter: PackageFilter,
    ) -> Result<PackageListResponse, anyhow::Error> {
        let page = _filter.page.unwrap_or(1).max(1);
        let limit = _filter.limit.unwrap_or(20).min(100);
        let offset = (page - 1) * limit;

        // Pobierz wszystkie pakiety i paginuj w pamięci
        let packages = sqlx::query_as::<_, Package>("SELECT * FROM packages ORDER BY updated_at DESC")
            .fetch_all(pool)
            .await?;

        let total = packages.len() as i64;

        // Prosta paginacja w pamięci dla demo
        let packages: Vec<Package> = packages
            .into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect();

        let total_pages = (total as f64 / limit as f64).ceil() as i64;

        Ok(PackageListResponse {
            packages,
            total,
            page,
            total_pages: total_pages.max(1),
        })
    }

    /// Pobierz pakiet po ID
    pub async fn get_by_id(pool: &DbPool, id: i64) -> Result<Option<Package>, anyhow::Error> {
        let pkg = sqlx::query_as::<_, Package>("SELECT * FROM packages WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(pkg)
    }

    /// Utwórz nowy pakiet
    pub async fn create(
        pool: &DbPool,
        req: CreatePackageRequest,
        maintainer_id: i64,
    ) -> Result<Package, anyhow::Error> {
        let pkg = sqlx::query_as::<_, Package>(
            r#"INSERT INTO packages (name, version, release, description, arch, maintainer_id)
               VALUES (?, ?, ?, ?, ?, ?) RETURNING *"#,
        )
        .bind(&req.name)
        .bind(&req.version)
        .bind(req.release.as_deref().unwrap_or("1"))
        .bind(req.description.as_deref().unwrap_or(""))
        .bind(req.arch.as_deref().unwrap_or("x86_64"))
        .bind(maintainer_id)
        .fetch_one(pool)
        .await?;

        Ok(pkg)
    }

    /// Parsuj i utwórz pakiet z pliku PAGBUILD
    pub async fn create_from_pagbuild(
        pool: &DbPool,
        pagbuild_path: &str,
        maintainer_id: i64,
    ) -> Result<Package, anyhow::Error> {
        let content = std::fs::read_to_string(pagbuild_path)?;
        let info = Self::parse_pagbuild(&content)?;

        Self::create(
            pool,
            CreatePackageRequest {
                name: info.name,
                version: info.version,
                release: info.release,
                description: info.description,
                arch: info.arch,
            },
            maintainer_id,
        )
        .await
    }

    /// Aktualizuj pakiet
    pub async fn update(
        pool: &DbPool,
        id: i64,
        req: UpdatePackageRequest,
    ) -> Result<Package, anyhow::Error> {
        let pkg = sqlx::query_as::<_, Package>(
            r#"UPDATE packages SET
                version = COALESCE(?, version),
                release = COALESCE(?, release),
                description = COALESCE(?, description),
                build_status = COALESCE(?, build_status),
                pkg_url = COALESCE(?, pkg_url),
                pkg_size = COALESCE(?, pkg_size),
                updated_at = datetime('now')
             WHERE id = ? RETURNING *"#,
        )
        .bind(req.version)
        .bind(req.release)
        .bind(req.description)
        .bind(req.build_status)
        .bind(req.pkg_url)
        .bind(req.pkg_size)
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(pkg)
    }

    /// Parsuj plik PAGBUILD (uproszczona składnia PKGBUILD)
    fn parse_pagbuild(content: &str) -> Result<PagbuildInfo, anyhow::Error> {
        let mut info = PagbuildInfo::default();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(value) = Self::extract_var(line, "pkgname") {
                info.name = value;
            } else if let Some(value) = Self::extract_var(line, "pkgver") {
                info.version = value;
            } else if let Some(value) = Self::extract_var(line, "pkgrel") {
                info.release = Some(value);
            } else if let Some(value) = Self::extract_var(line, "pkgdesc") {
                info.description = Some(value);
            } else if let Some(value) = Self::extract_var(line, "arch") {
                info.arch = Some(value);
            }
        }

        if info.name.is_empty() || info.version.is_empty() {
            anyhow::bail!("PAGBUILD musi zawierać przynajmniej pkgname i pkgver");
        }

        Ok(info)
    }

    fn extract_var(line: &str, var: &str) -> Option<String> {
        let prefix = format!("{}=", var);
        if line.starts_with(&prefix) {
            let value = line[prefix.len()..].trim();
            // Usuń cudzysłowy
            let value = value.trim_matches('"').trim_matches('\'');
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
        None
    }
}

#[derive(Default)]
struct PagbuildInfo {
    name: String,
    version: String,
    release: Option<String>,
    description: Option<String>,
    arch: Option<String>,
}
