// ────────────────────────────────────────────────────────────────────────────────
// Pag — Statystyki systemu
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::Result;
use colored::*;

use crate::config::Config;
use crate::cli::Cli;
use crate::db::localdb::LocalDatabase;
use crate::package::metadata::InstallReason;
use crate::utils::fs as fs_utils;

pub async fn run(config: &Config, _cli: &Cli) -> Result<()> {
    let db = LocalDatabase::open(&config.paths.db_path)?;
    let packages = db.list_packages()?;

    let explicit: Vec<_> = packages
        .iter()
        .filter(|p| p.install_reason == InstallReason::Explicit)
        .collect();
    let deps: Vec<_> = packages
        .iter()
        .filter(|p| p.install_reason == InstallReason::Dependency)
        .collect();

    let total_installed_size: u64 = packages.iter().map(|p| p.installed_size).sum();

    let cache_size = fs_utils::dir_size(&config.paths.pkg_cache_path);
    let build_size = fs_utils::dir_size(&config.paths.build_path);

    println!("{}", "=== PaganLinux Statistics ===".cyan().bold());
    println!();

    println!("{}", "Packages:".bold());
    println!("  Explicitly installed:  {}", explicit.len().to_string().cyan());
    println!("  Dependencies:            {}", deps.len().to_string().dimmed());
    println!("  Total:               {}", packages.len().to_string().green());
    println!();

    println!("{}", "Size:".bold());
    println!("  Installed packages: {}", fs_utils::human_size(total_installed_size));
    println!("  Package cache:        {}", fs_utils::human_size(cache_size));
    println!("  Build directory:     {}", fs_utils::human_size(build_size));
    println!();

    // Repozytoria
    println!("{}", "Repositories:".bold());
    for repo in &config.repositories.repos {
        let status = if repo.enabled {
            "enabled".green()
        } else {
            "disabled".red()
        };
        println!("  {} [{}] {}", repo.name.cyan(), status, repo.url.dimmed());
    }
    println!();

    // Migawki
    println!("{}", "Snapshots:".bold());
    if config.snapshots.enabled {
        match crate::snapshot::btrfs::list_snapshots(&config.paths.snapshot_path) {
            Ok(snapshots) => {
                println!("  Status:    {}", "enabled".green());
                println!("  Typ:       {}", config.snapshots.snapshot_type);
                println!("  Snapshots:   {}", snapshots.len());
            }
            Err(_) => {
                println!("  Status:    {}", "enabled, ale katalog nie istnieje".yellow());
            }
        }
    } else {
        println!("  Status:    {}", "disabled".red());
    }
    println!();

    // System
    println!("{}", "System:".bold());
    println!("  Architecture: {}", config.general.arch);
    println!("  Kompresja:     {}", config.general.compression);

    Ok(())
}
