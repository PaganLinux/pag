// Moduł budowania pakietów
//
// Odpowiada za:
// 1. Przygotowanie środowiska build (chroot/kontener)
// 2. Pobieranie źródeł
// 3. Weryfikację checksum
// 4. Wykonanie build(), check(), package()
// 5. Pakowanie do .pag

use colored::*;
use std::path::{Path, PathBuf};

/// Główna funkcja budowania
pub async fn build(
    pagports_dir: &str,
    output_dir: &str,
    arch: &str,
    package: &str,
    clean: bool,
    skip_checksums: bool,
    skip_tests: bool,
    no_deps: bool,
    use_container: bool,
) -> anyhow::Result<()> {
    // Wczytaj pagbuild
    let pagbuild = crate::parser::load_from_pagports(pagports_dir, package)?;

    println!(
        "{} Budowanie {}/{} (arch: {})",
        "🔨".bold(),
        pagbuild.pkgname.bold(),
        pagbuild.pkgver.green(),
        arch
    );

    if let Some(ref maintainer) = pagbuild.maintainer {
        println!("  {} {}", "Opiekun:".dimmed(), maintainer);
    }

    // Przygotuj katalogi
    let srcdir = PathBuf::from(format!("src/{}", pagbuild.pkgname));
    let builddir = PathBuf::from(format!("build/{}", pagbuild.pkgname));
    let pkgdir = PathBuf::from(format!("pkg/{}", pagbuild.pkgname));

    if clean {
        println!("  {} Czyszczenie katalogów...", "🧹".dimmed());
        let _ = std::fs::remove_dir_all(&srcdir);
        let _ = std::fs::remove_dir_all(&builddir);
        let _ = std::fs::remove_dir_all(&pkgdir);
    }

    std::fs::create_dir_all(&srcdir)?;
    std::fs::create_dir_all(&builddir)?;
    std::fs::create_dir_all(&pkgdir)?;

    // 1. Pobierz źródła
    if !pagbuild.source.is_empty() {
        println!("  {} Pobieranie źródeł...", "📥".dimmed());
        download_sources(&pagbuild.source, &srcdir).await?;

        // Weryfikuj checksumy
        if !skip_checksums && !pagbuild.sha512sums.is_empty() {
            println!("  {} Weryfikacja checksum...", "🔐".dimmed());
            verify_source_checksums(&srcdir, &pagbuild.source, &pagbuild.sha512sums)?;
            println!("  {} Checksumy OK", "✓".green());
        }
    }

    // 2. Zainstaluj zależności build-time
    if !no_deps && !pagbuild.makedepends.is_empty() {
        println!("  {} Instalowanie zależności build-time...", "📦".dimmed());
        for dep in &pagbuild.makedepends {
            println!("    - {}", dep);
        }
        // TODO: uruchom `pag install --as-deps <makedepends>`
    }

    // 3. Uruchom build()
    if let Some(ref build_func) = pagbuild.build_func {
        println!("  {} Budowanie...", "⚙️".bold());
        if use_container {
            run_in_container(&pagbuild, build_func, &srcdir, &builddir, &pkgdir).await?;
        } else {
            run_build_script(&pagbuild, build_func, &srcdir, &builddir, &pkgdir)?;
        }
        println!("  {} Budowanie zakończone", "✓".green());
    }

    // 4. Uruchom check() (opcjonalnie)
    if !skip_tests {
        if let Some(ref check_func) = pagbuild.check_func {
            println!("  {} Testowanie...", "🧪".dimmed());
            run_check_script(&pagbuild, check_func, &srcdir, &builddir)?;
            println!("  {} Testy zakończone", "✓".green());
        }
    }

    // 5. Uruchom package()
    if let Some(ref package_func) = pagbuild.package_func {
        println!("  {} Pakowanie...", "📦".dimmed());
        run_package_script(&pagbuild, package_func, &srcdir, &builddir, &pkgdir)?;
        println!("  {} Pakowanie zakończone", "✓".green());
    }

    // 6. Stwórz plik .pag
    println!("  {} Tworzenie archiwum .pag...", "🗜️".dimmed());
    let pag_path = create_pag_package(&pagbuild, &pkgdir, output_dir, arch)?;
    println!(
        "  {} Utworzono: {}",
        "✓".green(),
        pag_path.display().to_string().bold()
    );

    Ok(())
}

