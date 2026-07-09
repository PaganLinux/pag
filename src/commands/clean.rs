// ────────────────────────────────────────────────────────────────────────────────
// Pag — Komenda czyszczenia
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use colored::*;

use crate::config::Config;
use crate::cli::Cli;
use crate::utils::fs as fs_utils;

pub async fn run(config: &Config, clean_all: bool, _cli: &Cli) -> Result<()> {
    if !nix::unistd::Uid::effective().is_root() {
        anyhow::bail!("Cleaning requires root.");
    }

    let mut total_freed: u64 = 0;

    // Wyczyść pamięć podręczną pakietów
    let pkg_cache_size = fs_utils::dir_size(&config.paths.pkg_cache_path);
    if pkg_cache_size > 0 {
        println!("{} {}", "Cleaning package cache:".bold(),
            fs_utils::human_size(pkg_cache_size));
        std::fs::remove_dir_all(&config.paths.pkg_cache_path)?;
        std::fs::create_dir_all(&config.paths.pkg_cache_path)?;
        total_freed += pkg_cache_size;
    }

    // Wyczyść katalog budowania
    if clean_all {
        let build_size = fs_utils::dir_size(&config.paths.build_path);
        if build_size > 0 {
            println!("{} {}", "Cleaning build directory:".bold(),
                fs_utils::human_size(build_size));
            std::fs::remove_dir_all(&config.paths.build_path)?;
            std::fs::create_dir_all(&config.paths.build_path)?;
            total_freed += build_size;
        }

        // Wyczyść tymczasowy katalog
        let tmp_size = fs_utils::dir_size(&config.paths.tmp_path);
        if tmp_size > 0 {
            println!("{} {}", "Cleaning temporary files:".bold(),
                fs_utils::human_size(tmp_size));
            std::fs::remove_dir_all(&config.paths.tmp_path)?;
            std::fs::create_dir_all(&config.paths.tmp_path)?;
            total_freed += tmp_size;
        }
    }

    if total_freed == 0 {
        println!("{}", "Nothing to clean.".dimmed());
    } else {
        println!();
        println!("{} {}", "✓".green(), format!("Freed: {}", fs_utils::human_size(total_freed)).green().bold());
    }

    Ok(())
}
