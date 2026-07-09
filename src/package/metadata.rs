// ────────────────────────────────────────────────────────────────────────────────
// Pag — Metadane pakietu
// ────────────────────────────────────────────────────────────────────────────────

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pełne metadane pakietu .pag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMetadata {
    /// Name pakietu
    pub name: String,

    /// Version pakietu (semver)
    pub version: String,

    /// Numer wydania (dla tej samej wersji)
    pub release: u32,

    /// Description pakietu
    pub description: String,

    /// Architektura (x86_64, aarch64, any)
    pub architecture: String,

    /// Licencja (SPDX)
    pub license: Vec<String>,

    /// Grupa pakietów
    #[serde(default)]
    pub groups: Vec<String>,

    /// URL upstream
    #[serde(default)]
    pub url: Option<String>,

    /// Zależności (nazwa -> wymagana wersja)
    #[serde(default)]
    pub depends: Vec<String>,

    /// Opcjonalne zależności
    #[serde(default)]
    pub opt_depends: Vec<String>,

    /// Zależności wymagane do budowania
    #[serde(default)]
    pub makedepends: Vec<String>,

    /// Zależności do testów
    #[serde(default)]
    pub checkdepends: Vec<String>,

    /// Konflikty z innymi pakietami
    #[serde(default)]
    pub conflicts: Vec<String>,

    /// Pakiety zastępowane
    #[serde(default)]
    pub replaces: Vec<String>,

    /// Pakiety dostarczane (wirtualne)
    #[serde(default)]
    pub provides: Vec<String>,

    /// Rozmiar zainstalowanego pakietu (bajty)
    pub installed_size: u64,

    /// Rozmiar archiwum (bajty)
    pub archive_size: u64,

    /// Suma kontrolna SHA256 archiwum
    pub sha256: String,

    /// Suma kontrolna BLAKE3
    #[serde(default)]
    pub blake3: Option<String>,

    /// Sygnatariusz GPG
    #[serde(default)]
    pub pgp_signer: Option<String>,

    /// Data budowania
    pub build_date: DateTime<Utc>,

    /// Data instalacji (lokalnie)
    #[serde(default)]
    pub install_date: Option<DateTime<Utc>>,

    /// Reason instalacji (explicit / dependency)
    #[serde(default)]
    pub install_reason: InstallReason,

    /// Pliki w pakiecie (tylko lokalnie)
    #[serde(default)]
    pub files: Vec<PackageFile>,

    /// Skrypty hooków
    #[serde(default)]
    pub scripts: HashMap<String, String>,

    /// Dodatkowe metadane
    #[serde(default)]
    pub extra: HashMap<String, String>,

    /// Czy pakiet jest zweryfikowany
    #[serde(default)]
    pub verified: bool,

    /// Źródło repozytorium
    #[serde(default)]
    pub repository: Option<String>,

    /// Backup plików (nie usuwane przy deinstalacji)
    #[serde(default)]
    pub backup: Vec<String>,

    /// Oryginalna nazwa pliku PAGBUILD
    #[serde(default)]
    pub pagbuild_source: Option<String>,
}

impl PackageMetadata {
    /// Pełna nazwa wersji (wersja-wydanie)
    pub fn full_version(&self) -> String {
        if self.release > 0 {
            format!("{}-{}", self.version, self.release)
        } else {
            self.version.clone()
        }
    }

    /// Unikalny identyfikator
    pub fn id(&self) -> String {
        format!("{}-{}", self.name, self.full_version())
    }

    /// Sprawdź czy wersja jest nowsza
    pub fn is_newer_than(&self, other: &PackageMetadata) -> bool {
        version_compare(&self.version, &other.version) == std::cmp::Ordering::Greater
    }

