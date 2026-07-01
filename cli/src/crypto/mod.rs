// Moduł kryptografii - hashowanie, GPG, checksumy

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

pub fn verify_checksum(data: &[u8], expected: &str, algo: &str) -> bool {
    let actual = match algo.to_lowercase().as_str() {
        "sha256" => sha256(data),
        "sha512" => sha512(data),
        "blake3" => blake3(data),
        _ => return false,
    };
    actual == expected
}

/// Importuje klucz GPG przez zewnętrzne narzędzie
pub fn import_key(key_data: &[u8], _keyring: &Path) -> anyhow::Result<()> {
    let mut child = std::process::Command::new("gpg")
        .args(["--import", "--batch", "--no-tty"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()?;

    use std::io::Write;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(key_data)?;
    }

    child.wait()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let hash = sha256(b"hello pag");
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_blake3() {
        let hash = blake3(b"hello pag");
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_verify_checksum() {
        let data = b"test data";
        let hash = sha256(data);
        assert!(verify_checksum(data, &hash, "sha256"));
        assert!(!verify_checksum(data, "badhash", "sha256"));
    }
}
