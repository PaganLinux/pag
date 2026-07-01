// Moduł zarządzania repozytoriami

use crate::package::{RepoIndex, RepoPackageEntry};
use crate::config::Repository;
use std::path::Path;

/// Klient repozytorium
pub struct RepoClient {
    config: crate::config::Config,
    http: reqwest::Client,
}

impl RepoClient {
    pub fn new(config: &crate::config::Config) -> anyhow::Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.network.timeout))
            .user_agent(&config.network.user_agent)
            .gzip(true)
            .brotli(true)
            .build()?;

        Ok(Self {
            config: config.clone(),
            http,
        })
    }

    /// Pobiera indeks repozytorium
    pub async fn fetch_index(&self, repo: &Repository) -> anyhow::Result<RepoIndex> {
        let url = format!("{}/{}/{}/index.json", repo.url.trim_end_matches('/'), repo.arch, repo.branch);

        let response = self.http.get(&url).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Nie udało się pobrać indeksu repozytorium {} (HTTP {})", repo.name, response.status());
        }

        let index: RepoIndex = response.json().await?;
        Ok(index)
    }

    /// Pobiera plik .pag z repozytorium
    pub async fn download_package(&self, repo: &Repository, filename: &str, dest: &Path) -> anyhow::Result<()> {
        let url = format!("{}/{}/{}/{}", repo.url.trim_end_matches('/'), repo.arch, repo.branch, filename);

        let response = self.http.get(&url).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Nie udało się pobrać {} (HTTP {})", filename, response.status());
        }

        let bytes = response.bytes().await?;
        std::fs::write(dest, &bytes)?;

        Ok(())
    }

    /// Pobiera podpis pakietu (.sig)
    pub async fn download_signature(&self, repo: &Repository, filename: &str, dest: &Path) -> anyhow::Result<()> {
        let url = format!("{}/{}/{}/{}.sig", repo.url.trim_end_matches('/'), repo.arch, repo.branch, filename);
        let response = self.http.get(&url).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Nie udało się pobrać podpisu dla {}", filename);
        }

        let bytes = response.bytes().await?;
        std::fs::write(dest, &bytes)?;

        Ok(())
    }
}

/// Generuje indeks repozytorium z plików .pag w katalogu
pub fn generate_index(dir: &Path, name: &str, description: &str, url: &str) -> anyhow::Result<RepoIndex> {
    let mut packages = Vec::new();

    for entry in walkdir::WalkDir::new(dir).max_depth(2) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "pag" {
                    if let Ok(pkg) = crate::package::read_package(path) {
                        let filename = pkg.header.filename();
                        let conflicts = pkg.header.conflicts.clone();
                        packages.push(RepoPackageEntry {
                            name: pkg.header.name,
                            version: pkg.header.version,
                            release: pkg.header.release,
                            arch: pkg.header.arch,
                            description: pkg.header.description,
                            installed_size: pkg.header.installed_size,
                            compressed_size: pkg.header.compressed_size,
                            depends: pkg.header.depends,
                            provides: pkg.header.provides,
                            conflicts,
                            filename,
                            sha512: pkg.header.sha512,
                            blake3: pkg.header.blake3,
                            pgp_signature: pkg.header.pgp_signature,
                        });
                    }
                }
            }
        }
    }

    let index = RepoIndex {
        version: crate::package::PAG_FORMAT_VERSION,
        name: name.to_string(),
        description: description.to_string(),
        url: url.to_string(),
        updated: chrono::Utc::now().timestamp(),
        packages,
    };

    Ok(index)
}

/// Zapisuje indeks do pliku
pub fn save_index(index: &RepoIndex, path: &Path) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(index)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, json)?;
    Ok(())
}
