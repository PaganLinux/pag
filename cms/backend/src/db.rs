// Warstwa bazy danych — inicjalizacja i pool
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use crate::config::Config;

pub type DbPool = SqlitePool;

pub async fn init_db(config: &Config) -> anyhow::Result<DbPool> {
    // Wyciągnij ścieżkę pliku z URL sqlite:/sciezka/pliku.db
    let db_path = config.database_url.strip_prefix("sqlite:").unwrap_or(&config.database_url);
    if let Some(parent) = std::path::Path::new(db_path).parent() {
        std::fs::create_dir_all(parent)?;
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
    // Każdą tabelę tworzymy osobno — bezpieczniej niż split(';')
    let statements = vec![
        // Użytkownicy
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            role TEXT NOT NULL DEFAULT 'user' CHECK(role IN ('admin','maintainer','user')),
            avatar_url TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        // Pakiety .pag
        "CREATE TABLE IF NOT EXISTS packages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            version TEXT NOT NULL,
            release TEXT NOT NULL DEFAULT '1',
            description TEXT,
            arch TEXT NOT NULL DEFAULT 'x86_64',
            maintainer_id INTEGER REFERENCES users(id),
            build_status TEXT NOT NULL DEFAULT 'pending' CHECK(build_status IN ('pending','building','success','failed')),
            pkg_url TEXT,
            pkg_size INTEGER DEFAULT 0,
            checksum_sha TEXT,
            checksum_blake3 TEXT,
            gpg_signature TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            UNIQUE(name, version, arch)
        )",
        // Porty
        "CREATE TABLE IF NOT EXISTS ports (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            category TEXT,
            description TEXT,
            version TEXT,
            maintainer_id INTEGER REFERENCES users(id),
            pagbuild_path TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'active' CHECK(status IN ('active','outdated','broken','archived')),
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        // Buildy
        "CREATE TABLE IF NOT EXISTS builds (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            package_id INTEGER NOT NULL REFERENCES packages(id),
            job_id TEXT NOT NULL UNIQUE,
            status TEXT NOT NULL DEFAULT 'queued' CHECK(status IN ('queued','running','success','failed','cancelled')),
            arch TEXT NOT NULL DEFAULT 'x86_64',
            log_path TEXT,
            started_at TEXT,
            finished_at TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        // Port ↔ Package
        "CREATE TABLE IF NOT EXISTS port_packages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            port_id INTEGER NOT NULL REFERENCES ports(id),
            package_id INTEGER NOT NULL REFERENCES packages(id),
            UNIQUE(port_id, package_id)
        )",
        // Tokeny API
        "CREATE TABLE IF NOT EXISTS api_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL REFERENCES users(id),
            token TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            last_used TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            expires_at TEXT
        )",
        // Repozytoria Gitea
        "CREATE TABLE IF NOT EXISTS repos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            full_name TEXT NOT NULL,
            owner TEXT NOT NULL,
            description TEXT,
            gitea_id INTEGER,
            clone_url TEXT,
            webhook_url TEXT,
            active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        // Sesje
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL REFERENCES users(id),
            refresh_token TEXT NOT NULL UNIQUE,
            expires_at TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        // Indeksy
        "CREATE INDEX IF NOT EXISTS idx_packages_name ON packages(name)",
        "CREATE INDEX IF NOT EXISTS idx_packages_status ON packages(build_status)",
        "CREATE INDEX IF NOT EXISTS idx_builds_status ON builds(status)",
        "CREATE INDEX IF NOT EXISTS idx_builds_package ON builds(package_id)",
        "CREATE INDEX IF NOT EXISTS idx_ports_name ON ports(name)",
    ];

    for stmt in &statements {
        sqlx::query(stmt).execute(pool).await?;
    }

    tracing::info!("Database migrations completed ({} tables)", statements.len());
    Ok(())
}
