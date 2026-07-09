// ────────────────────────────────────────────────────────────────────────────────
// Pag — Komenda aktualizacji (upgrade) z obsługą --sys
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use colored::*;
use dialoguer::Confirm;

use crate::config::Config;
use crate::cli::Cli;
use crate::db::localdb::LocalDatabase;
use crate::repo::{sync, download};
use crate::utils::net;

pub async fn run(
    config: &Config,
    sys_upgrade: bool,
    packages: &[String],
    ignore_sig: bool,
    download_only: bool,
    cli: &Cli,
) -> Result<()> {
    if !nix::unistd::Uid::effective().is_root() {
        anyhow::bail!("System upgrade requires root.");
    }

    let client = net::create_client(&config.network)?;

    // Aktualizacja systemu (rootfs + ISO)
    if sys_upgrade {
        return system_upgrade(config, &client, cli).await;
    }

    // Standardowa aktualizacja pakietów
    let db = LocalDatabase::open(&config.paths.db_path)?;

    // Synchronizuj repozytoria
    let mut all_dbs = Vec::new();
    for repo in &config.repositories.repos {
        if repo.enabled {
            match sync::sync_repository(&client, repo, &config.paths.db_path, false).await {
                Ok(repo_db) => all_dbs.push(repo_db),
                Err(e) => log::warn!("Failed to sync {}: {}", repo.name, e),
            }
        }
    }

    // Pobierz listę installedch pakietów
    let installed = db.list_packages()?;

    // Sprawdź dostępne aktualizacje
    let mut updates: Vec<(String, String, String)> = Vec::new(); // (nazwa, obecna, nowa)

    let target_packages: Vec<&str> = if packages.is_empty() {
        installed.iter().map(|p| p.name.as_str()).collect()
    } else {
        packages.iter().map(|s| s.as_str()).collect()
    };

    for pkg_name in &target_packages {
        // Znajdź installed pakiet
        let installed_pkg = installed.iter().find(|p| p.name == *pkg_name);
        if installed_pkg.is_none() {
            if !packages.is_empty() {
                log::warn!("Package is not installed: {}", pkg_name);
            }
            continue;
        }
        let installed_pkg = installed_pkg.unwrap();

        // Znajdź w repozytoriach
        if let Some((_repo_db, entry)) = sync::find_package(pkg_name, &all_dbs) {
            use crate::package::metadata::version_compare;

            let repo_version = format!("{}-{}", entry.version, entry.release);
            let local_version = installed_pkg.full_version();

            if version_compare(&repo_version, &local_version) == std::cmp::Ordering::Greater {
                updates.push((
                    pkg_name.to_string(),
                    local_version,
                    entry.version.clone(),
                ));
            }
        }
    }

    if updates.is_empty() {
        println!("{}", "System jest aktualny.".green().bold());
        return Ok(());
    }

    // Pokaż dostępne aktualizacje
    println!();
    println!("{}", "Available updates:".bold());
    for (name, old, new) in &updates {
        println!("  {} {} -> {}", name.cyan(), old.yellow(), new.green());
    }
    println!();

    if !cli.force && !cli.dry_run && !download_only {
        let confirm = Confirm::new()
            .with_prompt("Proceed with upgrade?")
            .default(true)
            .interact()?;
        if !confirm {
            return Ok(());
        }
    }

    if cli.dry_run {
        println!("{}", "[dry-run] Upgrade skipped.".dimmed());
        return Ok(());
    }

    // Pobierz i zainstaluj aktualizacje
    for (name, old_ver, _new_ver) in &updates {
        let (_repo_db, entry) = sync::find_package(name, &all_dbs)
            .context(format!("Failed to find package: {}", name))?;

        let pkg_path = download::download_package(
            &client,
            _repo_db,
            entry,
            config,
            &config.paths.pkg_cache_path,
        ).await?;

        if download_only {
            println!("{} Pobrano: {}", "✓".green(), pkg_path.display());
            continue;
        }

        log::info!("Upgrading {} ({} -> {})...", name, old_ver, entry.version);

        // Rozpakuj (nadpisując stare pliki)
        use crate::package::pag::PagArchive;
        let metadata = PagArchive::extract(&pkg_path, &std::path::Path::new("/"))?;

        // Aktualizuj wpis w bazie
        let mut meta = metadata;
        meta.repository = Some(_repo_db.name.clone());
        meta.install_reason = crate::package::metadata::InstallReason::Explicit;

        // Usuń stary wpis, dodaj nowy
        db.remove_package(name)?;
        db.add_package(&meta)?;

        println!("{} {} {} -> {}", "✓".green(), name.cyan(), old_ver.yellow(), entry.version.green());
    }

    if download_only {
        println!("{}", "Packages downloaded. Use 'pag upgrade' without --download-only to install.".bold());
    } else {
        println!("{}", "Upgrade complete.".bold());
    }

    Ok(())
}

