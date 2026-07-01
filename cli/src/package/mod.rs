// Format pakietu .pag
//
// Struktura pliku .pag:
// ┌──────────────────────────────────────┐
// │  Magic: b"PAG\x01"  (4 bajty)        │
// │  Header Size: u32 LE                 │
// │  ┌─ Header (JSON) ────────────────┐  │
// │  │  name, version, arch, ...       │  │
// │  │  deps, provides, conflicts, ... │  │
// │  │  file_list: [{path, size, ...}] │  │
// │  │  scripts: {pre-install, ...}    │  │
// │  └────────────────────────────────┘  │
// │  ┌─ Payload (tar.zst) ────────────┐  │
// │  │  Skompresowane pliki pakietu   │  │
// │  └────────────────────────────────┘  │
// │  ┌─ Signature (opcjonalnie) ──────┐  │
// │  │  Podpis GPG                    │  │
// │  └────────────────────────────────┘  │
// └──────────────────────────────────────┘

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::Path;

/// Magiczne bajty identyfikujące plik .pag
pub const PAG_MAGIC: &[u8; 4] = b"PAG\x01";

/// Aktualna wersja formatu
pub const PAG_FORMAT_VERSION: u32 = 1;

/// Typy kompresji
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Compression {
    Zstd,
    Xz,
    Gzip,
    None,
}

/// Główny nagłówek pakietu .pag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageHeader {
    /// Wersja formatu
    pub format_version: u32,

    /// Nazwa pakietu
    pub name: String,

    /// Wersja pakietu
    pub version: String,

    /// Numer wydania
    pub release: u32,

    /// Opis pakietu
    pub description: String,

    /// URL projektu
    pub url: Option<String>,

    /// Architektura (x86_64, aarch64, all...)
    pub arch: String,

    /// Licencja
    pub license: String,

    /// Rozmiar zainstalowanego pakietu (bajty)
    pub installed_size: u64,

    /// Rozmiar skompresowanego archiwum (bajty)
    pub compressed_size: u64,

    /// Typ kompresji payloadu
    pub compression: Compression,

    /// Zależności
    #[serde(default)]
    pub depends: Vec<String>,

    /// Zależności opcjonalne
    #[serde(default)]
    pub optdepends: Vec<String>,

    /// Zależności build-time (makedepends)
    #[serde(default)]
    pub makedepends: Vec<String>,

    /// Konflikty
    #[serde(default)]
    pub conflicts: Vec<String>,

    /// Zastępowane pakiety
    #[serde(default)]
    pub replaces: Vec<String>,

    /// Dostarczane wirtualne pakiety
    #[serde(default)]
    pub provides: Vec<String>,

    /// Pakiety zależne od tego (reverse deps)
    #[serde(default)]
    pub required_by: Vec<String>,

    /// Opcjonalne (zalecane) pakiety
    #[serde(default)]
    pub optional: Vec<String>,

    /// Grupa pakietu
    pub group: Option<String>,

    /// Opiekun (maintainer)
    pub maintainer: Option<String>,

    /// Data budowania (unix timestamp)
    pub build_date: i64,

    /// Hash SHA512 payloadu
    pub sha512: String,

    /// Hash BLAKE3 payloadu
    pub blake3: String,

    /// PGPSig (opcjonalnie)
    pub pgp_signature: Option<String>,

    /// Lista plików
    #[serde(default)]
    pub files: Vec<FileEntry>,

    /// Skrypty instalacyjne
    #[serde(default)]
    pub scripts: HashMap<String, String>,

    /// Niestandardowe metadane
    #[serde(default)]
    pub custom: HashMap<String, String>,
}

/// Wpis pliku w pakiecie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    /// Ścieżka w systemie (np. /usr/bin/7zz)
    pub path: String,

    /// Rozmiar pliku
    pub size: u64,

    /// Tryb (uprawnienia)
    pub mode: u32,

    /// Właściciel (UID)
    pub uid: u32,

    /// Grupa (GID)
    pub gid: u32,

    /// Typ pliku (file, dir, symlink)
    #[serde(rename = "type")]
    pub file_type: FileType,

    /// Cel symlinka (jeśli symlink)
    pub link_target: Option<String>,

    /// SHA256 pliku
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    File,
    Dir,
    Symlink,
    Hardlink,
}

/// Reprezentuje wczytany pakiet .pag
pub struct Package {
    pub header: PackageHeader,
    /// Surowy payload (skompresowany)
    pub raw_payload: Vec<u8>,
}

impl PackageHeader {
    /// Tworzy nowy nagłówek z podstawowymi polami
    pub fn new(name: &str, version: &str, arch: &str) -> Self {
        Self {
            format_version: PAG_FORMAT_VERSION,
            name: name.to_string(),
            version: version.to_string(),
            release: 1,
            description: String::new(),
            url: None,
            arch: arch.to_string(),
            license: "custom".to_string(),
            installed_size: 0,
            compressed_size: 0,
            compression: Compression::Zstd,
            depends: vec![],
            optdepends: vec![],
            makedepends: vec![],
            conflicts: vec![],
            replaces: vec![],
            provides: vec![],
            required_by: vec![],
            optional: vec![],
            group: None,
            maintainer: None,
            build_date: chrono::Utc::now().timestamp(),
            sha512: String::new(),
            blake3: String::new(),
            pgp_signature: None,
            files: vec![],
            scripts: HashMap::new(),
            custom: HashMap::new(),
        }
    }

