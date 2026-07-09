// ────────────────────────────────────────────────────────────────────────────────
// Pag — Komenda budowania pakietu z PAGBUILD
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use colored::*;

use crate::build::compiler;
use crate::config::Config;
use crate::cli::Cli;
use crate::utils::net;

pub async fn run(
    config: &Config,
    path: &str,
    clean: bool,
    install: bool,
    sign: bool,
    cli: &Cli,
) -> Result<()> {
    let pagbuild_path = std::path::Path::new(path).join("PAGBUILD");

    if !pagbuild_path.exists() {
        anyhow::bail!(
            "PAGBUILD not found in: {}\nUse 'pag build <path>' or run in a PAGBUILD directory.",
            pagbuild_path.display()
        );
    }

    let client = net::create_client(&config.network)?;

    println!("{}", "=== Building package ===".bold().blue());
    println!("  PAGBUILD: {}", pagbuild_path.display().to_string().dimmed());

    let pkg_path = compiler::build_package(&pagbuild_path, config, &client, clean, sign).await?;

    println!();
    println!("{} {}", "✓".green().bold(), "Package built successfully!".green());
    println!("  {}", pkg_path.display());

    if install {
        println!();
        println!("{}", "Installing built package...".bold());

        if !nix::unistd::Uid::effective().is_root() {
            anyhow::bail!("Installation requires root. Use sudo.");
        }

        use crate::package::pag::PagArchive;
        let metadata = PagArchive::extract(&pkg_path, &std::path::Path::new("/"))?;

        let db = crate::db::localdb::LocalDatabase::open(&config.paths.db_path)?;
        db.add_package(&metadata)?;

        println!("{} {}", "✓".green(), format!("Installed: {} {}", metadata.name, metadata.version));
    }

    Ok(())
}