/// Pobiera źródła
async fn download_sources(sources: &[String], srcdir: &Path) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    for (_i, source) in sources.iter().enumerate() {
        let source = source.trim();
        if source.is_empty() {
            continue;
        }

        // Jeśli to URL
        if source.starts_with("http://") || source.starts_with("https://") {
            let filename = source.split('/').last().unwrap_or("source");
            let dest = srcdir.join(filename);

            if dest.exists() {
                println!("    {} już pobrano", filename);
                continue;
            }

            println!("    Pobieranie {}...", filename);
            let response = client.get(source).send().await?;

            if !response.status().is_success() {
                anyhow::bail!("Nie udało się pobrać {}: HTTP {}", source, response.status());
            }

            let bytes = response.bytes().await?;
            std::fs::write(&dest, &bytes)?;
            println!("    ✓ {}", filename);
        } else {
            // To lokalny plik (np. patch)
            println!("    Używam lokalnego: {}", source);
        }
    }

    Ok(())
}

/// Weryfikuje sumy SHA512 źródeł
fn verify_source_checksums(srcdir: &Path, _sources: &[String], sums: &[String]) -> anyhow::Result<()> {
    for (_i, sum_line) in sums.iter().enumerate() {
        let sum_line = sum_line.trim();
        if sum_line.is_empty() {
            continue;
        }

        // Format: "sha512  nazwapliku"
        let parts: Vec<&str> = sum_line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let expected_hash = parts[0];
        let filename = parts[1];
        let file_path = srcdir.join(filename);

        if !file_path.exists() {
            tracing::warn!("Plik {} nie istnieje do weryfikacji", filename);
            continue;
        }

        let actual_hash = crate::crypto::sha512(&std::fs::read(&file_path)?);
        if actual_hash != expected_hash {
            anyhow::bail!(
                "Nieprawidłowa suma SHA512 dla {}.\n  Oczekiwano: {}\n  Otrzymano: {}",
                filename, expected_hash, actual_hash
            );
        }
    }

    Ok(())
}

/// Weryfikuje źródła dla pakietu (komenda verify)
pub fn verify_source(pagports_dir: &str, package: &str) -> anyhow::Result<()> {
    let pagbuild = crate::parser::load_from_pagports(pagports_dir, package)?;
    let srcdir = PathBuf::from(format!("src/{}", pagbuild.pkgname));

    if !srcdir.exists() {
        println!("  Najpierw pobierz źródła: pagbuild build {}", package);
        return Ok(());
    }

    if pagbuild.sha512sums.is_empty() {
        println!("  {} Brak checksum do weryfikacji", "⚠".yellow());
        return Ok(());
    }

    verify_source_checksums(&srcdir, &pagbuild.source, &pagbuild.sha512sums)?;
    println!("  {} Wszystkie checksumy poprawne", "✓".green());
    Ok(())
}

/// Uruchamia funkcję build w skrypcie bash
fn run_build_script(
    pagbuild: &crate::parser::PagBuild,
    _build_func: &str,
    srcdir: &Path,
    builddir: &Path,
    pkgdir: &Path,
) -> anyhow::Result<()> {
    // Generujemy tymczasowy skrypt bash z funkcją build
    let script = generate_full_script(pagbuild, srcdir, builddir, pkgdir, true, false, false)?;
    let script_path = std::env::temp_dir().join(format!("pagbuild-{}.sh", pagbuild.pkgname));

    std::fs::write(&script_path, &script)?;

    // Uruchamiamy
    let status = std::process::Command::new("bash")
        .arg(&script_path)
        .current_dir(srcdir.parent().unwrap_or(Path::new(".")))
        .status()?;

    if !status.success() {
        anyhow::bail!("Komenda build zakończyła się błędem");
    }

    Ok(())
}

/// Uruchamia funkcję check
fn run_check_script(
    pagbuild: &crate::parser::PagBuild,
    _check_func: &str,
    srcdir: &Path,
    builddir: &Path,
) -> anyhow::Result<()> {
    let script = generate_full_script(pagbuild, srcdir, builddir, &PathBuf::from("/tmp/pkg"), false, true, false)?;
    let script_path = std::env::temp_dir().join(format!("pagcheck-{}.sh", pagbuild.pkgname));

    std::fs::write(&script_path, &script)?;

    let status = std::process::Command::new("bash")
        .arg(&script_path)
        .current_dir(srcdir.parent().unwrap_or(Path::new(".")))
        .status()?;

    if !status.success() {
        anyhow::bail!("Testy nie przeszły");
    }

    Ok(())
}

/// Uruchamia funkcję package
fn run_package_script(
    pagbuild: &crate::parser::PagBuild,
    _package_func: &str,
    srcdir: &Path,
    builddir: &Path,
    pkgdir: &Path,
) -> anyhow::Result<()> {
    let script = generate_full_script(pagbuild, srcdir, builddir, pkgdir, false, false, true)?;
    let script_path = std::env::temp_dir().join(format!("pagpackage-{}.sh", pagbuild.pkgname));

    std::fs::write(&script_path, &script)?;

    let status = std::process::Command::new("bash")
        .arg(&script_path)
        .current_dir(srcdir.parent().unwrap_or(Path::new(".")))
        .status()?;

    if !status.success() {
        anyhow::bail!("Pakowanie zakończyło się błędem");
    }

    Ok(())
}

