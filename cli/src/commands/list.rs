// Komenda: list
// Listowanie pakietów

use crate::config::Config;
use crate::db::InstallReason;
use colored::Colorize;

pub async fn execute(
    cfg: &Config,
    explicit: bool,
    deps: bool,
    orphans: bool,
    group: Option<&str>,
) -> anyhow::Result<()> {
    let db = crate::db::PackageDb::open(&std::path::PathBuf::from(&cfg.general.db_path))?;

    if orphans {
        let orphan_list = db.find_orphans()?;
        if orphan_list.is_empty() {
            println!("{} Brak osieroconych pakietów.", "✓".green());
        } else {
            println!("{} Osierocone pakiety ({}):", "::".bold(), orphan_list.len());
            for name in &orphan_list {
                println!("  {}", name);
            }
        }
        return Ok(());
    }

    let reason = if explicit {
        Some(InstallReason::Explicit)
    } else if deps {
        Some(InstallReason::Dependency)
    } else {
        None
    };

    let packages = db.list_packages(reason)?;

    if let Some(g) = group {
        let filtered: Vec<_> = packages.iter()
            .filter(|p| p.group_name.as_deref() == Some(g))
            .collect();

        println!("{} Pakiety z grupy \"{}\" ({}):", "::".bold(), g.bold(), filtered.len());
        for pkg in &filtered {
            println!("  {} {}", pkg.name.bold(), pkg.version.green());
        }
    } else {
        let label = if explicit {
            "Jawnie zainstalowane"
        } else if deps {
            "Zależności"
        } else {
            "Wszystkie zainstalowane"
        };

        println!("{} {} ({}):", "::".bold(), label, packages.len());
        for pkg in &packages {
            let marker = match pkg.install_reason {
                InstallReason::Explicit => "",
                InstallReason::Dependency => " [dep]",
            };
            println!(
                "  {} {}{}",
                pkg.name.bold(),
                pkg.version.green(),
                marker.dimmed()
            );
        }
    }

    Ok(())
}
