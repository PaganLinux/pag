// Warstwa bazy danych — inicjalizacja i pool
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use crate::config::Config;

pub type DbPool = SqlitePool;

pub async fn init_db(config: &Config) -> anyhow::Result<DbPool> {
    // Upewnij się, że katalog data istnieje
    if let Some(parent) = std::path::Path::new("data").parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all("data");
        }
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    // Włącz WAL dla lepszej wydajności
    sqlx::query("PRAGMA journal_mode=WAL")
        .execute(&pool)
        .await?;
    sqlx::query("PRAGMA foreign_keys=ON")
        .execute(&pool)
        .await?;

    // Uruchom migracje
    run_migrations(&pool).await?;

    Ok(pool)
}

async fn run_migrations(pool: &DbPool) -> anyhow::Result<()> {
    let migration_sql = include_str!("../migrations/001_init.sql");

    // Podziel na pojedyncze statementy
    for statement in migration_sql.split(';') {
        let trimmed = statement.trim();
        if !trimmed.is_empty() && !trimmed.starts_with("--") {
            sqlx::query(trimmed)
                .execute(pool)
                .await?;
        }
    }

    tracing::info!("Database migrations completed");
    Ok(())
}
