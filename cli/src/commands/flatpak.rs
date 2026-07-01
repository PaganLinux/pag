// Komenda: flatpak - zarządzanie flatpakami

use crate::cli_types::FlatpakCommand;
use crate::config::Config;
use colored::Colorize;

pub async fn execute(_cfg: &Config, action: &FlatpakCommand) -> anyhow::Result<()> {
    if !crate::flatpak::is_flatpak_available() {
        anyhow::bail!("Flatpak nie jest zainstalowany. Zainstaluj: pag install flatpak");
    }

    match action {
        FlatpakCommand::Install { packages } => {
            crate::flatpak::install(packages, None)?;
            for pkg in packages {
                println!("  {} zainstalowano flatpak: {}", "✓".green(), pkg.bold());
            }
        }
        FlatpakCommand::Remove { packages } => {
            crate::flatpak::remove(packages)?;
            for pkg in packages {
                println!("  {} usunięto flatpak: {}", "✓".green(), pkg.bold());
            }
        }
        FlatpakCommand::Search { query } => {
            let results = crate::flatpak::search(query)?;
            if results.is_empty() {
                println!("{} Brak wyników.", "ℹ".cyan());
            } else {
                println!("{} Znalezione flatpaki:", "::".bold());
                for info in &results {
                    println!(
                        "  {:<30} {:<15} {}",
                        info.name.bold(),
                        info.version.green(),
                        info.description.dimmed()
                    );
                }
            }
        }
        FlatpakCommand::Update => {
            println!("{} Aktualizowanie flatpaków...", "::".bold());
            crate::flatpak::update()?;
            println!("{} Flatpaki zaktualizowane.", "✓".green());
        }
        FlatpakCommand::List => {
            let installed = crate::flatpak::list_installed()?;
            if installed.is_empty() {
                println!("{} Brak zainstalowanych flatpaków.", "ℹ".cyan());
            } else {
                println!("{} Zainstalowane flatpaki ({}):", "::".bold(), installed.len());
                for info in &installed {
                    println!(
                        "  {:<30} {:<15} [{}]",
                        info.name.bold(),
                        info.version.green(),
                        info.branch.dimmed()
                    );
                }
            }
        }
        FlatpakCommand::RemoteAdd { name, url } => {
            crate::flatpak::add_remote(name, url)?;
            println!("{} Dodano flatpak remote: {}", "✓".green(), name.bold());
        }
    }

    Ok(())
}
