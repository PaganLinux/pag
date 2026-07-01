// Komenda: info
// Szczegółowe informacje o pakiecie

use crate::config::Config;
use colored::Colorize;

pub async fn execute(
    cfg: &Config,
    package: &str,
    local: bool,
) -> anyhow::Result<()> {
    let db = crate::db::PackageDb::open(&std::path::PathBuf::from(&cfg.general.db_path))?;

    if let Some(pkg) = db.get_package(package)? {
        // Pakiet jest zainstalowany
        println!("{} {}/{}", "📦".bold(), pkg.name.bold(), pkg.version.green());
        println!("  {} {}", "Opis:".bold(), pkg.description);
        if let Some(ref url) = pkg.url {
            println!("  {} {}", "URL:".bold(), url.underline());
        }
        println!("  {} {}", "Architektura:".bold(), pkg.arch);
        println!("  {} {}", "Licencja:".bold(), pkg.license);
        println!("  {} {}", "Rozmiar:".bold(), human_size(pkg.installed_size));
        if let Some(ref maintainer) = pkg.maintainer {
            println!("  {} {}", "Opiekun:".bold(), maintainer);
        }

        let reason = match pkg.install_reason {
            crate::db::InstallReason::Explicit => "jawnie zainstalowany",
            crate::db::InstallReason::Dependency => "zależność",
        };
        println!("  {} {}", "Powód instalacji:".bold(), reason);

        // Pliki
        let files = db.get_package_files(package)?;
        println!("  {} ({} plików):", "Pliki:".bold(), files.len());
        for file in files.iter().take(20) {
            println!("    {}", file);
        }
        if files.len() > 20 {
            println!("    ... i {} więcej", files.len() - 20);
        }

        return Ok(());
    }

    if local {
        println!("{} Pakiet {} nie jest zainstalowany.", "✗".red(), package.bold());
        return Ok(());
    }

    // Spróbuj znaleźć w repozytoriach
    if let Ok(Some((repo_name, filename))) = db.get_repo_package(package) {
        println!("{} {}/{} [{}]", "📦".bold(), package.bold(), "?".yellow(), repo_name.purple());
        println!("  {} {}", "Plik:".bold(), filename);
        println!("  {} {}", "Repozytorium:".bold(), repo_name);
    } else {
        println!("{} Pakiet {} nie został znaleziony.", "✗".red(), package.bold());
    }

    Ok(())
}

fn human_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    format!("{:.2} {}", size, UNITS[unit])
}
