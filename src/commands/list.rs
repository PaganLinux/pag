// ────────────────────────────────────────────────────────────────────────────────
// Pag — Listowanie installedch pakietów
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::Result;
use colored::*;

use crate::config::Config;
use crate::cli::Cli;
use crate::db::localdb::LocalDatabase;
use crate::package::metadata::InstallReason;
use crate::utils::fmt;

pub async fn run(
    config: &Config,
    explicit_only: bool,
    orphans: bool,
    cli: &Cli,
) -> Result<()> {
    let db = LocalDatabase::open(&config.paths.db_path)?;

    if orphans {
        let orphan_list = db.find_orphans()?;
        if orphan_list.is_empty() {
            println!("{}", "No orphaned packages.".green());
        } else {
            println!("{} ({}):", "Osierocone pakiety".yellow().bold(), orphan_list.len());
            for name in &orphan_list {
                println!("  {}", name.yellow());
            }
            println!();
            println!("{}", "Use 'pag remove --cascade <pkg>' to remove orphaned dependencies.".dimmed());
        }
        return Ok(());
    }

    let packages = db.list_packages()?;

    if packages.is_empty() {
        println!("{}", "No packages installed.".dimmed());
        return Ok(());
    }

    let filtered: Vec<_> = if explicit_only {
        packages
            .iter()
            .filter(|p| p.install_reason == InstallReason::Explicit)
            .collect()
    } else {
        packages.iter().collect()
    };

    let explicit_count = packages.iter()
        .filter(|p| p.install_reason == InstallReason::Explicit)
        .count();
    let dep_count = packages.len() - explicit_count;

    println!(
        "{} ({} explicit, {} dependencies):",
        "Installed packages".bold(),
        explicit_count,
        dep_count
    );
    println!();

    let headers = ["Name", "Version", "Reason", "Description"];
    let rows: Vec<Vec<String>> = filtered
        .iter()
        .map(|p| {
            let reason = match p.install_reason {
                InstallReason::Explicit => "explicit".cyan().to_string(),
                InstallReason::Dependency => "dependency".dimmed().to_string(),
            };
            vec![
                p.name.clone(),
                p.full_version(),
                reason,
                truncate(&p.description, 50),
            ]
        })
        .collect();

    fmt::print_table(
        &headers,
        &rows,
        &[colored::Color::Cyan, colored::Color::Green, colored::Color::White, colored::Color::White],
    );

    println!();
    println!("Total: {} packages", filtered.len());

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