/// Aktualizacja systemu (--sys): sprawdza nowe wydanie ISO/rootfs
async fn system_upgrade(config: &Config, client: &reqwest::Client, cli: &Cli) -> Result<()> {
    println!("{}", "Checking for new PaganLinux system version...".bold());

    // Sprawdź na serwerze
    let sys_version = download::check_system_version(client, config).await?;

    match sys_version {
        Some(version) => {
            // Sprawdź obecną wersję systemu
            let current_version = get_current_system_version()?;

            use crate::package::metadata::version_compare;
            if version_compare(&version.version, &current_version) != std::cmp::Ordering::Greater {
                println!("{} {}", "✓".green(), "System jest aktualny.".green());
                println!("  Obecna: {}", current_version);
                return Ok(());
            }

            println!();
            println!("{}", "=== New system version available! ===".green().bold());
            println!("  Obecna wersja: {}", current_version.yellow());
            println!("  Nowa wersja:   {}", version.version.green());
            if !version.description.is_empty() {
                println!("  Description: {}", version.description.dimmed());
            }
            if version.is_lts {
                println!("  {} {}", "LTS".blue().bold(), "Long-term support release".blue());
            }
            println!();

            if !cli.force && !cli.dry_run {
                let confirm = Confirm::new()
                    .with_prompt("Download and prepare system update? (reboot required)")
                    .default(false)
                    .interact()?;

                if !confirm {
                    println!("{}", "Aborted.".dimmed());
                    return Ok(());
                }
            }

            if cli.dry_run {
                println!("{}", "[dry-run] System upgrade skipped.".dimmed());
                return Ok(());
            }

            // Pobierz rootfs
            let rootfs_path = config.paths.cache_path.join("rootfs.tar.zst");
            download::download_system_rootfs(client, &version, &rootfs_path, config).await?;

            println!("{} {}", "✓".green(), format!("Pobrano rootfs: {} ({})",
                version.version,
                crate::utils::fs::human_size(version.rootfs_size)
            ));

            // Zapisz informację o aktualizacji
            let update_conf = format!(
                "version={}\nrootfs={}\nsha256={}\ndate={}\n",
                version.version,
                rootfs_path.display(),
                version.rootfs_sha256,
                version.release_date
            );
            std::fs::create_dir_all("/etc/pag")?;
            std::fs::write("/etc/pag/system_update.conf", update_conf)?;

            println!();
            println!("{}", "System update prepared!".green().bold());
            println!("  {}", "Reboot to complete the update.".yellow());
            println!("  {}", "Rootfs zostanie installed automatycznie podczas startu.".dimmed());
            println!();
            println!("  {}", "pag snapshot create --message 'before upgrade to {}'".replacen("{}", &version.version, 1).dimmed());
        }
        None => {
            println!("{}", "Cannot check system version — server unavailable.".yellow());
        }
    }

    Ok(())
}

/// Odczytaj obecną wersję systemu
fn get_current_system_version() -> Result<String> {
    // Spróbuj odczytać z /etc/pag-release
    if let Ok(content) = std::fs::read_to_string("/etc/pag-release") {
        return Ok(content.trim().to_string());
    }

    // Fallback: /etc/os-release
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("VERSION_ID=") {
                return Ok(line.trim_start_matches("VERSION_ID=")
                    .trim_matches('"')
                    .to_string());
            }
        }
    }

    Ok("unknown".to_string())
}
