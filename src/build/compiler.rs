// ────────────────────────────────────────────────────────────────────────────────
// Pag — Orkiestracja budowania pakietów
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

use crate::build::pagbuild::{self, PagBuildInfo};
use crate::config::Config;
use crate::package::metadata::{InstallReason, PackageMetadata};
use crate::package::pag::{CompressionType, PagArchive};
use crate::package::signature;

/// Zbuduj pakiet .pag z PAGBUILD
pub async fn build_package(
    pagbuild_path: &Path,
    config: &Config,
    client: &reqwest::Client,
    clean: bool,
    sign: bool,
) -> Result<PathBuf> {
    // Parsuj PAGBUILD
    let pagbuild_info = PagBuildInfo::parse(pagbuild_path)?;

    log::info!(
        "Building package: {} {}-{}",
        pagbuild_info.pkgname,
        pagbuild_info.pkgver,
        pagbuild_info.pkgrel
    );

    // Sprawdź architekturę
    if !pagbuild_info.is_arch_supported(&config.general.arch) {
        anyhow::bail!(
            "Pakiet {} nie wspiera architektury {}",
            pagbuild_info.pkgname,
            config.general.arch
        );
    }

    // Przygotuj katalogi
    let build_dir = config.paths.build_path.join(&pagbuild_info.pkgname);
    let pkg_dir = build_dir.join("pkg");
    let src_dir = build_dir.join("src");

    fs::create_dir_all(&build_dir)?;
    fs::create_dir_all(&pkg_dir)?;
    fs::create_dir_all(&src_dir)?;

    // Pobierz źródła
    pagbuild::download_sources(&pagbuild_info, &src_dir, client, config).await?;

    // Wykonaj PAGBUILD
    pagbuild::execute_pagbuild(&pagbuild_info, &build_dir, &pkg_dir, &config.build)?;

    // Sprawdź czy katalog pkg nie jest pusty
    if fs::read_dir(&pkg_dir)?.next().is_none() {
        anyhow::bail!("pkg directory is empty after build — PAGBUILD may not have created files");
    }

    // Zbierz metadane
    let metadata = collect_metadata(&pagbuild_info, &pkg_dir, config)?;

    // Stwórz archiwum .pag
    let compression = CompressionType::from_str(&config.general.compression);
    let pkg_filename = pagbuild_info.pag_filename();
    let pkg_dest = config.paths.pkg_cache_path.join(&pkg_filename);

    let archive_size = PagArchive::create(&pkg_dir, &pkg_dest, metadata, compression)?;

    log::info!(
        "Package built: {} ({})",
        pkg_filename,
        crate::utils::fs::human_size(archive_size)
    );

    // Podpisz jeśli wymagane
    if sign {
        match signature::sign_package(&pkg_dest, config.gpg.default_key.as_deref()) {
            Ok(sig_path) => {
                log::info!("Package signed: {}", sig_path.display());
            }
            Err(e) => {
                log::warn!("Failed to sign package: {}", e);
            }
        }
    }

    // Wyczyść jeśli wymagane
    if clean {
        let _ = fs::remove_dir_all(&build_dir);
    }

    Ok(pkg_dest)
}

/// Zbierz metadane pakietu po budowaniu
fn collect_metadata(
    pagbuild: &PagBuildInfo,
    pkg_dir: &Path,
    config: &Config,
) -> Result<PackageMetadata> {
    use walkdir::WalkDir;

    // Oblicz rozmiar zainstalowanego pakietu
    let installed_size: u64 = WalkDir::new(pkg_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum();

    // Zbierz listę plików
    let files: Vec<crate::package::metadata::PackageFile> = WalkDir::new(pkg_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() || e.file_type().is_symlink())
        .filter_map(|e| {
            let path = e.path();
            let rel_path = path.strip_prefix(pkg_dir).unwrap_or(path);

            let metadata = match e.metadata() {
                Ok(m) => m,
                Err(_) => {
                    log::warn!("Failed to read metadata for: {}", path.display());
                    return None;
                }
            };

            Some(crate::package::metadata::PackageFile {
                path: format!("/{}", rel_path.display()),
                size: metadata.len(),
                #[cfg(unix)]
                mode: {
                    use std::os::unix::fs::PermissionsExt;
                    metadata.permissions().mode()
                },
                #[cfg(not(unix))]
                mode: 0o644,
                #[cfg(unix)]
                uid: {
                    use std::os::unix::fs::MetadataExt;
                    metadata.uid()
                },
                #[cfg(not(unix))]
                uid: 0,
                #[cfg(unix)]
                gid: {
                    use std::os::unix::fs::MetadataExt;
                    metadata.gid()
                },
                #[cfg(not(unix))]
                gid: 0,
                md5: None,
                is_symlink: e.file_type().is_symlink(),
                symlink_target: if e.file_type().is_symlink() {
                    fs::read_link(path).ok().map(|p| p.display().to_string())
                } else {
                    None
                },
                is_config: pagbuild.backup.iter().any(|b| {
                    rel_path.to_string_lossy().contains(b)
                }),
            })
        })
        .collect();

    // Hash pustego placeholderu (zostanie uzupełniony przy tworzeniu archiwum)
    let placeholder_sha = String::from("0000000000000000000000000000000000000000000000000000000000000000");

    Ok(PackageMetadata {
        name: pagbuild.pkgname.clone(),
        version: pagbuild.pkgver.clone(),
        release: pagbuild.pkgrel,
        description: pagbuild.pkgdesc.clone(),
        architecture: pagbuild.arch.first().cloned().unwrap_or_else(|| "x86_64".into()),
        license: pagbuild.license.clone(),
        groups: pagbuild.groups.clone(),
        url: pagbuild.url.clone(),
        depends: pagbuild.depends.clone(),
        opt_depends: pagbuild.optdepends.clone(),
        makedepends: pagbuild.makedepends.clone(),
        checkdepends: pagbuild.checkdepends.clone(),
        conflicts: pagbuild.conflicts.clone(),
        replaces: pagbuild.replaces.clone(),
        provides: pagbuild.provides.clone(),
        installed_size,
        archive_size: 0, // Będzie uzupełnione
        sha256: placeholder_sha, // Będzie uzupełnione
        blake3: None,
        pgp_signer: config.gpg.default_key.clone(),
        build_date: Utc::now(),
        install_date: None,
        install_reason: InstallReason::Explicit,
        files,
        scripts: std::collections::HashMap::new(),
        extra: pagbuild.custom_vars.clone(),
        verified: false,
        repository: None,
        backup: pagbuild.backup.clone(),
        pagbuild_source: Some(pagbuild.source_path.display().to_string()),
    })
}
