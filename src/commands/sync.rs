// ────────────────────────────────────────────────────────────────────────────────
// Pag — Komenda synchronizacji repozytoriów
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::Result;
use colored::*;

use crate::config::Config;
use crate::cli::Cli;
use crate::repo::sync;
use crate::utils::net;

pub async fn run(config: &Config, force_refresh: bool, cli: &Cli) -> Result<()> {
    if !nix::unistd::Uid::effective().is_root() {
        anyhow::bail!("Repository sync requires root.");
    }

    let client = net::create_client(&config.network)?;

    println!("{}", "Synchronizing repository databases...".bold());

    let mut total_packages = 0usize;

    for repo in &config.repositories.repos {
        if !repo.enabled {
            continue;
        }

        print!("  {} ... ", repo.name.cyan());

        match sync::sync_repository(&client, repo, &config.paths.db_path, force_refresh).await {
            Ok(repo_db) => {
                let count = repo_db.packages.len();
                total_packages += count;
                println!("{} ({} packages)", "✓".green(), count);
            }
            Err(e) => {
                println!("{} {}", "✗".red(), e.to_string().red());
            }
        }
    }

    println!();
    println!("{} {} {}", "✓".green(), "Synchronized".bold(), format!("— {} packages total", total_packages).dimmed());

    Ok(())
}
