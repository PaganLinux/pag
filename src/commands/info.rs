// ────────────────────────────────────────────────────────────────────────────────
// Pag — Komenda informacji o pakiecie
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::Result;
use colored::*;

use crate::config::Config;
use crate::cli::Cli;
use crate::db::localdb::LocalDatabase;
use crate::repo::sync;
use crate::utils::fmt;

pub async fn run(
    config: &Config,
    packages: &[String],
    extended: bool,
    cli: &Cli,
) -> Result<()> {
    let db = LocalDatabase::open(&config.paths.db_path)?;
    let dbs = sync::load_all_dbs(&config.paths.db_path)?;

    for pkg_name in packages {
        println!("{}", "─".repeat(60).dimmed());

        // Sprawdź lokalną bazę
        if let Ok(Some(local_meta)) = db.get_package(pkg_name) {
            print_package_info(&local_meta.name, &local_meta.version, &local_meta.description,
                &local_meta.architecture, local_meta.installed_size,
                &local_meta.license, &local_meta.depends,
                local_meta.url.as_deref(),
                Some("installed".green().to_string()),
                extended);
        }

        // Sprawdź repozytoria
        if let Some((repo_db, entry)) = sync::find_package(pkg_name, &dbs) {
            let status = if db.is_installed(pkg_name).unwrap_or(false) {
                format!("installed [{}]", repo_db.name)
            } else {
                format!("available [{}]", repo_db.name)
            };

            print_package_info(
                &entry.name,
                &entry.version,
                &entry.description,
                &entry.architecture,
                entry.installed_size,
                &entry.licenses,
                &entry.depends,
                entry.url.as_deref(),
                Some(status),
                extended,
            );
            continue;
        }

        if !db.is_installed(pkg_name).unwrap_or(false) {
            println!("{} {}", "✗".red(), format!("Package not found: {}", pkg_name).red());
        }
    }

    Ok(())
}

fn print_package_info(
    name: &str,
    version: &str,
    desc: &str,
    arch: &str,
    size: u64,
    licenses: &[String],
    depends: &[String],
    url: Option<&str>,
    status: Option<String>,
    extended: bool,
) {
    println!("{} {} {}", "Name:".bold(), name.cyan(), version.green());
    println!("  {} {}", "Description:".bold(), desc);

    if let Some(ref s) = status {
        println!("  {} {}", "Status:".bold(), s);
    }

    println!("  {} {}", "Architecture:".bold(), arch);
    println!("  {} {}", "Size:".bold(), crate::utils::fs::human_size(size));

    if !licenses.is_empty() {
        println!("  {} {}", "Licenses:".bold(), licenses.join(", "));
    }

    if let Some(u) = url {
        println!("  {} {}", "URL:".bold(), u.underline());
    }

    if !depends.is_empty() {
        println!("  {} {}", "Dependencies:".bold(), depends.join(" "));
    }

    if extended {
        println!();
    }
}
