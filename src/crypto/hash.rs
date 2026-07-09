// ────────────────────────────────────────────────────────────────────────────────
// Pag — Funkcje haszujące
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use sha2::{Sha256, Digest as Sha2Digest};
use std::fs;
use std::io::Read;
use std::path::Path;

/// Oblicz SHA256 pliku
pub fn sha256_file(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Oblicz BLAKE3 pliku
pub fn blake3_file(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0u8; 65536];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

/// Oblicz MD5 pliku
pub fn md5_file(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = md5::Md5::new();
    let mut buffer = [0u8; 65536];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Oblicz wiele hashy naraz (dla wydajności)
pub fn multi_hash_file(path: &Path) -> Result<FileHashes> {
    let mut file = fs::File::open(path)?;
    let mut sha256 = Sha256::new();
    let mut blake3 = blake3::Hasher::new();
    let mut md5 = md5::Md5::new();
    let mut buffer = [0u8; 65536];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 { break; }
        sha256.update(&buffer[..n]);
        blake3.update(&buffer[..n]);
        md5.update(&buffer[..n]);
    }

    Ok(FileHashes {
        sha256: format!("{:x}", sha256.finalize()),
        blake3: blake3.finalize().to_hex().to_string(),
        md5: format!("{:x}", md5.finalize()),
    })
}

#[derive(Debug, Clone)]
pub struct FileHashes {
    pub sha256: String,
    pub blake3: String,
    pub md5: String,
}

/// Weryfikuj sumę kontrolną
pub fn verify_checksum(path: &Path, expected_sha256: &str) -> Result<bool> {
    let actual = sha256_file(path)?;
    Ok(actual == expected_sha256)
}
