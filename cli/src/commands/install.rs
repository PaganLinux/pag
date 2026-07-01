// Komenda: install
// Instalacja pakietów .pag z repozytoriów

use crate::config::Config;
use crate::db::InstallReason;
use crate::deps::{DepSolver, PackageInfo};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

pub async fn execute(
    cfg: &Config,
    packages: &[String],
    as_deps: bool,
    overwrite: bool,
    ignore_deps: bool,
    dry_run: bool,
) -> anyhow::Result<()> {
    if packages.is_empty() {
        anyhow::bail!("Nie podano pakietów do zainstalowania");
    }

    // Otwórz bazę danych
    let db = crate::db::PackageDb::open(&std::path::PathBuf::from(&cfg.general.db_path))?;

    // Pobierz zainstalowane pakiety
    let installed = db.list_packages(None)?;
    let mut solver = DepSolver::new();

    for pkg in &installed {
        solver.add_installed(&pkg.name, &pkg.version);
    }

    // TODO: Pobierz indeksy z repozytoriów i dodaj do solwera
    // Na razie symulacja:
    tracing::info!("Rozwiązywanie zależności...");

    // Pobieranie i instalacja
    let client = crate::repo::RepoClient::new(cfg)?;

    let install_reason = if as_deps {
        InstallReason::Dependency
    } else {
        InstallReason::Explicit
    };

    let total = packages.len() as u64;
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    for pkg_name in packages {
        pb.set_message(format!("Instalowanie {}...", pkg_name.bold()));

        if dry_run {
            println!("  {} instalowałby: {}", "→".yellow(), pkg_name.bold());
            pb.inc(1);
            continue;
        }

        // Sprawdź czy pakiet jest już zainstalowany
        if db.is_installed(pkg_name)? {
            println!(
                "  {} {} jest już zainstalowany",
                "⚠".yellow(),
                pkg_name.bold()
            );
            pb.inc(1);
            continue;
        }

        // Pobierz pakiet
        let cache_dir = std::path::PathBuf::from(&cfg.general.cache_path);
        std::fs::create_dir_all(&cache_dir)?;

        // Znajdź pakiet w repozytoriach
        for repo in cfg.active_repos() {
            if let Ok(index) = client.fetch_index(repo).await {
                for entry in &index.packages {
                    if entry.name == *pkg_name {
                        let pkg_path = cache_dir.join(&entry.filename);
                        client.download_package(repo, &entry.filename, &pkg_path).await?;

                        // Wczytaj i zainstaluj pakiet
                        let pkg = crate::package::read_package(&pkg_path)?;

                        // Weryfikuj sygnaturę
                        if cfg.security.require_signatures {
                            if let Some(ref sig) = pkg.header.pgp_signature {
                                tracing::debug!("Weryfikacja podpisu {}...", pkg_name);
                                // TODO: pełna weryfikacja GPG
                            }
                        }

                        // Rozpakuj
                        tracing::debug!("Rozpakowywanie {}...", pkg_name);
                        install_package_files(cfg, &pkg, overwrite)?;

                        // Dodaj do bazy danych
                        db.add_package(&pkg.header, install_reason)?;

                        // Historia
                        db.add_history("install", pkg_name, &pkg.header.version, "")?;

                        break;
                    }
                }
            }
        }

        pb.inc(1);
    }

    pb.finish_with_message("Instalacja zakończona");

    // Wyczyść cache jeśli autoclean
    if cfg.general.autoclean {
        let cache_dir = std::path::PathBuf::from(&cfg.general.cache_path);
        if cache_dir.exists() {
            for entry in std::fs::read_dir(&cache_dir)? {
                let entry = entry?;
                if entry.file_type()?.is_file() {
                    std::fs::remove_file(entry.path())?;
                }
            }
        }
    }

    Ok(())
}

/// Instaluje pliki pakietu na dysk
fn install_package_files(cfg: &Config, pkg: &crate::package::Package, overwrite: bool) -> anyhow::Result<()> {
    use std::io::Read;
    use flate2::read::GzDecoder;
    use xz2::read::XzDecoder;

    let root = cfg.general.root.trim_end_matches('/');

    // Dekompresuj payload
    let decompressed = match pkg.header.compression {
        crate::package::Compression::Zstd => {
            let decoder = zstd::stream::read::Decoder::new(&pkg.raw_payload[..])?;
            let mut data = Vec::new();
            let mut reader = std::io::BufReader::new(decoder);
            reader.read_to_end(&mut data)?;
            data
        }
        crate::package::Compression::Xz => {
            let mut decoder = XzDecoder::new(&pkg.raw_payload[..]);
            let mut data = Vec::new();
            decoder.read_to_end(&mut data)?;
            data
        }
        crate::package::Compression::Gzip => {
            let mut decoder = GzDecoder::new(&pkg.raw_payload[..]);
            let mut data = Vec::new();
            decoder.read_to_end(&mut data)?;
            data
        }
        crate::package::Compression::None => pkg.raw_payload.clone(),
    };

    // Rozpakuj tar
    let mut archive = tar::Archive::new(&decompressed[..]);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        let dest = format!("{}/{}", root, path.display());

        // Sprawdź czy plik już istnieje
        let dest_path = std::path::Path::new(&dest);
        if dest_path.exists() && !overwrite {
            tracing::warn!("Plik {} już istnieje, pomijam", dest);
            continue;
        }

        // Utwórz katalogi nadrzędne
        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Rozpakuj
        entry.unpack(dest_path)?;
    }

    Ok(())
}
