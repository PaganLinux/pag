// ────────────────────────────────────────────────────────────────────────────────
// Pag — Narzędzia sieciowe
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use futures_util::StreamExt;
use reqwest::Client;
use std::path::Path;
use std::time::Duration;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::config::NetworkConfig;

/// Klient HTTP z konfiguracją
pub fn create_client(config: &NetworkConfig) -> Result<Client> {
    let mut builder = Client::builder()
        .timeout(Duration::from_secs(config.timeout_secs))
        .user_agent(&config.user_agent)
        .danger_accept_invalid_certs(!config.verify_ssl);

    if let Some(ref proxy) = config.proxy {
        builder = builder.proxy(reqwest::Proxy::all(proxy)?);
    }

    builder.build().context("Failed to create HTTP client")
}

/// Pobierz plik z URL z paskiem postępu
pub async fn download_file(
    client: &Client,
    url: &str,
    dest: &Path,
    config: &NetworkConfig,
) -> Result<()> {
    let mut retries = 0;
    let max_retries = config.max_retries;

    loop {
        match try_download(client, url, dest).await {
            Ok(()) => return Ok(()),
            Err(e) if retries < max_retries => {
                retries += 1;
                log::warn!(
                    "Download failed (attempt {}/{}): {}",
                    retries,
                    max_retries,
                    e
                );
                tokio::time::sleep(Duration::from_secs(2u64.pow(retries))).await;
            }
            Err(e) => return Err(e),
        }
    }
}

async fn try_download(client: &Client, url: &str, dest: &Path) -> Result<()> {
    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("Failed to download: {}", url))?;

    if !response.status().is_success() {
        anyhow::bail!("HTTP {} podczas pobierania: {}", response.status(), url);
    }

    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    let mut file = fs::File::create(dest).await?;
use futures_util::StreamExt;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;

        if total_size > 0 {
            let progress = (downloaded as f64 / total_size as f64) * 100.0;
            log::debug!("Pobieranie: {:.1}% ({}/{})", progress,
                super::fs::human_size(downloaded),
                super::fs::human_size(total_size));
        }
    }

    file.flush().await?;
    Ok(())
}

/// Pobierz zawartość jako string
pub async fn fetch_string(client: &Client, url: &str) -> Result<String> {
    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("Failed to download: {}", url))?;

    if !response.status().is_success() {
        anyhow::bail!("HTTP {} dla: {}", response.status(), url);
    }

    response
        .text()
        .await
        .context("Failed to read response")
}

/// Sprawdź czy URL istnieje (HEAD request)
pub async fn url_exists(client: &Client, url: &str) -> bool {
    client
        .head(url)
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

/// Parsuj URL repozytorium
pub fn parse_repo_url(base: &str, arch: &str, package: &str, file: &str) -> String {
    format!("{}/{}/{}", base.trim_end_matches('/'), package, file)
}