/// Generuje pełny skrypt bash z wszystkimi zmiennymi i funkcjami
fn generate_full_script(
    pagbuild: &crate::parser::PagBuild,
    srcdir: &Path,
    builddir: &Path,
    pkgdir: &Path,
    run_build: bool,
    run_check: bool,
    run_package: bool,
) -> anyhow::Result<String> {
    let mut script = String::new();

    script.push_str("#!/bin/bash\n");
    script.push_str("set -e\n\n");

    // Definicje zmiennych
    script.push_str(&format!("export srcdir=\"{}\"\n", srcdir.display()));
    script.push_str(&format!("export builddir=\"{}\"\n", builddir.display()));
    script.push_str(&format!("export pkgdir=\"{}\"\n", pkgdir.display()));
    script.push_str(&format!("export pkgname=\"{}\"\n", pagbuild.pkgname));
    script.push_str(&format!("export pkgver=\"{}\"\n", pagbuild.pkgver));
    script.push_str(&format!("export pkgrel=\"{}\"\n", pagbuild.pkgrel));

    // Dodatkowe zmienne
    for (key, value) in &pagbuild.extra_vars {
        script.push_str(&format!("export {}={}\n", key, value));
    }

    script.push('\n');

    // Funkcje
    if let Some(ref func) = pagbuild.build_func {
        script.push_str("build() {\n");
        script.push_str(func);
        script.push_str("\n}\n\n");
    }

    if let Some(ref func) = pagbuild.check_func {
        script.push_str("check() {\n");
        script.push_str(func);
        script.push_str("\n}\n\n");
    }

    if let Some(ref func) = pagbuild.package_func {
        script.push_str("package() {\n");
        script.push_str(func);
        script.push_str("\n}\n\n");
    }

    // Dodatkowe funkcje
    for (name, func) in &pagbuild.extra_funcs {
        script.push_str(&format!("{}() {{\n", name));
        script.push_str(func);
        script.push_str("\n}\n\n");
    }

    // Wywołanie
    if run_build {
        script.push_str("echo '>>> build()'\nbuild\n");
    }
    if run_check {
        script.push_str("echo '>>> check()'\ncheck\n");
    }
    if run_package {
        script.push_str("echo '>>> package()'\npackage\n");
    }

    Ok(script)
}

/// Budowanie w kontenerze (Docker/Podman)
async fn run_in_container(
    pagbuild: &crate::parser::PagBuild,
    _build_func: &str,
    srcdir: &Path,
    builddir: &Path,
    pkgdir: &Path,
) -> anyhow::Result<()> {
    println!("  {} Używam kontenera do budowania...", "🐳".dimmed());

    // Sprawdź dostępność Dockera/Podmana
    let runtime = if which::which("podman").is_ok() {
        "podman"
    } else if which::which("docker").is_ok() {
        "docker"
    } else {
        anyhow::bail!("Nie znaleziono Dockera ani Podmana. Zainstaluj jeden z nich lub użyj --no-container.");
    };

    let script = generate_full_script(pagbuild, srcdir, builddir, pkgdir, true, false, true)?;
    let script_path = std::env::temp_dir().join(format!("pagbuild-container-{}.sh", pagbuild.pkgname));
    std::fs::write(&script_path, &script)?;

    // TODO: Uruchom kontener z odpowiednim obrazem PaganLinux
    let status = std::process::Command::new(runtime)
        .args([
            "run",
            "--rm",
            "-v", &format!("{}:/workspace", std::env::current_dir()?.display()),
            "-w", "/workspace",
            "paganlinux:latest",
            "bash", &script_path.to_string_lossy(),
        ])
        .status()?;

    if !status.success() {
        anyhow::bail!("Budowanie w kontenerze zakończyło się błędem");
    }

    Ok(())
}

