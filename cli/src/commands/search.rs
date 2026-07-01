// Komenda: search
// Wyszukiwanie pakietów

use crate::config::Config;
use colored::Colorize;

pub async fn execute(
    cfg: &Config,
    query: &[String],
    verbose: bool,
    installed_only: bool,
    quiet: bool,
) -> anyhow::Result<()> {
    if query.is_empty() {
        anyhow::bail!("Podaj wzorzec wyszukiwania");
    }

    let query_str = query.join(" ");

    if installed_only {
        // Szukaj tylko w zainstalowanych
        let db = crate::db::PackageDb::open(&std::path::PathBuf::from(&cfg.general.db_path))?;
        let packages = db.list_packages(None)?;

        let results: Vec<_> = packages.iter()
            .filter(|p| p.name.contains(&query_str) || (verbose && p.description.contains(&query_str)))
            .collect();

        if quiet {
            for pkg in &results {
                println!("{}", pkg.name);
            }
        } else {
            println!("{} Wyniki dla \"{}\" (zainstalowane):", "::".bold(), query_str.bold());
            for pkg in &results {
                println!(
                    "  {}/{} {}",
                    pkg.name.bold(),
                    pkg.version.green(),
                    if verbose { format!(" - {}", pkg.description) } else { String::new() }
                );
            }
        }
    } else {
        // Szukaj w repozytoriach
        let db = crate::db::PackageDb::open(&std::path::PathBuf::from(&cfg.general.db_path))?;
        let results = db.search_repo_packages(&query_str, verbose)?;

        if quiet {
            let mut seen = std::collections::HashSet::new();
            for (_, name, _, _) in &results {
                if seen.insert(name) {
                    println!("{}", name);
                }
            }
        } else {
            println!("{} Wyniki dla \"{}\":", "::".bold(), query_str.bold());

            if results.is_empty() {
                println!("  {} Brak wyników.", "ℹ".cyan());
                // Zaproponuj flatpaka
                if crate::flatpak::is_flatpak_available() {
                    println!("  {} Spróbuj też: {} {}", "💡".yellow(), "pag flatpak search".bold(), query_str);
                }
            } else {
                for (repo, name, version, desc) in &results {
                    println!(
                        "  {}/{:<30} {:<15} {}",
                        repo.purple(),
                        name.bold(),
                        version.green(),
                        desc.dimmed()
                    );
                }
            }
        }
    }

    Ok(())
}
