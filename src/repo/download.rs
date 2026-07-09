// ────────────────────────────────────────────────────────────────────────────────
// Pag — Pobieranie pakietów z repozytoriów
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use reqwest::Client;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::package::signature;
use crate::repo::sync::{RepoDatabase, RepoPackageEntry};
use crate::utils::net;
use crate::utils::fs as fs_utils;

/// Pobierz pakiet .pag i zweryfikuj go
pub async fn download_package(
    client: &Client,
    repo_db: &RepoDatabase,
    entry: &RepoPackageEntry,
    config: &Config,
    cache_dir: &Path,
) -> Result<PathBuf> {
    let pkg_filename = entry.pkg_filename();
    let dest_path = cache_dir.join(&pkg_filename);

    // Sprawdź czy już jest w cache
    if dest_path.exists() {
        // Zweryfikuj sumę kontrolną
        let actual_sha = fs_utils::sha256_file(&dest_path)?;
        if actual_sha == entry.sha256 {
            log::debug!("Package {} already in cache (SHA256 matches)", pkg_filename);
            return Ok(dest_path);
        }
        log::warn!("Pakiet w cache ma niezgodny SHA256, pobieram ponownie");
    }

    // Znajdź URL repozytorium
    let repo_url = find_repo_url(&entry.name, config)?;
    let pkg_url = format!("{}/{}/{}", repo_url.trim_end_matches('/'), entry.name, pkg_filename);

    log::info!("Downloading: {}", pkg_url);

    // Pobierz pakiet
    net::download_file(client, &pkg_url, &dest_path, &config.network).await?;

    // Zweryfikuj SHA256
    let actual_sha = fs_utils::sha256_file(&dest_path)?;
    if actual_sha != entry.sha256 {
        // Usuń uszkodzony plik
        let _ = std::fs::remove_file(&dest_path);
        anyhow::bail!(
            "Checksum mismatch for {}: expected {}, got {}",
            pkg_filename,
            entry.sha256,
            actual_sha
        );
    }

    // Pobierz i zweryfikuj podpis jeśli istnieje
    if entry.has_signature {
        let sig_filename = entry.sig_filename();
        let sig_path = cache_dir.join(&sig_filename);
        let sig_url = format!("{}/{}/{}", repo_url.trim_end_matches('/'), entry.name, sig_filename);

        if net::url_exists(client, &sig_url).await {
            net::download_file(client, &sig_url, &sig_path, &config.network).await?;

            let verify_result = signature::verify_signature(
                &dest_path,
                Some(&config.paths.keyring_path),
            )?;

            match verify_result {
                signature::VerificationResult::Valid { .. } => {
                    log::info!("Package {} signature verified successfully", pkg_filename);
                }
                signature::VerificationResult::NoSignature => {
                    log::warn!("Brak podpisu dla {}", pkg_filename);
                }
                signature::VerificationResult::UnknownKey { .. } => {
                    log::warn!(
                        "Unknown signing key for {}. Import with: pag keyring import <key-id>",
                        pkg_filename
                    );
                }
                _ => {
                    let _ = std::fs::remove_file(&dest_path);
                    anyhow::bail!(
                        "Invalid signature for package {}! Package removed.",
                        pkg_filename
                    );
                }
            }
        }
    }

    Ok(dest_path)
}

/// Pobierz PAGBUILD dla pakietu
pub async fn download_pagbuild(
    client: &Client,
    package_name: &str,
    config: &Config,
) -> Result<String> {
    let repo_url = find_repo_url(package_name, config)?;
    let pagbuild_url = format!(
        "{}/{}/PAGBUILD",
        repo_url.trim_end_matches('/'),
        package_name
    );

    log::info!("Downloading PAGBUILD: {}", pagbuild_url);
    net::fetch_string(client, &pagbuild_url).await
}

/// Pobierz checkver.toml dla pakietu
pub async fn download_checkver(
    client: &Client,
    package_name: &str,
    config: &Config,
) -> Result<Option<String>> {
    let repo_url = find_repo_url(package_name, config)?;
    let checkver_url = format!(
        "{}/{}/checkver.toml",
        repo_url.trim_end_matches('/'),
        package_name
    );

    match net::fetch_string(client, &checkver_url).await {
        Ok(content) => Ok(Some(content)),
        Err(_) => Ok(None), // checkver.toml jest opcjonalny
    }
}

/// Znajdź URL repozytorium dla pakietu
fn find_repo_url(package_name: &str, config: &Config) -> Result<String> {
    // Przeszukaj repozytoria w kolejności — zwróć pierwsze które ma pakiet
    // W praktyce używamy głównego repozytorium
    for repo in &config.repositories.repos {
        if repo.enabled {
            return Ok(repo.url.clone());
        }
    }

    anyhow::bail!("No enabled repositories")
}

/// Pobierz informację o wersji systemu (ISO/rootfs)
pub async fn check_system_version(
    client: &Client,
    config: &Config,
) -> Result<Option<SystemVersion>> {
    let iso_url = format!("https://pagrepos.paganlinux.eu/iso/version.json");

    match net::fetch_string(client, &iso_url).await {
        Ok(content) => {
            let version: SystemVersion = serde_json::from_str(&content)?;
            Ok(Some(version))
        }
        Err(_) => Ok(None),
    }
}

/// Informacja o wersji systemu PaganLinux
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SystemVersion {
    /// Version systemu
    pub version: String,

    /// Data wydania (ISO 8601)
    pub release_date: String,

    /// URL do rootfs
    pub rootfs_url: String,

    /// SHA256 rootfs
    pub rootfs_sha256: String,

    /// Rozmiar rootfs
    pub rootfs_size: u64,

    /// Description wydania
    #[serde(default)]
    pub description: String,

    /// Czy to wydanie LTS
    #[serde(default)]
    pub is_lts: bool,

    /// Minimalna wersja PAG do aktualizacji
    #[serde(default)]
    pub min_pag_version: Option<String>,
}

/// Pobierz rootfs systemu
pub async fn download_system_rootfs(
    client: &Client,
    version: &SystemVersion,
    dest: &Path,
    config: &Config,
) -> Result<()> {
    log::info!("Pobieranie rootfs systemu (wersja {})...", version.version);
    net::download_file(client, &version.rootfs_url, dest, &config.network).await?;

    // Zweryfikuj SHA256
    let actual_sha = fs_utils::sha256_file(dest)?;
    if actual_sha != version.rootfs_sha256 {
        let _ = std::fs::remove_file(dest);
        anyhow::bail!("Rootfs checksum mismatch!");
    }

    Ok(())
}
