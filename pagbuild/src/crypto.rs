// Kryptografia - dla pagbuild

use sha2::{Sha256, Sha512, Digest};
use std::path::Path;

pub fn sha512(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn blake3(data: &[u8]) -> String {
    let hash = blake3::hash(data);
    hash.to_hex().to_string()
}

pub fn file_sha512(path: &Path) -> anyhow::Result<String> {
    let data = std::fs::read(path)?;
    Ok(sha512(&data))
}

pub fn file_blake3(path: &Path) -> anyhow::Result<String> {
    let data = std::fs::read(path)?;
    Ok(blake3(&data))
}