    /// Sprawdź czy zależności są spełnione
    pub fn check_depends(&self, installed: &[PackageMetadata]) -> Vec<String> {
        let installed_names: Vec<&str> = installed.iter().map(|p| p.name.as_str()).collect();
        self.depends
            .iter()
            .filter(|dep| {
                let dep_name = dep.split(&['=', '>', '<'][..]).next().unwrap_or(dep);
                !installed_names.contains(&dep_name)
            })
            .cloned()
            .collect()
    }
}

/// Reason instalacji
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InstallReason {
    /// Jawnie installed przez użytkownika
    #[serde(rename = "explicit")]
    Explicit,
    /// Zainstalowany jako dependency
    #[serde(rename = "dependency")]
    Dependency,
}

impl Default for InstallReason {
    fn default() -> Self {
        InstallReason::Explicit
    }
}

/// Plik w pakiecie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageFile {
    /// Ścieżka bezwzględna
    pub path: String,

    /// Rozmiar w bajtach
    pub size: u64,

    /// Tryb uprawnień (ósemkowo)
    pub mode: u32,

    /// Właściciel UID
    pub uid: u32,

    /// Grupa GID
    pub gid: u32,

    /// Suma kontrolna MD5
    #[serde(default)]
    pub md5: Option<String>,

    /// Czy to dowiązanie symboliczne
    #[serde(default)]
    pub is_symlink: bool,

    /// Cel dowiązania (jeśli symlink)
    #[serde(default)]
    pub symlink_target: Option<String>,

    /// Czy to plik konfiguracyjny
    #[serde(default)]
    pub is_config: bool,
}

/// Wyciągnij nazwę pakietu z łańcucha zależności
pub fn dep_name(dep: &str) -> &str {
    dep.split(&['=', '>', '<', '!'][..]).next().unwrap_or(dep)
}

/// Parsuj epoch z wersji
fn parse_epoch(v: &str) -> (u32, &str) {
    if let Some(pos) = v.find(':') {
        (v[..pos].parse().unwrap_or(0), &v[pos + 1..])
    } else {
        (0, v)
    }
}

/// Porównaj wersje semantyczne
pub fn version_compare(a: &str, b: &str) -> std::cmp::Ordering {
    let (epoch_a, rest_a) = parse_epoch(a);
    let (epoch_b, rest_b) = parse_epoch(b);

    match epoch_a.cmp(&epoch_b) {
        std::cmp::Ordering::Equal => {},
        ord => return ord,
    }

    let parts_a: Vec<&str> = rest_a.split(&['.', '-', '_'][..]).collect();
    let parts_b: Vec<&str> = rest_b.split(&['.', '-', '_'][..]).collect();

    let max_len = parts_a.len().max(parts_b.len());

    for i in 0..max_len {
        let pa = parts_a.get(i).copied().unwrap_or("0");
        let pb = parts_b.get(i).copied().unwrap_or("0");

        let na: u64 = pa.parse().unwrap_or(u64::MAX);
        let nb: u64 = pb.parse().unwrap_or(u64::MAX);

        match na.cmp(&nb) {
            std::cmp::Ordering::Equal => {
                // Porównaj alfabetycznie jeśli oba nie są numeryczne
                if na == u64::MAX || nb == u64::MAX {
                    match pa.cmp(pb) {
                        std::cmp::Ordering::Equal => {},
                        ord => return ord,
                    }
                }
            }
            ord => return ord,
        }
    }

    std::cmp::Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_compare() {
        assert_eq!(version_compare("1.0.0", "1.0.0"), std::cmp::Ordering::Equal);
        assert_eq!(version_compare("1.0.1", "1.0.0"), std::cmp::Ordering::Greater);
        assert_eq!(version_compare("1.0.0", "1.0.1"), std::cmp::Ordering::Less);
        assert_eq!(version_compare("2.0.0", "1.9.9"), std::cmp::Ordering::Greater);
        assert_eq!(version_compare("1.0.0", "1.0.0-1"), std::cmp::Ordering::Less);
        assert_eq!(version_compare("1:1.0.0", "2.0.0"), std::cmp::Ordering::Greater);
    }
}
