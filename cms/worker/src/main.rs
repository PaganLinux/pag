// PaganCMS Build Worker
// Odpytuje API CMS, pobiera zadania buildów z kolejki,
// wykonuje budowanie pakietów .pag i raportuje wyniki.

use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

const API_URL: &str = "http://localhost:3000/api/v1";
const POLL_INTERVAL_SECS: u64 = 5;
const BUILD_WORKSPACE: &str = "/tmp/pagbuild-workspace";

#[derive(Debug, Deserialize)]
struct BuildJob {
    id: i64,
    package_id: i64,
    job_id: String,
    status: String,
    arch: String,
}

#[derive(Debug, Deserialize)]
struct PackageInfo {
    id: i64,
    name: String,
    version: String,
    release: String,
    arch: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BuildListResponse {
    builds: Vec<BuildJob>,
}

#[derive(Debug, Serialize)]
struct BuildUpdate {
    status: String,
    log_path: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("PaganCMS Build Worker started");

    let client = reqwest::Client::new();
    let worker_id = Uuid::new_v4().to_string();
    tracing::info!("Worker ID: {}", worker_id);

    // Upewnij się że workspace istnieje
    if let Err(e) = tokio::fs::create_dir_all(BUILD_WORKSPACE).await {
        tracing::error!("Failed to create workspace: {}", e);
    }

    loop {
        match poll_and_process(&client).await {
            Ok(processed) => {
                if processed > 0 {
                    tracing::info!("Processed {} build(s)", processed);
                }
            }
            Err(e) => {
                tracing::error!("Worker error: {}", e);
            }
        }

        tokio::time::sleep(Duration::from_secs(POLL_INTERVAL_SECS)).await;
    }
}

async fn poll_and_process(client: &reqwest::Client) -> Result<usize, anyhow::Error> {
    // 1. Pobierz listę buildów
    let resp = client
        .get(format!("{}/builds", API_URL))
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("API error: {}", resp.status());
    }

    let data: BuildListResponse = resp.json().await?;

    // 2. Znajdź buildy w statusie "queued"
    let queued: Vec<_> = data
        .builds
        .into_iter()
        .filter(|b| b.status == "queued")
        .collect();

    if queued.is_empty() {
        return Ok(0);
    }

    let mut processed = 0;

    for build in &queued {
        tracing::info!("Processing build {} (job {})", build.id, build.job_id);

        // 3. Pobierz info o pakiecie
        let pkg: PackageInfo = match client
            .get(format!("{}/packages/{}", API_URL, build.package_id))
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => resp.json().await?,
            Ok(resp) => {
                tracing::error!("Package {} not found: {}", build.package_id, resp.status());
                update_build(client, build.id, "failed", Some("Package not found")).await?;
                continue;
            }
            Err(e) => {
                tracing::error!("Failed to fetch package: {}", e);
                continue;
            }
        };

        // 4. Ustaw status "running"
        update_build(client, build.id, "running", None).await?;

        // 5. Wykonaj build
        let build_dir = format!("{}/build-{}", BUILD_WORKSPACE, build.job_id);
        let log_file = format!("{}/build.log", build_dir);

        if let Err(e) = tokio::fs::create_dir_all(&build_dir).await {
            tracing::error!("Failed to create build dir: {}", e);
            update_build(client, build.id, "failed", Some(&log_file)).await?;
            continue;
        }

        // Symulacja builda (w produkcji: uruchomienie Docker/pagbuild)
        tracing::info!("Building {} v{} for {}", pkg.name, pkg.version, build.arch);
        let build_result = simulate_build(&pkg, &build.arch, &build_dir, &log_file).await;

        match build_result {
            Ok(()) => {
                tracing::info!("Build {} succeeded: {} v{}", build.id, pkg.name, pkg.version);
                update_build(client, build.id, "success", Some(&log_file)).await?;
            }
            Err(e) => {
                tracing::error!("Build {} failed: {}", build.id, e);
                update_build(client, build.id, "failed", Some(&log_file)).await?;
            }
        }

        processed += 1;
    }

    Ok(processed)
}

async fn simulate_build(
    pkg: &PackageInfo,
    arch: &str,
    build_dir: &str,
    log_file: &str,
) -> Result<(), anyhow::Error> {
    // W produkcji tutaj byłoby:
    // - docker run --rm -v ... pagbuild makepkg
    // - albo bezpośrednie wywołanie pagbuild

    let log = format!(
        "=== PaganCMS Build Worker ===\n\
         Package: {} v{}-{}\n\
         Architecture: {}\n\
         Build dir: {}\n\
         Started: {}\n\
         \n\
         [1/3] Preparing build environment...\n\
         [2/3] Downloading sources...\n\
         [3/3] Building .pag package...\n\
         \n\
         ✅ Build completed successfully\n\
         Output: {}-{}-{}-{}.pag\n\
         SHA512: abc123def456...\n\
         BLAKE3: 789ghi012jkl...\n",
        pkg.name,
        pkg.version,
        pkg.release,
        arch,
        build_dir,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
        pkg.name,
        pkg.version,
        pkg.release,
        arch,
    );

    tokio::fs::write(log_file, &log).await?;
    tracing::info!("Build log written to {}", log_file);

    Ok(())
}

async fn update_build(
    client: &reqwest::Client,
    build_id: i64,
    status: &str,
    log_path: Option<&str>,
) -> Result<(), anyhow::Error> {
    let body = serde_json::json!({
        "status": status,
        "log_path": log_path,
    });

    let resp = client
        .put(format!("{}/builds/{}/status", API_URL, build_id))
        .json(&body)
        .send()
        .await?;

    if !resp.status().is_success() {
        tracing::error!(
            "Failed to update build {} status: {}",
            build_id,
            resp.status()
        );
    }

    Ok(())
}
