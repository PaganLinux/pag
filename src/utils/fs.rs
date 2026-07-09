// ────────────────────────────────────────────────────────────────────────────────
// Pag — Narzędzia systemu plików
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use sha2::{Sha256, Digest};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Oblicz SHA256 pliku
pub fn sha256_file(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
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
    let mut buffer = [0u8; 8192];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

/// Bezpieczne usuwanie katalogu
pub fn remove_dir_all_safe(path: &Path) -> Result<()> {
    if path.exists() {
        // Sprawdź czy nie usuwamy poza dozwolonym obszarem
        let canonical = path.canonicalize()?;
        let canonical_str = canonical.to_string_lossy();

        // Nigdy nie usuwaj katalogu root
        if canonical_str == "/" {
            anyhow::bail!("Cannot remove root directory!");
        }

        fs::remove_dir_all(path)?;
    }
    Ok(())
}

/// Sprawdź czy ścieżka znajduje się w dozwolonym obszarze
pub fn is_path_safe(base: &Path, target: &Path) -> Result<bool> {
    let base_canon = base.canonicalize().unwrap_or_else(|_| base.to_path_buf());
    let target_canon = match target.canonicalize() {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };
    Ok(target_canon.starts_with(&base_canon))
}

/// Pobierz rozmiar katalogu
pub fn dir_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum()
}

/// Formatuj rozmiar w czytelnej formie
pub fn human_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    if unit_idx == 0 {
        format!("{} {}", bytes, UNITS[unit_idx])
    } else {
        format!("{:.2} {}", size, UNITS[unit_idx])
    }
}

/// Lista plików w katalogu rekurencyjnie
pub fn list_files(path: &Path) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Atomowe zapisywanie pliku (przez temp + rename)
pub fn atomic_write(path: &Path, content: &[u8]) -> Result<()> {
    let dir = path.parent().context("No parent directory")?;
    let temp_path = dir.join(format!(
        ".{}.tmp",
        path.file_name().unwrap().to_string_lossy()
    ));

    fs::write(&temp_path, content)?;
    fs::rename(&temp_path, path)?;

    Ok(())
}

/// Sprawdź czy plik jest dowiązaniem symbolicznym
pub fn is_symlink(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}
