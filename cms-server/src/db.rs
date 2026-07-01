use sqlx::sqlite::SqlitePool;


use crate::models::*;

pub async fn init_pool(db_path: &str) -> anyhow::Result<SqlitePool> {
    if let Some(parent) = std::path::Path::new(db_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path)).await?;
    run_migrations(&pool).await?;
    seed_defaults(&pool).await?;
    Ok(pool)
}

async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS package_submissions (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            forgejo_pr_id   INTEGER NOT NULL,
            forgejo_pr_url  TEXT NOT NULL DEFAULT '',
            package_name    TEXT NOT NULL,
            package_version TEXT NOT NULL,
            description     TEXT NOT NULL DEFAULT '',
            submitter       TEXT NOT NULL DEFAULT 'unknown',
            build_script    TEXT NOT NULL DEFAULT '',
            status          TEXT NOT NULL DEFAULT 'pending',
            created_at      DATETIME NOT NULL DEFAULT (datetime('now')),
            updated_at      DATETIME NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS build_jobs (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            job_uuid        TEXT NOT NULL UNIQUE,
            submission_id   INTEGER REFERENCES package_submissions(id),
            package_name    TEXT NOT NULL,
            package_version TEXT NOT NULL,
            build_script    TEXT NOT NULL DEFAULT '',
            status          TEXT NOT NULL DEFAULT 'queued',
            log_output      TEXT NOT NULL DEFAULT '',
            started_at      DATETIME,
            finished_at     DATETIME,
            exit_code       INTEGER,
            artifact_path   TEXT,
            created_at      DATETIME NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS cms_users (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            username        TEXT NOT NULL UNIQUE,
            password_hash   TEXT NOT NULL,
            role            TEXT NOT NULL DEFAULT 'admin',
            created_at      DATETIME NOT NULL DEFAULT (datetime('now')),
            last_login      DATETIME
        );

        CREATE TABLE IF NOT EXISTS sessions (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            session_token   TEXT NOT NULL UNIQUE,
            user_id         INTEGER NOT NULL REFERENCES cms_users(id),
            created_at      DATETIME NOT NULL DEFAULT (datetime('now')),
            expires_at      DATETIME NOT NULL
        );

        CREATE TABLE IF NOT EXISTS cms_settings (
            key             TEXT PRIMARY KEY,
            value           TEXT NOT NULL DEFAULT '',
            updated_at      DATETIME NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS build_log_stream (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            job_uuid        TEXT NOT NULL,
            line_number     INTEGER NOT NULL,
            content         TEXT NOT NULL,
            timestamp       DATETIME NOT NULL DEFAULT (datetime('now'))
        );

        CREATE INDEX IF NOT EXISTS idx_submissions_status ON package_submissions(status);
        CREATE INDEX IF NOT EXISTS idx_build_jobs_status ON build_jobs(status);
        CREATE INDEX IF NOT EXISTS idx_build_log_job ON build_log_stream(job_uuid);
        CREATE INDEX IF NOT EXISTS idx_sessions_token ON sessions(session_token);
        CREATE INDEX IF NOT EXISTS idx_sessions_expires ON sessions(expires_at);
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn seed_defaults(pool: &SqlitePool) -> anyhow::Result<()> {
    // Seed default settings
    let defaults = vec![
        ("pagports_source", "github"),
        ("official_repo_url", "https://github.com/PaganLinux/pagports"),
        ("community_repo_url", "https://git.paganlinux.eu/pagan-community"),
        ("auto_approve_trusted", "false"),
        ("max_concurrent_builds", "2"),
        ("build_timeout_minutes", "120"),
        ("notify_on_build_complete", "true"),
        ("retention_days_build_logs", "30"),
    ];

    for (key, value) in defaults {
        sqlx::query(
            "INSERT OR IGNORE INTO cms_settings (key, value) VALUES (?, ?)",
        )
        .bind(key)
        .bind(value)
        .execute(pool)
        .await?;
    }

    Ok(())
}

// ─── Package Submissions ──────────────────────────────────

pub async fn get_submissions(pool: &SqlitePool, status: Option<&str>) -> anyhow::Result<Vec<PackageSubmission>> {
    let rows = match status {
        Some(s) => {
            sqlx::query_as::<_, PackageSubmission>(
                "SELECT * FROM package_submissions WHERE status = ? ORDER BY created_at DESC",
            )
            .bind(s)
            .fetch_all(pool)
            .await?
        }
        None => {
            sqlx::query_as::<_, PackageSubmission>(
                "SELECT * FROM package_submissions ORDER BY created_at DESC",
            )
            .fetch_all(pool)
            .await?
        }
    };
    Ok(rows)
}

pub async fn get_submission_by_id(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<PackageSubmission>> {
    let row = sqlx::query_as::<_, PackageSubmission>(
        "SELECT * FROM package_submissions WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn update_submission_status(
    pool: &SqlitePool,
    id: i64,
    status: &str,
) -> anyhow::Result<()> {
    sqlx::query(
        "UPDATE package_submissions SET status = ?, updated_at = datetime('now') WHERE id = ?",
    )
    .bind(status)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_submission_script(
    pool: &SqlitePool,
    id: i64,
    script: &str,
) -> anyhow::Result<()> {
    sqlx::query(
        "UPDATE package_submissions SET build_script = ?, updated_at = datetime('now') WHERE id = ?",
    )
    .bind(script)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn create_submission(pool: &SqlitePool, sub: &PackageSubmission) -> anyhow::Result<i64> {
    let id = sqlx::query(
        r#"INSERT INTO package_submissions
           (forgejo_pr_id, forgejo_pr_url, package_name, package_version, description, submitter, build_script, status)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(sub.forgejo_pr_id)
    .bind(&sub.forgejo_pr_url)
    .bind(&sub.package_name)
    .bind(&sub.package_version)
    .bind(&sub.description)
    .bind(&sub.submitter)
    .bind(&sub.build_script)
    .bind(sub.status.to_string())
    .execute(pool)
    .await?
    .last_insert_rowid();
    Ok(id)
}

// ─── Build Jobs ───────────────────────────────────────────

pub async fn create_build_job(pool: &SqlitePool, job: &BuildJob) -> anyhow::Result<i64> {
    let id = sqlx::query(
        r#"INSERT INTO build_jobs
           (job_uuid, submission_id, package_name, package_version, build_script, status)
           VALUES (?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&job.job_uuid)
    .bind(job.submission_id)
    .bind(&job.package_name)
    .bind(&job.package_version)
    .bind(&job.build_script)
    .bind(job.status.to_string())
    .execute(pool)
    .await?
    .last_insert_rowid();
    Ok(id)
}

pub async fn get_build_jobs(pool: &SqlitePool, status: Option<&str>) -> anyhow::Result<Vec<BuildJob>> {
    let rows = match status {
        Some(s) => {
            sqlx::query_as::<_, BuildJob>(
                "SELECT * FROM build_jobs WHERE status = ? ORDER BY created_at DESC",
            )
            .bind(s)
            .fetch_all(pool)
            .await?
        }
        None => {
            sqlx::query_as::<_, BuildJob>(
                "SELECT * FROM build_jobs ORDER BY created_at DESC LIMIT 100",
            )
            .fetch_all(pool)
            .await?
        }
    };
    Ok(rows)
}

pub async fn get_build_job_by_uuid(pool: &SqlitePool, uuid: &str) -> anyhow::Result<Option<BuildJob>> {
    let row = sqlx::query_as::<_, BuildJob>(
        "SELECT * FROM build_jobs WHERE job_uuid = ?",
    )
    .bind(uuid)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn update_build_job_status(
    pool: &SqlitePool,
    uuid: &str,
    status: &str,
) -> anyhow::Result<()> {
    sqlx::query(
        "UPDATE build_jobs SET status = ?, started_at = CASE WHEN ? IN ('running') THEN datetime('now') ELSE started_at END, finished_at = CASE WHEN ? IN ('completed', 'failed', 'cancelled') THEN datetime('now') ELSE finished_at END WHERE job_uuid = ?",
    )
    .bind(status)
    .bind(status)
    .bind(status)
    .bind(uuid)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn append_build_log(
    pool: &SqlitePool,
    uuid: &str,
    content: &str,
) -> anyhow::Result<()> {
    sqlx::query(
        "UPDATE build_jobs SET log_output = log_output || ? WHERE job_uuid = ?",
    )
    .bind(content)
    .bind(uuid)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn append_build_log_stream(
    pool: &SqlitePool,
    job_uuid: &str,
    line_number: i64,
    content: &str,
) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO build_log_stream (job_uuid, line_number, content) VALUES (?, ?, ?)",
    )
    .bind(job_uuid)
    .bind(line_number)
    .bind(content)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn finalize_build_job(
    pool: &SqlitePool,
    uuid: &str,
    exit_code: i32,
    artifact_path: Option<&str>,
) -> anyhow::Result<()> {
    let status = if exit_code == 0 { "completed" } else { "failed" };
    sqlx::query(
        "UPDATE build_jobs SET status = ?, exit_code = ?, artifact_path = ?, finished_at = datetime('now') WHERE job_uuid = ?",
    )
    .bind(status)
    .bind(exit_code)
    .bind(artifact_path)
    .bind(uuid)
    .execute(pool)
    .await?;
    Ok(())
}

// ─── Auth ─────────────────────────────────────────────────

pub async fn get_user_by_username(pool: &SqlitePool, username: &str) -> anyhow::Result<Option<CmsUser>> {
    let row = sqlx::query_as::<_, CmsUser>(
        "SELECT * FROM cms_users WHERE username = ?",
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn create_user(
    pool: &SqlitePool,
    username: &str,
    password_hash: &str,
    role: &str,
) -> anyhow::Result<i64> {
    let id = sqlx::query(
        "INSERT INTO cms_users (username, password_hash, role) VALUES (?, ?, ?)",
    )
    .bind(username)
    .bind(password_hash)
    .bind(role)
    .execute(pool)
    .await?
    .last_insert_rowid();
    Ok(id)
}

pub async fn create_session(
    pool: &SqlitePool,
    token: &str,
    user_id: i64,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO sessions (session_token, user_id, expires_at) VALUES (?, ?, ?)",
    )
    .bind(token)
    .bind(user_id)
    .bind(expires_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_session(pool: &SqlitePool, token: &str) -> anyhow::Result<Option<Session>> {
    let row = sqlx::query_as::<_, Session>(
        "SELECT * FROM sessions WHERE session_token = ? AND expires_at > datetime('now')",
    )
    .bind(token)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn delete_session(pool: &SqlitePool, token: &str) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM sessions WHERE session_token = ?")
        .bind(token)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_last_login(pool: &SqlitePool, user_id: i64) -> anyhow::Result<()> {
    sqlx::query("UPDATE cms_users SET last_login = datetime('now') WHERE id = ?")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

// ─── Settings ─────────────────────────────────────────────

pub async fn get_all_settings(pool: &SqlitePool) -> anyhow::Result<Vec<CmsSetting>> {
    let rows = sqlx::query_as::<_, CmsSetting>(
        "SELECT * FROM cms_settings ORDER BY key",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_setting(pool: &SqlitePool, key: &str) -> anyhow::Result<Option<String>> {
    let row = sqlx::query_as::<_, CmsSetting>(
        "SELECT * FROM cms_settings WHERE key = ?",
    )
    .bind(key)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.value))
}

pub async fn update_setting(pool: &SqlitePool, key: &str, value: &str) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO cms_settings (key, value, updated_at) VALUES (?, ?, datetime('now'))
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = datetime('now')",
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;
    Ok(())
}

// ─── Dashboard Stats ──────────────────────────────────────

pub async fn get_dashboard_stats(pool: &SqlitePool) -> anyhow::Result<DashboardStats> {
    let total_packages: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM package_submissions WHERE status = 'published'")
            .fetch_one(pool)
            .await?;

    let pending: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM package_submissions WHERE status IN ('pending', 'under_review')")
            .fetch_one(pool)
            .await?;

    let active_builds: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM build_jobs WHERE status IN ('queued', 'running')")
            .fetch_one(pool)
            .await?;

    let completed_today: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM build_jobs WHERE status = 'completed' AND date(finished_at) = date('now')")
            .fetch_one(pool)
            .await?;

    let failed_today: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM build_jobs WHERE status = 'failed' AND date(finished_at) = date('now')")
            .fetch_one(pool)
            .await?;

    let published: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM package_submissions WHERE status = 'published'")
            .fetch_one(pool)
            .await?;

    Ok(DashboardStats {
        total_packages: total_packages.0,
        pending_submissions: pending.0,
        active_builds: active_builds.0,
        completed_builds_today: completed_today.0,
        failed_builds_today: failed_today.0,
        published_packages: published.0,
        disk_usage_mb: 0, // TODO: implement
    })
}

// ─── Cleanup ──────────────────────────────────────────────

pub async fn cleanup_expired_sessions(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM sessions WHERE expires_at < datetime('now')")
        .execute(pool)
        .await?;
    Ok(())
}
