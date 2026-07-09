// ────────────────────────────────────────────────────────────────────────────────
// Pag — Format archiwum .pag
// Struktura: [metadata.json][compressed_data]
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression as GzCompression;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tar::{Archive, Builder};
use xz2::read::XzDecoder;
use xz2::write::XzEncoder;
use zstd::stream::{read::Decoder as ZstdDecoder, write::Encoder as ZstdEncoder};

use super::metadata::PackageMetadata;

/// Typ kompresji dla .pag
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionType {
    Zstd,
    Xz,
    Gzip,
    Bzip2,
    None,
}

impl CompressionType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "zstd" => CompressionType::Zstd,
            "xz" => CompressionType::Xz,
            "gzip" | "gz" => CompressionType::Gzip,
            "bzip2" | "bz2" => CompressionType::Bzip2,
            "none" => CompressionType::None,
            _ => CompressionType::Zstd,
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            CompressionType::Zstd => "zst",
            CompressionType::Xz => "xz",
            CompressionType::Gzip => "gz",
            CompressionType::Bzip2 => "bz2",
            CompressionType::None => "",
        }
    }
}

/// Struktura archiwum .pag
pub struct PagArchive {
    pub metadata: PackageMetadata,
    pub compression: CompressionType,
}

impl PagArchive {
    /// Utwórz archiwum .pag z katalogu
    pub fn create(
        source_dir: &Path,
        dest_path: &Path,
        metadata: PackageMetadata,
        compression: CompressionType,
    ) -> Result<u64> {
        // Krok 1: Stwórz tarball z katalogu
        let tar_data = create_tarball(source_dir)?;

        // Krok 2: Kompresuj
        let compressed = compress_data(&tar_data, compression)?;

        // Krok 3: Serializuj metadane
        let meta_json = serde_json::to_vec(&metadata)?;

        // Krok 4: Zapisz plik .pag: [4 bajty długości metadanych][metadane JSON][skompresowane dane]
        let mut file = fs::File::create(dest_path)?;

        // Zapisz długość metadanych jako 4 bajty (little-endian)
        let meta_len = meta_json.len() as u32;
        file.write_all(&meta_len.to_le_bytes())?;

        // Zapisz metadane JSON
        file.write_all(&meta_json)?;

        // Zapisz skompresowane dane
        file.write_all(&compressed)?;

        let total_size = 4 + meta_json.len() as u64 + compressed.len() as u64;
        file.flush()?;

        Ok(total_size)
    }

    /// Otwórz i odczytaj metadane z archiwum .pag
    pub fn read_metadata(path: &Path) -> Result<PackageMetadata> {
        let mut file = fs::File::open(path)?;

        // Odczytaj 4 bajty długości metadanych
        let mut len_buf = [0u8; 4];
        file.read_exact(&mut len_buf)?;
        let meta_len = u32::from_le_bytes(len_buf) as usize;

        // Odczytaj metadane JSON
        let mut meta_buf = vec![0u8; meta_len];
        file.read_exact(&mut meta_buf)?;

        let metadata: PackageMetadata = serde_json::from_slice(&meta_buf)?;
        Ok(metadata)
    }

    /// Rozpakuj archiwum .pag do katalogu docelowego
    pub fn extract(path: &Path, dest_dir: &Path) -> Result<PackageMetadata> {
        let mut file = fs::File::open(path)?;

        // Odczytaj metadane
        let mut len_buf = [0u8; 4];
        file.read_exact(&mut len_buf)?;
        let meta_len = u32::from_le_bytes(len_buf) as usize;

        let mut meta_buf = vec![0u8; meta_len];
        file.read_exact(&mut meta_buf)?;
        let metadata: PackageMetadata = serde_json::from_slice(&meta_buf)?;

        // Odczytaj skompresowane dane
        let mut compressed = Vec::new();
        file.read_to_end(&mut compressed)?;

        // Dekompresuj
        let tar_data = decompress_data(&compressed)?;

        // Rozpakuj tarball
        extract_tarball(&tar_data, dest_dir)?;

        Ok(metadata)
    }
}

/// Utwórz tarball z katalogu
fn create_tarball(source_dir: &Path) -> Result<Vec<u8>> {
    let mut tar_data = Vec::new();
    {
        let mut builder = Builder::new(&mut tar_data);
        builder.append_dir_all(".", source_dir)?;
        builder.finish()?;
    }
    Ok(tar_data)
}

/// Rozpakuj tarball do katalogu
fn extract_tarball(data: &[u8], dest_dir: &Path) -> Result<()> {
    let cursor = std::io::Cursor::new(data);
    let mut archive = Archive::new(cursor);

    // Upewnij się że katalog docelowy istnieje
    fs::create_dir_all(dest_dir)?;

    archive.unpack(dest_dir)?;
    Ok(())
}

/// Kompresuj dane
fn compress_data(data: &[u8], compression: CompressionType) -> Result<Vec<u8>> {
    match compression {
        CompressionType::Zstd => {
            let mut encoder = ZstdEncoder::new(Vec::new(), 19)?;
            encoder.write_all(data)?;
            Ok(encoder.finish()?)
        }
        CompressionType::Xz => {
            let mut encoder = XzEncoder::new(Vec::new(), 6);
            encoder.write_all(data)?;
            Ok(encoder.finish()?)
        }
        CompressionType::Gzip => {
            let mut encoder = GzEncoder::new(Vec::new(), GzCompression::best());
            encoder.write_all(data)?;
            Ok(encoder.finish()?)
        }
        CompressionType::Bzip2 => {
            let mut encoder = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::best());
            encoder.write_all(data)?;
            Ok(encoder.finish()?)
        }
        CompressionType::None => Ok(data.to_vec()),
    }
}

/// Dekompresuj dane (automatyczne wykrywanie)
fn decompress_data(data: &[u8]) -> Result<Vec<u8>> {
    // Próbuj Zstd
    if let Ok(mut decoder) = ZstdDecoder::new(data) {
        let mut out = Vec::new();
        decoder.read_to_end(&mut out)?;
        return Ok(out);
    }

    // Próbuj XZ
    {
        let mut decoder = XzDecoder::new(data);
        let mut out = Vec::new();
        if decoder.read_to_end(&mut out).is_ok() {
            return Ok(out);
        }
    }

    // Próbuj Gzip
    {
        let mut decoder = GzDecoder::new(data);
        let mut out = Vec::new();
        if decoder.read_to_end(&mut out).is_ok() {
            return Ok(out);
        }
    }

    // Zakładamy brak kompresji
    Ok(data.to_vec())
}

/// Pobierz listę plików z archiwum .pag (bez rozpakowywania)
pub fn list_archive_files(path: &Path) -> Result<Vec<String>> {
    let mut file = fs::File::open(path)?;

    // Pomiń metadane
    let mut len_buf = [0u8; 4];
    file.read_exact(&mut len_buf)?;
    let meta_len = u32::from_le_bytes(len_buf) as usize;
    let mut meta_buf = vec![0u8; meta_len];
    file.read_exact(&mut meta_buf)?;

    // Czytaj skompresowane dane
    let mut compressed = Vec::new();
    file.read_to_end(&mut compressed)?;

    // Dekompresuj
    let tar_data = decompress_data(&compressed)?;

    // Listuj pliki z tara
    let cursor = std::io::Cursor::new(tar_data);
    let mut archive = Archive::new(cursor);

    let files: Vec<String> = archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|e| e.path().ok().map(|p| p.to_string_lossy().to_string()))
        .filter_map(|s| s)
        .collect();

    Ok(files)
}