    /// Unikalny identyfikator pakietu: nazwa-wersja-rWydanie
    pub fn full_id(&self) -> String {
        format!("{}-{}-r{}", self.name, self.version, self.release)
    }

    /// Nazwa pliku .pag: nazwa-wersja-rWydanie-arch.pag
    pub fn filename(&self) -> String {
        format!("{}-{}-r{}-{}.pag", self.name, self.version, self.release, self.arch)
    }

    /// Sprawdza czy pakiet spełnia zależność
    pub fn satisfies(&self, dep: &str) -> bool {
        // Dokładna nazwa
        if dep == self.name {
            return true;
        }
        // Wirtualne provides
        for provide in &self.provides {
            if dep == provide || provide.starts_with(&format!("{}=", dep)) {
                return true;
            }
            // Wersjonowane provides: foo>=1.0
            if let Some((name, ver)) = provide.split_once('=') {
                if name == dep {
                    return true;
                }
            }
        }
        false
    }
}

/// Odczytuje pakiet .pag z pliku
pub fn read_package(path: &Path) -> anyhow::Result<Package> {
    let data = std::fs::read(path)?;
    read_package_from_bytes(&data)
}

/// Parsuje pakiet .pag z bajtów
pub fn read_package_from_bytes(data: &[u8]) -> anyhow::Result<Package> {
    if data.len() < 8 {
        anyhow::bail!("Plik za krótki na nagłówek .pag");
    }

    // Sprawdź magic
    if &data[0..4] != PAG_MAGIC.as_slice() {
        anyhow::bail!("Nieprawidłowy magic: to nie jest plik .pag");
    }

    // Odczytaj rozmiar nagłówka
    let header_size = u32::from_le_bytes(data[4..8].try_into()?) as usize;

    if data.len() < 8 + header_size {
        anyhow::bail!("Plik za krótki na zadeklarowany nagłówek");
    }

    // Parsuj JSON header
    let header_json = &data[8..8 + header_size];
    let header: PackageHeader = serde_json::from_slice(header_json)?;

    // Payload to reszta danych
    let payload_start = 8 + header_size;
    let raw_payload = data[payload_start..].to_vec();

    Ok(Package { header, raw_payload })
}

/// Zapisuje pakiet .pag do pliku
pub fn write_package(path: &Path, header: &PackageHeader, payload: &[u8]) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(path)?;
    write_package_to_writer(&mut file, header, payload)
}

/// Zapisuje pakiet .pag do writera
pub fn write_package_to_writer<W: Write>(writer: &mut W, header: &PackageHeader, payload: &[u8]) -> anyhow::Result<()> {
    let header_json = serde_json::to_vec(header)?;
    let header_size = header_json.len() as u32;

    // Magic
    writer.write_all(PAG_MAGIC)?;
    // Header size (LE u32)
    writer.write_all(&header_size.to_le_bytes())?;
    // Header JSON
    writer.write_all(&header_json)?;
    // Payload
    writer.write_all(payload)?;

    Ok(())
}

/// Tworzy indeks repozytorium (APKINDEX-like format dla pag)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoIndex {
    /// Wersja formatu indeksu
    pub version: u32,

    /// Nazwa repozytorium
    pub name: String,

    /// Opis repozytorium
    pub description: String,

    /// URL repozytorium
    pub url: String,

    /// Data ostatniej aktualizacji
    pub updated: i64,

    /// Lista pakietów
    pub packages: Vec<RepoPackageEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPackageEntry {
    pub name: String,
    pub version: String,
    pub release: u32,
    pub arch: String,
    pub description: String,
    pub installed_size: u64,
    pub compressed_size: u64,
    pub depends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub filename: String,
    pub sha512: String,
    pub blake3: String,
    pub pgp_signature: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_roundtrip() {
        let mut header = PackageHeader::new("test-pkg", "1.0.0", "x86_64");
        header.description = "Test package".into();
        header.license = "MIT".into();
        header.depends = vec!["glibc".into()];
        header.provides = vec!["test-virtual".into()];

        let payload = b"fake-payload-data";

        let mut buf = Vec::new();
        write_package_to_writer(&mut buf, &header, payload).unwrap();

        let pkg = read_package_from_bytes(&buf).unwrap();
        assert_eq!(pkg.header.name, "test-pkg");
        assert_eq!(pkg.header.version, "1.0.0");
        assert_eq!(pkg.header.depends, vec!["glibc"]);
        assert_eq!(pkg.raw_payload, payload);
    }

    #[test]
    fn test_full_id() {
        let header = PackageHeader::new("nginx", "1.26.0", "x86_64");
        assert_eq!(header.full_id(), "nginx-1.26.0-r1");
    }

    #[test]
    fn test_satisfies() {
        let mut header = PackageHeader::new("glibc", "2.39", "x86_64");
        header.provides = vec!["libc.so.6".into(), "glibc=2.39".into()];

        assert!(header.satisfies("glibc"));
        assert!(header.satisfies("libc.so.6"));
        assert!(!header.satisfies("nonexistent"));
    }
}