/// Tworzy archiwum .pag z plików w pkgdir
fn create_pag_package(
    pagbuild: &crate::parser::PagBuild,
    pkgdir: &Path,
    output_dir: &str,
    arch: &str,
) -> anyhow::Result<PathBuf> {
    std::fs::create_dir_all(output_dir)?;

    // Stwórz tar.zst plików
    let mut tar_data = Vec::new();
    {
        let mut tar = tar::Builder::new(&mut tar_data);
        add_files_to_tar(&mut tar, pkgdir, pkgdir)?;
        tar.finish()?;
    }

    // Kompresuj zstd
    let compressed = zstd::encode_all(&tar_data[..], 19)?; // poziom 19 (wysoka kompresja)

    // Utwórz nagłówek
    let mut header = crate::pag_package_header::PackageHeader::new(
        &pagbuild.pkgname,
        &pagbuild.pkgver,
        arch,
    );

    header.release = pagbuild.pkgrel;
    header.description = pagbuild.pkgdesc.clone();
    header.url = pagbuild.url.clone();
    header.license = pagbuild.license.clone();
    header.maintainer = pagbuild.maintainer.clone();
    header.arch = if pagbuild.arch == "all" { arch.to_string() } else { pagbuild.arch.clone() };
    header.depends = pagbuild.depends.clone();
    header.makedepends = pagbuild.makedepends.clone();
    header.provides = pagbuild.provides.clone();
    header.replaces = pagbuild.replaces.clone();
    header.conflicts = pagbuild.conflicts.clone();
    header.compressed_size = compressed.len() as u64;
    header.compression = crate::pag_package_header::Compression::Zstd;

    // Oblicz rozmiar i hashe
    header.sha512 = crate::crypto::sha512(&compressed);
    header.blake3 = crate::crypto::blake3(&compressed);

    // Oblicz rozmiar zainstalowanych plików
    header.installed_size = calculate_dir_size(pkgdir)?;

    // Pobierz listę plików
    header.files = collect_file_list(pkgdir, pkgdir)?;

    // Zapisz pakiet
    let filename = header.filename();
    let output_path = PathBuf::from(output_dir).join(&filename);

    let mut file = std::fs::File::create(&output_path)?;
    crate::pag_package_header::write_package_to_writer(&mut file, &header, &compressed)?;

    Ok(output_path)
}

/// Dodaje pliki z katalogu do archiwum tar
fn add_files_to_tar(tar: &mut tar::Builder<&mut Vec<u8>>, base: &Path, current: &Path) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(base)?;

        if path.is_dir() {
            // Dodaj katalog
            let mut header = tar::Header::new_gnu();
            header.set_path(relative)?;
            header.set_entry_type(tar::EntryType::Directory);
            header.set_mode(0o755);
            tar.append_data(&mut header, relative, &mut std::io::empty())?;

            add_files_to_tar(tar, base, &path)?;
        } else if path.is_symlink() {
            let target = std::fs::read_link(&path)?;
            let mut header = tar::Header::new_gnu();
            header.set_path(relative)?;
            header.set_entry_type(tar::EntryType::Symlink);
            header.set_link_name(&target)?;
            tar.append_data(&mut header, relative, &mut std::io::empty())?;
        } else {
            tar.append_path_with_name(&path, relative)?;
        }
    }
    Ok(())
}

/// Oblicza rozmiar katalogu
fn calculate_dir_size(dir: &Path) -> anyhow::Result<u64> {
    let mut total = 0u64;
    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            total += entry.metadata()?.len();
        }
    }
    Ok(total)
}

/// Zbiera listę plików z katalogu
fn collect_file_list(base: &Path, current: &Path) -> anyhow::Result<Vec<crate::pag_package_header::FileEntry>> {
    let mut files = Vec::new();

    for entry in walkdir::WalkDir::new(current).sort_by_file_name() {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(base)?;
        let rel_path = format!("/{}", relative.display());

        let file_type = if path.is_symlink() {
            crate::pag_package_header::FileType::Symlink
        } else if path.is_dir() {
            crate::pag_package_header::FileType::Dir
        } else {
            crate::pag_package_header::FileType::File
        };

        let size = if path.is_file() {
            entry.metadata()?.len()
        } else {
            0
        };

        let link_target = if path.is_symlink() {
            Some(std::fs::read_link(path)?.display().to_string())
        } else {
            None
        };

        let sha256 = if path.is_file() {
            Some(crate::crypto::sha256(&std::fs::read(path)?))
        } else {
            None
        };

        files.push(crate::pag_package_header::FileEntry {
            path: rel_path,
            size,
            mode: 0o755,
            uid: 0,
            gid: 0,
            file_type,
            link_target,
            sha256,
        });
    }

    Ok(files)
}

/// Czyści katalogi build
pub fn clean(_pagports_dir: &str, package: Option<&str>, all: bool) -> anyhow::Result<()> {
    let dirs = if all {
        vec!["src", "build", "pkg"]
    } else {
        vec!["build", "pkg"]
    };

    if let Some(pkg) = package {
        for d in &dirs {
            let path = PathBuf::from(format!("{}/{}", d, pkg));
            if path.exists() {
                std::fs::remove_dir_all(&path)?;
                println!("  {} Usunięto: {}", "✓".green(), path.display());
            }
        }
    } else {
        for d in &dirs {
            let path = PathBuf::from(d);
            if path.exists() {
                std::fs::remove_dir_all(&path)?;
                std::fs::create_dir_all(&path)?;
            }
        }
        println!("  {} Wyczyszczono katalogi build", "✓".green());
    }

    Ok(())
}
