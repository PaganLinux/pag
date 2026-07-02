// Serwis portów — zarządzanie PAGBUILD
use crate::db::DbPool;
use crate::models::*;

pub struct PortService;

impl PortService {
    pub async fn list(pool: &DbPool) -> Result<Vec<Port>, anyhow::Error> {
        let ports = sqlx::query_as::<_, Port>(
            "SELECT * FROM ports ORDER BY name ASC",
        )
        .fetch_all(pool)
        .await?;
        Ok(ports)
    }

    pub async fn get_by_id(pool: &DbPool, id: i64) -> Result<Option<Port>, anyhow::Error> {
        let port = sqlx::query_as::<_, Port>("SELECT * FROM ports WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(port)
    }

    pub async fn create(
        pool: &DbPool,
        req: CreatePortRequest,
        maintainer_id: i64,
    ) -> Result<Port, anyhow::Error> {
        let port = sqlx::query_as::<_, Port>(
            r#"INSERT INTO ports (name, category, description, version, maintainer_id, pagbuild_path)
               VALUES (?, ?, ?, ?, ?, ?) RETURNING *"#,
        )
        .bind(&req.name)
        .bind(req.category.as_deref().unwrap_or(""))
        .bind(req.description.as_deref().unwrap_or(""))
        .bind(req.version.as_deref().unwrap_or(""))
        .bind(maintainer_id)
        .bind(&req.pagbuild_path)
        .fetch_one(pool)
        .await?;

        Ok(port)
    }

    pub async fn update(
        pool: &DbPool,
        id: i64,
        req: UpdatePortRequest,
    ) -> Result<Port, anyhow::Error> {
        let port = sqlx::query_as::<_, Port>(
            r#"UPDATE ports SET
                description = COALESCE(?, description),
                version = COALESCE(?, version),
                pagbuild_path = COALESCE(?, pagbuild_path),
                status = COALESCE(?, status),
                updated_at = datetime('now')
             WHERE id = ? RETURNING *"#,
        )
        .bind(req.description)
        .bind(req.version)
        .bind(req.pagbuild_path)
        .bind(req.status)
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(port)
    }

    pub async fn delete(pool: &DbPool, id: i64) -> Result<(), anyhow::Error> {
        sqlx::query("DELETE FROM port_packages WHERE port_id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        sqlx::query("DELETE FROM ports WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
