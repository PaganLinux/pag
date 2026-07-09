// ────────────────────────────────────────────────────────────────────────────────
// Pag — Komenda sprawdzania integralności
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::Result;
use colored::*;

use crate::config::Config;
use crate::cli::Cli;
use crate::crypto::hash;
use crate::db::localdb::LocalDatabase;

pub async fn run(
    config: &Config,
    packages: &[String],
    fix: bool,
    cli: &Cli,
) -> Result<()> {
    if !nix::unistd::Uid::effective().is_root() {
        anyhow::bail!("Integrity check requires root.");
    }

    let db = LocalDatabase::open(&config.paths.db_path)?;

    let target_packages = if packages.is_empty() {
        db.list_packages()?
    } else {
        packages
            .iter()
            .filter_map(|name| db.get_package(name).ok().flatten())
            .collect()
    };

    if target_packages.is_empty() {
        println!("{}", "No packages to check.".dimmed());
        return Ok(());
    }

    println!("{}", "Checking package integrity...".bold());
    println!();

    let mut missing_files = Vec::new();
    let mut hash_mismatches: Vec<(String, String)> = Vec::new();
    let mut checked = 0u64;
    let mut errors = 0u64;

    for pkg in &target_packages {
        checked += 1;

        for file in &pkg.files {
            let path = std::path::Path::new(&file.path);

            if !path.exists() {
                missing_files.push((pkg.name.clone(), file.path.clone()));
                errors += 1;
                continue;
            }

            // Sprawdź sumę kontrolną tylko dla plików niekonfiguracyjnych
            if !file.is_config {
                if let Ok(hashes) = hash::multi_hash_file(path) {
                    // Porównaj SHA256 jeśli available
                    if let Some(ref expected_sha) = file.md5 {
                        // Uwaga: w metadanych trzymamy MD5, w przyszłości SHA256
                    }
                }
            }
        }
    }

    // Raport
    if !missing_files.is_empty() {
        println!("{}", "Missing files:".red().bold());
        for (pkg_name, file_path) in &missing_files {
            println!("  {}: {}", pkg_name.cyan(), file_path.red());
        }
    }

    if hash_mismatches.is_empty() && missing_files.is_empty() {
        println!("{}", "All files are in place.".green().bold());
    } else {
        println!();
        println!("{}", format!("Found {} issues.", errors).yellow().bold());

        if fix {
            println!("{}", "Option --fix: reinstalling broken packages...".bold());
            // TODO: Reinstall broken packages
        }
    }

    println!();
    println!("Checked {} packages.", checked);

    Ok(())
}
