// ────────────────────────────────────────────────────────────────────────────────
// Pag — Komenda usuwania pakietów
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use colored::*;
use dialoguer::Confirm;

use crate::config::Config;
use crate::cli::Cli;
use crate::db::localdb::LocalDatabase;

pub async fn run(
    config: &Config,
    packages: &[String],
    cascade: bool,
    no_save: bool,
    cli: &Cli,
) -> Result<()> {
    if !nix::unistd::Uid::effective().is_root() {
        anyhow::bail!("Package removal requires root.");
    }

    let db = LocalDatabase::open(&config.paths.db_path)?;

    // Sprawdź które pakiety są zainstalowane
    let mut to_remove: Vec<String> = Vec::new();
    for pkg in packages {
        if db.is_installed(pkg)? {
            to_remove.push(pkg.clone());
        } else {
            log::warn!("Package is not installed: {}", pkg);
        }
    }

    if to_remove.is_empty() {
        println!("{}", "No packages to remove.".dimmed());
        return Ok(());
    }

    // Jeśli cascade, znajdź osierocone zależności
    let mut orphans = Vec::new();
    if cascade {
        for pkg in &to_remove {
            let deps = db.find_dependents(pkg)?;
            for dep in deps {
                if !to_remove.contains(&dep) {
                    orphans.push(dep);
                }
            }
        }
    }

    // Pokaż listę
    println!();
    println!("{}", "Packages to remove:".bold());
    for pkg in &to_remove {
        println!("  {} {}", "✗".red(), pkg.red());
    }
    if !orphans.is_empty() {
        println!();
        println!("{}", "Orphaned dependencies (--cascade):".yellow());
        for pkg in &orphans {
            println!("  {}", pkg.yellow());
        }
    }
    println!();

    // Potwierdzenie
    if !cli.force && !cli.dry_run {
        let confirm = Confirm::new()
            .with_prompt("Proceed with removal?")
            .default(false)
            .interact()?;

        if !confirm {
            println!("{}", "Aborted.".dimmed());
            return Ok(());
        }
    }

    if cli.dry_run {
        println!("{}", "[dry-run] Removal skipped.".dimmed());
        return Ok(());
    }

    // Usuń pakierty
    for pkg in &to_remove {
        if let Ok(Some(metadata)) = db.get_package(pkg) {
            // Usuń pliki (nie backupowe)
            for file in &metadata.files {
                if file.is_config && !no_save {
                    // Zapisz kopię .pagsave
                    let backup_path = format!("{}.pagsave", file.path);
                    let _ = std::fs::copy(&file.path, &backup_path);
                    log::info!("Saved backup: {}", backup_path);
                    continue;
                }
                if !file.is_config || no_save {
                    let _ = std::fs::remove_file(&file.path);
                }
            }

            // Usuń puste katalogi (od najgłębszych)
            let mut dirs: Vec<String> = metadata.files.iter()
                .filter_map(|f| {
                    let p = std::path::Path::new(&f.path);
                    p.parent().map(|d| d.display().to_string())
                })
                .collect();
            dirs.sort();
            dirs.dedup();
            dirs.reverse(); // najgłębsze najpierw

            for dir in dirs {
                if std::fs::read_dir(&dir).map(|mut d| d.next().is_none()).unwrap_or(false) {
                    let _ = std::fs::remove_dir(&dir);
                }
            }
        }

        db.remove_package(pkg)?;
        println!("{} {}", "✓".green(), format!("Removed: {}", pkg).green());
    }

    // Usuń osierocone
    for pkg in &orphans {
        if let Ok(Some(metadata)) = db.get_package(pkg) {
            for file in &metadata.files {
                if !file.is_config {
                    let _ = std::fs::remove_file(&file.path);
                }
            }
        }
        db.remove_package(pkg)?;
        println!("{} {}", "✓".green(), format!("Removed (orphan): {}", pkg).dimmed());
    }

    println!();
    println!("{}", "Removal complete.".bold());
    Ok(())
}
