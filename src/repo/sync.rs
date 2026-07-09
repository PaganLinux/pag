// ────────────────────────────────────────────────────────────────────────────────
// Pag — Synchronizacja repozytoriów
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::config::Repository;
use crate::package::metadata::PackageMetadata;
use crate::utils::net;

/// Baza danych repozytorium (pobierana z serwera)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoDatabase {
    /// Name repozytorium
    pub name: String,

    /// Data ostatniej aktualizacji (Unix timestamp)
    pub last_update: u64,

    /// Mapa pakietów: nazwa -> metadane
    pub packages: HashMap<String, RepoPackageEntry>,

    /// Version formatu
    #[serde(default = "default_format_version")]
    pub format_version: u32,
}

fn default_format_version() -> u32 { 1 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPackageEntry {
    /// Name pakietu
    pub name: String,

    /// Aktualna wersja w repozytorium
    pub version: String,

    /// Numer wydania
    pub release: u32,

    /// Description
    pub description: String,

    /// Architektura
    pub architecture: String,

    /// Rozmiar pakietu .pag
    pub package_size: u64,

    /// Rozmiar po instalacji
    pub installed_size: u64,

    /// SHA256 archiwum
    pub sha256: String,

    /// Zależności
    #[serde(default)]
    pub depends: Vec<String>,

    /// Opcjonalne zależności
    #[serde(default)]
    pub opt_depends: Vec<String>,

    /// Konflikty
    #[serde(default)]
    pub conflicts: Vec<String>,

    /// Dostarcza
    #[serde(default)]
    pub provides: Vec<String>,

    /// Licencje
    #[serde(default)]
    pub licenses: Vec<String>,

    /// Grupy
    #[serde(default)]
    pub groups: Vec<String>,

    /// URL upstream
    pub url: Option<String>,

    /// Data budowania (ISO 8601)
    pub build_date: String,

    /// Czy ma podpis .pag.sig
    #[serde(default)]
    pub has_signature: bool,

    /// Name pliku pakietu
    #[serde(default = "default_pkg_filename")]
    pub filename: String,

    /// Ostatnia znana wersja upstream (z checkver)
    #[serde(default)]
    pub upstream_version: Option<String>,
}

fn default_pkg_filename() -> String {
    String::new() // będzie dynamicznie generowane
}

impl RepoPackageEntry {
    /// Pełna nazwa pliku .pag
    pub fn pkg_filename(&self) -> String {
        if self.filename.is_empty() {
            format!(
                "{}-{}-{}-{}.pag",
                self.name, self.version, self.release, self.architecture
            )
        } else {
            self.filename.clone()
        }
    }

    /// Name pliku podpisu
    pub fn sig_filename(&self) -> String {
        format!("{}.sig", self.pkg_filename())
    }
}

/// Zsynchronizuj repozytorium
pub async fn sync_repository(
    client: &Client,
    repo: &Repository,
    db_dir: &Path,
    force: bool,
) -> Result<RepoDatabase> {
    let db_path = db_dir.join(format!("{}.db", repo.name));

    // Sprawdź czy potrzebujemy synchronizacji
    if !force && db_path.exists() {
        if let Ok(db) = load_local_db(&db_path) {
            let age = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - db.last_update;

            // Jeśli mniej niż 1 godzina od ostatniej synchronizacji, zwróć lokalną kopię
            if age < 3600 {
                return Ok(db);
            }
        }
    }

    log::info!("Syncing repository: {} ({})", repo.name, repo.url);

    // Pobierz bazę danych z serwera
    let db_url = format!("{}/pag.db", repo.url.trim_end_matches('/'));

    let response = client.get(&db_url).send().await?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to download repository database {}: HTTP {}",
            repo.name,
            response.status()
        );
    }

    let db_json = response.text().await?;
    let mut db: RepoDatabase = serde_json::from_str(&db_json)
        .context("Invalid repository database format")?;

    // Uzupełnij nazwy plików jeśli puste
    for entry in db.packages.values_mut() {
        if entry.filename.is_empty() {
            entry.filename = format!(
                "{}-{}-{}-{}.pag",
                entry.name, entry.version, entry.release, entry.architecture
            );
        }
    }

    db.last_update = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Zapisz lokalnie
    save_local_db(&db_path, &db)?;

    log::info!("Repository {} synced: {} packages", repo.name, db.packages.len());

    Ok(db)
}

/// Wczytaj lokalną kopię bazy
pub fn load_local_db(path: &Path) -> Result<RepoDatabase> {
    let content = fs::read_to_string(path)?;
    let db: RepoDatabase = serde_json::from_str(&content)?;
    Ok(db)
}

/// Zapisz lokalną kopię bazy
pub fn save_local_db(path: &Path, db: &RepoDatabase) -> Result<()> {
    let content = serde_json::to_string_pretty(db)?;
    fs::write(path, content)?;
    Ok(())
}

/// Załaduj wszystkie lokalne bazy repozytoriów
pub fn load_all_dbs(db_dir: &Path) -> Result<Vec<RepoDatabase>> {
    let mut dbs = Vec::new();

    if !db_dir.exists() {
        return Ok(dbs);
    }

    for entry in fs::read_dir(db_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |e| e == "db") {
            match load_local_db(&path) {
                Ok(db) => dbs.push(db),
                Err(e) => {
                    log::warn!("Failed to load database {:?}: {}", path, e);
                }
            }
        }
    }

    Ok(dbs)
}

/// Znajdź pakiet we wszystkich repozytoriach
pub fn find_package<'a>(
    name: &str,
    dbs: &'a [RepoDatabase],
) -> Option<(&'a RepoDatabase, &'a RepoPackageEntry)> {
    for db in dbs {
        if let Some(entry) = db.packages.get(name) {
            return Some((db, entry));
        }
    }
    None
}

/// Wyszukaj pakiety po nazwie i opisie
pub fn search_packages<'a>(
    query: &str,
    dbs: &'a [RepoDatabase],
    search_desc: bool,
) -> Vec<(&'a RepoDatabase, &'a RepoPackageEntry)> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();

    for db in dbs {
        for entry in db.packages.values() {
            if entry.name.to_lowercase().contains(&query_lower) {
                results.push((db, entry));
            } else if search_desc
                && entry.description.to_lowercase().contains(&query_lower)
            {
                results.push((db, entry));
            }
        }
    }

    results
}
