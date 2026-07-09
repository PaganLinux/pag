// ────────────────────────────────────────────────────────────────────────────────
// Pag — Komenda instalacji
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use colored::*;
use dialoguer::Confirm;

use crate::config::Config;
use crate::cli::Cli;
use crate::db::localdb::LocalDatabase;
use crate::package::metadata::InstallReason;
use crate::package::pag::PagArchive;
use crate::repo::{sync, download};
use crate::utils::net;

pub async fn run(
    config: &Config,
    packages: &[String],
    as_deps: bool,
    overwrite: &[String],
    cli: &Cli,
) -> Result<()> {
    // Sprawdź uprawnienia roota
    if !nix::unistd::Uid::effective().is_root() {
        anyhow::bail!("Package installation requires root. Use sudo.");
    }

    let client = net::create_client(&config.network)?;
    let db = LocalDatabase::open(&config.paths.db_path)?;

    // Synchronizuj repozytoria
    let mut all_dbs = Vec::new();
    for repo in &config.repositories.repos {
        if repo.enabled {
            match sync::sync_repository(&client, repo, &config.paths.db_path, false).await {
                Ok(repo_db) => all_dbs.push(repo_db),
                Err(e) => log::warn!("Failed to sync repository {}: {}", repo.name, e),
            }
        }
    }

    // Rozwiąż zależności
    let mut to_install: Vec<(String, String)> = Vec::new(); // (nazwa, wersja)

    for pkg_name in packages {
        resolve_deps(pkg_name, &all_dbs, &db, &mut to_install)?;
    }

    if to_install.is_empty() {
        println!("{}", "All packages are already installed.".dimmed());
        return Ok(());
    }

    // Pokaż listę
    println!();
    println!("{}", "Packages to install:".bold());
    for (name, ver) in &to_install {
        let already = db.is_installed(name).unwrap_or(false);
        let status = if already { " (aktualizacja)".yellow() } else { " (nowy)".green() };
        println!("  {} {}{}", name.cyan(), ver, status);
    }
    println!();

    // Potwierdzenie
    if !cli.force && !cli.dry_run {
        let confirm = Confirm::new()
            .with_prompt("Proceed with installation?")
            .default(true)
            .interact()?;

        if !confirm {
            println!("{}", "Aborted.".dimmed());
            return Ok(());
        }
    }

    if cli.dry_run {
        println!("{}", "[dry-run] Installation skipped.".dimmed());
        return Ok(());
    }

    // Pobierz i zainstaluj
    for (name, _version) in &to_install {
        if db.is_installed(name)? {
            continue; // już zainstalowane, chyba że aktualizacja
        }

        // Znajdź pakiet w repozytoriach
        let (repo_db, entry) = sync::find_package(name, &all_dbs)
            .with_context(|| format!("Package not found in repositories: {}", name))?;

        // Pobierz pakiet
        let pkg_path = download::download_package(
            &client, repo_db, entry, config, &config.paths.pkg_cache_path,
        ).await?;

        // Rozpakuj
        log::info!("Installing {}...", name);
        let metadata = PagArchive::extract(&pkg_path, &Path::new("/"))?;

        // Zapisz w bazie
        let mut meta = metadata;
        meta.repository = Some(repo_db.name.clone());
        meta.install_reason = if as_deps {
            InstallReason::Dependency
        } else {
            InstallReason::Explicit
        };

        db.add_package(&meta)?;

        println!("{} {}", "✓".green(), format!("Installed: {} {}", name, entry.version).green());
    }

    println!();
    println!("{}", "Installation complete.".bold());
    Ok(())
}

/// Rekursywnie rozwiąż zależności
fn resolve_deps(
    pkg_name: &str,
    repos: &[crate::repo::sync::RepoDatabase],
    db: &LocalDatabase,
    result: &mut Vec<(String, String)>,
) -> Result<()> {
    // Sprawdź czy już na liście
    if result.iter().any(|(n, _)| n == pkg_name) {
        return Ok(());
    }

    // Sprawdź czy już zainstalowane (pomiń jeśli tak)
    if db.is_installed(pkg_name)? {
        return Ok(());
    }

    // Znajdź w repozytoriach
    let (_repo_db, entry) = sync::find_package(pkg_name, repos)
        .with_context(|| format!("Package not found: {}", pkg_name))?;

    // Najpierw zależności
    for dep in &entry.depends {
        let dep_name = crate::package::metadata::dep_name(dep);
        resolve_deps(dep_name, repos, db, result)?;
    }

    result.push((entry.name.clone(), entry.version.clone()));

    Ok(())
}

use std::path::Path;
