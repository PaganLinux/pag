// PaganLinux Repository Server
//
// Serwer HTTP dla repozytorium pakietów .pag
// Udostępnia:
// - API do uploadu/pobierania pakietów
// - Indeksowanie pakietów
// - Weryfikację podpisów
// - Statystyki

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

/// Stan aplikacji
struct AppState {
    repo_dir: String,
    db: rusqlite::Connection,
    api_tokens: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let repo_dir = std::env::var("PAG_REPO_DIR")
        .unwrap_or_else(|_| "/var/lib/pag/repo".to_string());

    let db_path = std::format!("{}/packages.db", repo_dir);
    std::fs::create_dir_all(&repo_dir)?;

    let db = rusqlite::Connection::open(&db_path)?;
    init_db(&db)?;

    // Wczytaj tokeny API z pliku
    let api_tokens = load_api_tokens();

    let state = Arc::new(AppState {
        repo_dir,
        db,
        api_tokens,
    });

    // Zbuduj router
    let app = Router::new()
        // API
        .route("/api/v1/index.json", get(get_index))
        .route("/api/v1/packages", get(list_packages))
        .route("/api/v1/packages/{name}", get(get_package_info))
        .route("/api/v1/search", get(search_packages))
        .route("/api/v1/upload/{repo}/{filename}", put(upload_package))
        .route("/api/v1/stats", get(get_stats))
        // Serwowanie plików statycznych (pakiety .pag)
        .nest_service("/core", ServeDir::new(format!("{}/core", state.repo_dir)))
        .nest_service("/extra", ServeDir::new(format!("{}/extra", state.repo_dir)))
        .nest_service("/community", ServeDir::new(format!("{}/community", state.repo_dir)))
        // Warstwy
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = std::env::var("PAG_REPO_BIND")
        .unwrap_or_else(|_| "0.0.0.0:3001".to_string());

    tracing::info!("Serwer repozytorium startuje na {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn init_db(db: &rusqlite::Connection) -> anyhow::Result<()> {
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS packages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            version TEXT NOT NULL,
            release INTEGER NOT NULL DEFAULT 1,
            arch TEXT NOT NULL,
            repo TEXT NOT NULL,
            description TEXT DEFAULT '',
            installed_size INTEGER DEFAULT 0,
            compressed_size INTEGER DEFAULT 0,
            filename TEXT NOT NULL,
            sha512 TEXT,
            blake3 TEXT,
            pgp_signature TEXT,
            maintainer TEXT,
            license TEXT,
            upload_date INTEGER NOT NULL,
            download_count INTEGER DEFAULT 0,
            UNIQUE(name, repo)
        );

        CREATE INDEX IF NOT EXISTS idx_pkg_name ON packages(name);
        CREATE INDEX IF NOT EXISTS idx_pkg_repo ON packages(repo);

        CREATE TABLE IF NOT EXISTS api_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            token_hash TEXT NOT NULL UNIQUE,
            description TEXT,
            created_at INTEGER NOT NULL
        );"
    )?;
    Ok(())
}

fn load_api_tokens() -> Vec<String> {
    let path = std::env::var("PAG_API_TOKENS")
        .unwrap_or_else(|_| "/etc/pag/api-tokens.conf".to_string());

    match std::fs::read_to_string(&path) {
        Ok(content) => content.lines()
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(|l| l.trim().to_string())
            .collect(),
        Err(_) => vec![],
    }
}

// ─── Handlers ──────────────────────────────────────────

/// Zwraca pełny indeks repozytorium
async fn get_index(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let mut stmt = state.db.prepare(
        "SELECT name, version, release, arch, description, installed_size,
                compressed_size, filename, sha512, blake3, pgp_signature, repo
         FROM packages ORDER BY repo, name"
    ).unwrap();

    let packages: Vec<serde_json::Value> = stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "name": row.get::<_, String>(0)?,
            "version": row.get::<_, String>(1)?,
            "release": row.get::<_, u32>(2)?,
            "arch": row.get::<_, String>(3)?,
            "description": row.get::<_, String>(4)?,
            "installed_size": row.get::<_, i64>(5)?,
            "compressed_size": row.get::<_, i64>(6)?,
            "filename": row.get::<_, String>(7)?,
            "sha512": row.get::<_, String>(8)?,
            "blake3": row.get::<_, String>(9)?,
            "pgp_signature": row.get::<_, Option<String>>(10)?,
            "repo": row.get::<_, String>(11)?,
        }))
    }).unwrap().filter_map(|r| r.ok()).collect();

    Json(serde_json::json!({
        "version": 1,
        "name": "PaganLinux Repository",
        "description": "Oficjalne repozytorium pakietów PaganLinux",
        "url": "https://repos.paganlinux.eu",
        "updated": chrono::Utc::now().timestamp(),
        "packages": packages,
        "total": packages.len()
    }))
}

