// Format pakietu .pag - dla pagbuild (uproszczona wersja)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

pub const PAG_MAGIC: &[u8; 4] = b"PAG\x01";
pub const PAG_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Compression {
    Zstd,
    Xz,
    Gzip,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageHeader {
    pub format_version: u32,
    pub name: String,
    pub version: String,
    pub release: u32,
    pub description: String,
    pub url: Option<String>,
    pub arch: String,
    pub license: String,
    pub installed_size: u64,
    pub compressed_size: u64,
    pub compression: Compression,
    #[serde(default)]
    pub depends: Vec<String>,
    #[serde(default)]
    pub optdepends: Vec<String>,
    #[serde(default)]
    pub makedepends: Vec<String>,
    #[serde(default)]
    pub conflicts: Vec<String>,
    #[serde(default)]
    pub replaces: Vec<String>,
    #[serde(default)]
    pub provides: Vec<String>,
    #[serde(default)]
    pub required_by: Vec<String>,
    #[serde(default)]
    pub optional: Vec<String>,
    pub group: Option<String>,
    pub maintainer: Option<String>,
    pub build_date: i64,
    pub sha512: String,
    pub blake3: String,
    pub pgp_signature: Option<String>,
    #[serde(default)]
    pub files: Vec<FileEntry>,
    #[serde(default)]
    pub scripts: HashMap<String, String>,
    #[serde(default)]
    pub custom: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub size: u64,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    #[serde(rename = "type")]
    pub file_type: FileType,
    pub link_target: Option<String>,
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

impl PackageHeader {
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

    pub fn filename(&self) -> String {
        format!("{}-{}-r{}-{}.pag", self.name, self.version, self.release, self.arch)
    }
}

pub struct Package {
    pub header: PackageHeader,
    pub raw_payload: Vec<u8>,
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

    if &data[0..4] != PAG_MAGIC.as_slice() {
        anyhow::bail!("Nieprawidłowy magic: to nie jest plik .pag");
    }

    let header_size = u32::from_le_bytes(data[4..8].try_into()?) as usize;

    if data.len() < 8 + header_size {
        anyhow::bail!("Plik za krótki na nagłówek");
    }

    let header_json = &data[8..8 + header_size];
    let header: PackageHeader = serde_json::from_slice(header_json)?;

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

    writer.write_all(PAG_MAGIC)?;
    writer.write_all(&header_size.to_le_bytes())?;
    writer.write_all(&header_json)?;
    writer.write_all(payload)?;

    Ok(())
}
