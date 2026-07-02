// Serwis buildów — kolejka i logi
use crate::db::DbPool;
use crate::models::*;
use uuid::Uuid;

pub struct BuildService;

impl BuildService {
    /// Lista buildów
    pub async fn list(pool: &DbPool) -> Result<BuildListResponse, anyhow::Error> {
        let builds = sqlx::query_as::<_, BuildWithPackage>(
            r#"SELECT b.*, p.name as package_name, p.version as package_version
               FROM builds b
               JOIN packages p ON p.id = b.package_id
               ORDER BY b.created_at DESC
               LIMIT 100"#,
        )
        .fetch_all(pool)
        .await?;

        let total = builds.len() as i64;

        Ok(BuildListResponse { builds, total })
    }

    /// Pobierz build po ID
    pub async fn get_by_id(pool: &DbPool, id: i64) -> Result<Option<Build>, anyhow::Error> {
        let build = sqlx::query_as::<_, Build>("SELECT * FROM builds WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(build)
    }

    /// Lista buildów dla danego pakietu
    pub async fn list_for_package(
        pool: &DbPool,
        package_id: i64,
    ) -> Result<Vec<Build>, anyhow::Error> {
        let builds = sqlx::query_as::<_, Build>(
            "SELECT * FROM builds WHERE package_id = ? ORDER BY created_at DESC LIMIT 50",
        )
        .bind(package_id)
        .fetch_all(pool)
        .await?;
        Ok(builds)
    }

    /// Utwórz nowe zadanie builda
    pub async fn create(
        pool: &DbPool,
        req: CreateBuildRequest,
    ) -> Result<Build, anyhow::Error> {
        let job_id = format!("build-{}", Uuid::new_v4());

        let build = sqlx::query_as::<_, Build>(
            r#"INSERT INTO builds (package_id, job_id, status, arch)
               VALUES (?, ?, 'queued', ?) RETURNING *"#,
        )
        .bind(req.package_id)
        .bind(&job_id)
        .bind(req.arch.as_deref().unwrap_or("x86_64"))
        .fetch_one(pool)
        .await?;

        // Aktualizuj status pakietu
        sqlx::query("UPDATE packages SET build_status = 'building', updated_at = datetime('now') WHERE id = ?")
            .bind(req.package_id)
            .execute(pool)
            .await?;

        Ok(build)
    }

    /// Aktualizuj status builda (wywoływane przez workera)
    pub async fn update_status(
        pool: &DbPool,
        id: i64,
        status: &str,
        log_path: Option<&str>,
    ) -> Result<Build, anyhow::Error> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let started_at = if status == "running" { Some(&now) } else { None };
        let finished_at = if status == "success" || status == "failed" { Some(&now) } else { None };

        let build = sqlx::query_as::<_, Build>(
            r#"UPDATE builds SET
                status = ?,
                log_path = COALESCE(?, log_path),
                started_at = COALESCE(?, started_at),
                finished_at = COALESCE(?, finished_at)
             WHERE id = ? RETURNING *"#,
        )
        .bind(status)
        .bind(log_path)
        .bind(started_at)
        .bind(finished_at)
        .bind(id)
        .fetch_one(pool)
        .await?;

        // Aktualizuj status pakietu
        let pkg_status = match status {
            "success" => "success",
            "failed" => "failed",
            _ => "building",
        };
        sqlx::query("UPDATE packages SET build_status = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(pkg_status)
            .bind(build.package_id)
            .execute(pool)
            .await?;

        Ok(build)
    }

    /// Statystyki
    pub async fn stats(pool: &DbPool) -> Result<StatsResponse, anyhow::Error> {
        let total_packages = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM packages")
            .fetch_one(pool)
            .await?;
        let total_ports = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM ports")
            .fetch_one(pool)
            .await?;
        let total_builds = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM builds")
            .fetch_one(pool)
            .await?;
        let builds_today = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM builds WHERE date(created_at) = date('now')",
        )
        .fetch_one(pool)
        .await?;
        let successful_builds = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM builds WHERE status = 'success'",
        )
        .fetch_one(pool)
        .await?;
        let failed_builds = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM builds WHERE status = 'failed'",
        )
        .fetch_one(pool)
        .await?;

        Ok(StatsResponse {
            total_packages,
            total_ports,
            total_builds,
            builds_today,
            successful_builds,
            failed_builds,
        })
    }
}
