// ────────────────────────────────────────────────────────────────────────────────
// Pag — checkver.toml — system sprawdzania aktualności pakietów
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Struktura pliku checkver.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckverConfig {
    /// Metadane pakietu
    pub package: CheckverPackage,

    /// Źródła do sprawdzania
    #[serde(default)]
    pub sources: Vec<CheckSource>,

    /// Konfiguracja sprawdzania
    #[serde(default)]
    pub check: CheckOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckverPackage {
    /// Name pakietu
    pub name: String,

    /// Aktualna wersja lokalna
    pub version: String,

    /// Description (opcjonalny)
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckSource {
    /// Typ źródła: git, github, gitlab, url, pypi, cratesio, npm
    #[serde(rename = "type")]
    pub source_type: String,

    /// URL źródła
    pub url: String,

    /// Regex do wyłuskania wersji z outputu
    #[serde(default)]
    pub version_regex: Option<String>,

    /// Ścieżka do pliku z wersją (dla git)
    #[serde(default)]
    pub version_path: Option<String>,

    /// Gałąź do sprawdzania (dla git)
    #[serde(default = "default_branch")]
    pub branch: String,

    /// Token API (dla GitHub/GitLab)
    #[serde(default)]
    pub api_token: Option<String>,

    /// Niestandardowe nagłówki HTTP
    #[serde(default)]
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckOptions {
    /// Jak często sprawdzać (w godzinach)
    #[serde(default = "default_check_interval")]
    pub interval_hours: u64,

    /// Czy ignorować pre-release
    #[serde(default)]
    pub ignore_prerelease: bool,

    /// Regex do odfiltrowania wersji
    #[serde(default)]
    pub version_filter: Option<String>,

    /// Ostatni czas sprawdzenia
    #[serde(default)]
    pub last_check: Option<DateTime<Utc>>,

    /// Ostatnia znana wersja upstream
    #[serde(default)]
    pub last_upstream_version: Option<String>,
}

fn default_branch() -> String { "main".into() }
fn default_check_interval() -> u64 { 24 }

impl CheckverConfig {
    /// Wczytaj checkver.toml ze ścieżki
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read checkver.toml: {:?}", path))?;
        toml::from_str(&content).context("Invalid checkver.toml format")
    }

    /// Zapisz checkver.toml
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Sprawdź czy pakiet ma nowszą wersję
    pub async fn check_upstream(&mut self, client: &Client) -> Result<Option<String>> {
        for source in &self.sources {
            match source.source_type.as_str() {
                "github" => {
                    if let Some(version) = check_github(client, source).await? {
                        if is_version_newer(&version, &self.package.version) {
                            return Ok(Some(version));
                        }
                    }
                }
                "gitlab" => {
                    if let Some(version) = check_gitlab(client, source).await? {
                        if is_version_newer(&version, &self.package.version) {
                            return Ok(Some(version));
                        }
                    }
                }
                "git" => {
                    if let Some(version) = check_git_tag(source).await? {
                        if is_version_newer(&version, &self.package.version) {
                            return Ok(Some(version));
                        }
                    }
                }
                "url" => {
                    if let Some(version) = check_url(client, source).await? {
                        if is_version_newer(&version, &self.package.version) {
                            return Ok(Some(version));
                        }
                    }
                }
                "pypi" => {
                    if let Some(version) = check_pypi(client, source).await? {
                        if is_version_newer(&version, &self.package.version) {
                            return Ok(Some(version));
                        }
                    }
                }
                "cratesio" => {
                    if let Some(version) = check_cratesio(client, source).await? {
                        if is_version_newer(&version, &self.package.version) {
                            return Ok(Some(version));
                        }
                    }
                }
                other => {
                    log::warn!("Unknown source type: {}", other);
                }
            }
        }

        Ok(None)
    }
}

/// Sprawdź wersję na GitHub
async fn check_github(client: &Client, source: &CheckSource) -> Result<Option<String>> {
    // Parsuj URL: https://github.com/user/repo
    let repo = source
        .url
        .trim_start_matches("https://github.com/")
        .trim_start_matches("http://github.com/")
        .trim_end_matches(".git")
        .trim_end_matches('/');

    let api_url = format!("https://api.github.com/repos/{}/releases/latest", repo);

    let mut req = client.get(&api_url).header("User-Agent", "pag-package-manager");

    if let Some(ref token) = source.api_token {
        req = req.header("Authorization", format!("Bearer {}", token));
    }

    let response = req.send().await?;
    if !response.status().is_success() {
        return Ok(None);
    }

    let release: serde_json::Value = response.json().await?;
    let tag = release["tag_name"].as_str().unwrap_or("").to_string();

    // Wyciągnij wersję z tagu
    let version = extract_version(&tag, source.version_regex.as_deref());

    Ok(Some(version))
}

/// Sprawdź wersję na GitLab
async fn check_gitlab(client: &Client, source: &CheckSource) -> Result<Option<String>> {
    let repo = source
        .url
        .trim_start_matches("https://gitlab.com/")
        .trim_start_matches("http://gitlab.com/")
        .trim_end_matches(".git")
        .trim_end_matches('/');

    let encoded = repo.replace('/', "%2F");
    let api_url = format!("https://gitlab.com/api/v4/projects/{}/releases", encoded);

    let response = client.get(&api_url).send().await?;
    if !response.status().is_success() {
        return Ok(None);
    }

    let releases: Vec<serde_json::Value> = response.json().await?;
    if let Some(latest) = releases.first() {
        let tag = latest["tag_name"].as_str().unwrap_or("").to_string();
        let version = extract_version(&tag, source.version_regex.as_deref());
        return Ok(Some(version));
    }

    Ok(None)
}

/// Sprawdź tagi git
async fn check_git_tag(source: &CheckSource) -> Result<Option<String>> {
    use std::process::Command;

    let output = Command::new("git")
        .args(["ls-remote", "--tags", "--refs", &source.url])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let mut tags: Vec<String> = stdout
                .lines()
                .filter_map(|l| {
                    let parts: Vec<&str> = l.split('\t').collect();
                    parts.get(1).map(|t| {
                        t.trim_start_matches("refs/tags/").to_string()
                    })
                })
                .collect();

            // Sortuj i weź najnowszy
            tags.sort_by(|a, b| {
                super::super::package::metadata::version_compare(b, a)
            });

            if let Some(latest) = tags.into_iter().next() {
                let version = extract_version(&latest, source.version_regex.as_deref());
                return Ok(Some(version));
            }
        }
        _ => {}
    }

    Ok(None)
}