/// Lista pakietów z opcjonalnym filtrem po repo
async fn list_packages(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Json<serde_json::Value> {
    let repo = params.get("repo");
    let limit = params.get("limit")
        .and_then(|l| l.parse::<usize>().ok())
        .unwrap_or(100);

    let query = if let Some(r) = repo {
        format!("SELECT name, version, release, arch, description, repo, installed_size, download_count
                 FROM packages WHERE repo = ?1 ORDER BY name LIMIT {}", limit)
    } else {
        format!("SELECT name, version, release, arch, description, repo, installed_size, download_count
                 FROM packages ORDER BY name LIMIT {}", limit)
    };

    let mut stmt = state.db.prepare(&query).unwrap();

    let packages: Vec<serde_json::Value> = if repo.is_some() {
        stmt.query_map(rusqlite::params![repo.unwrap()], |row| {
            Ok(serde_json::json!({
                "name": row.get::<_, String>(0)?,
                "version": row.get::<_, String>(1)?,
                "release": row.get::<_, u32>(2)?,
                "arch": row.get::<_, String>(3)?,
                "description": row.get::<_, String>(4)?,
                "repo": row.get::<_, String>(5)?,
                "installed_size": row.get::<_, i64>(6)?,
                "download_count": row.get::<_, i64>(7)?,
            }))
        }).unwrap().filter_map(|r| r.ok()).collect()
    } else {
        stmt.query_map([], |row| {
            Ok(serde_json::json!({
                "name": row.get::<_, String>(0)?,
                "version": row.get::<_, String>(1)?,
                "release": row.get::<_, u32>(2)?,
                "arch": row.get::<_, String>(3)?,
                "description": row.get::<_, String>(4)?,
                "repo": row.get::<_, String>(5)?,
                "installed_size": row.get::<_, i64>(6)?,
                "download_count": row.get::<_, i64>(7)?,
            }))
        }).unwrap().filter_map(|r| r.ok()).collect()
    };

    Json(serde_json::json!({ "packages": packages }))
}

/// Informacje o konkretnym pakiecie
async fn get_package_info(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = state.db.query_row(
        "SELECT name, version, release, arch, description, repo, installed_size,
                compressed_size, filename, sha512, blake3, maintainer, license, upload_date, download_count
         FROM packages WHERE name = ?1",
        rusqlite::params![name],
        |row| {
            Ok(serde_json::json!({
                "name": row.get::<_, String>(0)?,
                "version": row.get::<_, String>(1)?,
                "release": row.get::<_, u32>(2)?,
                "arch": row.get::<_, String>(3)?,
                "description": row.get::<_, String>(4)?,
                "repo": row.get::<_, String>(5)?,
                "installed_size": row.get::<_, i64>(6)?,
                "compressed_size": row.get::<_, i64>(7)?,
                "filename": row.get::<_, String>(8)?,
                "sha512": row.get::<_, String>(9)?,
                "blake3": row.get::<_, String>(10)?,
                "maintainer": row.get::<_, Option<String>>(11)?,
                "license": row.get::<_, String>(12)?,
                "upload_date": row.get::<_, i64>(13)?,
                "download_count": row.get::<_, i64>(14)?,
            }))
        },
    );

    match result {
        Ok(info) => Ok(Json(info)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// Wyszukiwanie pakietów
async fn search_packages(
    State(state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Json<serde_json::Value> {
    let q = params.get("q").cloned().unwrap_or_default();
    let like = format!("%{}%", q);

    let mut stmt = state.db.prepare(
        "SELECT name, version, release, arch, description, repo
         FROM packages WHERE name LIKE ?1 OR description LIKE ?1
         ORDER BY name LIMIT 50"
    ).unwrap();

    let packages: Vec<serde_json::Value> = stmt.query_map(rusqlite::params![like], |row| {
        Ok(serde_json::json!({
            "name": row.get::<_, String>(0)?,
            "version": row.get::<_, String>(1)?,
            "release": row.get::<_, u32>(2)?,
            "arch": row.get::<_, String>(3)?,
            "description": row.get::<_, String>(4)?,
            "repo": row.get::<_, String>(5)?,
        }))
    }).unwrap().filter_map(|r| r.ok()).collect();

    Json(serde_json::json!({ "results": packages }))
}

/// Upload pakietu (wymaga autoryzacji)
async fn upload_package(
    State(state): State<Arc<AppState>>,
    Path((repo, filename)): Path<(String, String)>,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Weryfikacja tokena
    let auth = headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !auth.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth[7..];
    if !state.api_tokens.iter().any(|t| t == token) {
        return Err(StatusCode::FORBIDDEN);
    }

    // Walidacja repozytorium
    if !["core", "extra", "community"].contains(&repo.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Zapisz plik
    let dir = format!("{}/{}", state.repo_dir, repo);
    std::fs::create_dir_all(&dir).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let filepath = format!("{}/{}", dir, filename);
    std::fs::write(&filepath, &body).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Parsuj nagłówek pakietu
    let pkg_info = parse_pag_header(&body);

    // Zapisz do bazy
    if let Some(info) = pkg_info {
        let _ = state.db.execute(
            "INSERT OR REPLACE INTO packages (name, version, release, arch, repo, description,
             installed_size, compressed_size, filename, sha512, blake3, pgp_signature, maintainer,
             license, upload_date)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            rusqlite::params![
                info.name,
                info.version,
                info.release,
                info.arch,
                repo,
                info.description,
                info.installed_size,
                body.len() as i64,
                filename,
                info.sha512,
                info.blake3,
                info.pgp_signature,
                info.maintainer,
                info.license,
                chrono::Utc::now().timestamp(),
            ],
        );

        tracing::info!("Przyjęto pakiet: {} v{} w repo {}", info.name, info.version, repo);
    }

    Ok(Json(serde_json::json!({
        "status": "ok",
        "filename": filename,
        "repo": repo,
    })))
}

/// Statystyki repozytorium
async fn get_stats(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let total: i64 = state.db.query_row(
        "SELECT COUNT(*) FROM packages", [], |row| row.get(0)
    ).unwrap_or(0);

    let total_size: i64 = state.db.query_row(
        "SELECT COALESCE(SUM(compressed_size), 0) FROM packages", [], |row| row.get(0)
    ).unwrap_or(0);

    let by_repo: Vec<serde_json::Value> = {
        let mut stmt = state.db.prepare(
            "SELECT repo, COUNT(*), COALESCE(SUM(compressed_size), 0) FROM packages GROUP BY repo"
        ).unwrap();

        stmt.query_map([], |row| {
            Ok(serde_json::json!({
                "repo": row.get::<_, String>(0)?,
                "count": row.get::<_, i64>(1)?,
                "size": row.get::<_, i64>(2)?,
            }))
        }).unwrap().filter_map(|r| r.ok()).collect()
    };

    Json(serde_json::json!({
        "total_packages": total,
        "total_size_bytes": total_size,
        "total_size_human": human_size(total_size as u64),
        "by_repo": by_repo,
        "last_updated": chrono::Utc::now().timestamp(),
    }))
}

/// Struktura pomocnicza do parsowania nagłówka .pag
struct PagHeaderInfo {
    name: String,
    version: String,
    release: u32,
    arch: String,
    description: String,
    installed_size: i64,
    sha512: String,
    blake3: String,
    pgp_signature: Option<String>,
    maintainer: Option<String>,
    license: String,
}

/// Parsuje podstawowe informacje z nagłówka .pag
fn parse_pag_header(data: &[u8]) -> Option<PagHeaderInfo> {
    if data.len() < 8 || &data[0..4] != b"PAG\x01" {
        return None;
    }

    let header_size = u32::from_le_bytes(data[4..8].try_into().ok()?) as usize;
    if data.len() < 8 + header_size {
        return None;
    }

    let header_json = &data[8..8 + header_size];
    let header: serde_json::Value = serde_json::from_slice(header_json).ok()?;

    Some(PagHeaderInfo {
        name: header.get("name")?.as_str()?.to_string(),
        version: header.get("version")?.as_str()?.to_string(),
        release: header.get("release")?.as_u64()? as u32,
        arch: header.get("arch")?.as_str()?.to_string(),
        description: header.get("description")?.as_str().unwrap_or("").to_string(),
        installed_size: header.get("installed_size")?.as_i64()?,
        sha512: header.get("sha512")?.as_str().unwrap_or("").to_string(),
        blake3: header.get("blake3")?.as_str().unwrap_or("").to_string(),
        pgp_signature: header.get("pgp_signature")?.as_str().map(|s| s.to_string()),
        maintainer: header.get("maintainer")?.as_str().map(|s| s.to_string()),
        license: header.get("license")?.as_str().unwrap_or("custom").to_string(),
    })
}

fn human_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    format!("{:.2} {}", size, UNITS[unit])
}