/// Sprawdź wersję z URL (pobiera zawartość i szuka regex)
async fn check_url(client: &Client, source: &CheckSource) -> Result<Option<String>> {
    let response = client.get(&source.url).send().await?;
    if !response.status().is_success() {
        return Ok(None);
    }

    let content = response.text().await?;

    if let Some(ref regex_str) = source.version_regex {
        let re = Regex::new(regex_str)?;
        if let Some(caps) = re.captures(&content) {
            if let Some(version) = caps.get(1) {
                return Ok(Some(version.as_str().to_string()));
            }
        }
    }

    Ok(None)
}

/// Sprawdź wersję na PyPI
async fn check_pypi(client: &Client, source: &CheckSource) -> Result<Option<String>> {
    let package_name = source
        .url
        .trim_start_matches("https://pypi.org/project/")
        .trim_start_matches("https://pypi.org/pypi/")
        .trim_end_matches('/');

    let api_url = format!("https://pypi.org/pypi/{}/json", package_name);

    let response = client.get(&api_url).send().await?;
    if !response.status().is_success() {
        return Ok(None);
    }

    let data: serde_json::Value = response.json().await?;
    let version = data["info"]["version"].as_str().unwrap_or("").to_string();

    if version.is_empty() {
        Ok(None)
    } else {
        Ok(Some(version))
    }
}

/// Sprawdź wersję na crates.io
async fn check_cratesio(client: &Client, source: &CheckSource) -> Result<Option<String>> {
    let crate_name = source
        .url
        .trim_start_matches("https://crates.io/crates/")
        .trim_start_matches("https://crates.io/api/v1/crates/")
        .trim_end_matches('/');

    let api_url = format!("https://crates.io/api/v1/crates/{}", crate_name);

    let response = client.get(&api_url)
        .header("User-Agent", "pag-package-manager")
        .send().await?;

    if !response.status().is_success() {
        return Ok(None);
    }

    let data: serde_json::Value = response.json().await?;
    let version = data["crate"]["max_stable_version"]
        .as_str()
        .or_else(|| data["crate"]["max_version"].as_str())
        .unwrap_or("")
        .to_string();

    if version.is_empty() {
        Ok(None)
    } else {
        Ok(Some(version))
    }
}

/// Wyciągnij wersję z tekstu używając opcjonalnego regex
fn extract_version(text: &str, regex: Option<&str>) -> String {
    // Usuń prefix 'v' jeśli jest
    let cleaned = text.trim_start_matches('v').trim_start_matches('V');

    if let Some(re_str) = regex {
        if let Ok(re) = Regex::new(re_str) {
            if let Some(caps) = re.captures(cleaned) {
                if let Some(v) = caps.get(1) {
                    return v.as_str().to_string();
                }
            }
        }
    }

    cleaned.to_string()
}

/// Sprawdź czy wersja jest nowsza
fn is_version_newer(new_ver: &str, current_ver: &str) -> bool {
    use crate::package::metadata::version_compare;
    version_compare(new_ver, current_ver) == std::cmp::Ordering::Greater
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_version() {
        assert_eq!(extract_version("v1.2.3", None), "1.2.3");
        assert_eq!(extract_version("release-2.0.0", None), "release-2.0.0");
        assert_eq!(
            extract_version("v1.2.3-beta", Some(r"(\d+\.\d+\.\d+)")),
            "1.2.3"
        );
    }
}
