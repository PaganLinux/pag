// Komenda: clean
// Czyszczenie cache pakietów

use crate::config::Config;
use colored::Colorize;

pub async fn execute(
    cfg: &Config,
    all: bool,
    unused: bool,
) -> anyhow::Result<()> {
    let cache_dir = std::path::PathBuf::from(&cfg.general.cache_path);

    if !cache_dir.exists() {
        println!("{} Cache jest pusty.", "ℹ".cyan());
        return Ok(());
    }

    let mut removed_bytes = 0u64;
    let mut removed_files = 0u32;

    if all {
        // Usuń cały cache
        for entry in walkdir::WalkDir::new(&cache_dir).max_depth(2) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let meta = entry.metadata()?;
                removed_bytes += meta.len();
                std::fs::remove_file(entry.path())?;
                removed_files += 1;
            }
        }
    } else if unused {
        // Usuń tylko stare wersje
        let db = crate::db::PackageDb::open(&std::path::PathBuf::from(&cfg.general.db_path))?;
        let installed = db.list_packages(None)?;
        let installed_names: std::collections::HashSet<_> = installed.iter().map(|p| p.name.clone()).collect();

        for entry in walkdir::WalkDir::new(&cache_dir).max_depth(2) {
            let entry = entry?;
            if entry.file_type().is_file() {
                if let Some(stem) = entry.path().file_stem() {
                    let name = stem.to_string_lossy();
                    // Sprawdź czy nazwa pakietu jest w zainstalowanych
                    let pkg_name = name.split('-').next().unwrap_or(&name);
                    if !installed_names.contains(pkg_name) {
                        let meta = entry.metadata()?;
                        removed_bytes += meta.len();
                        std::fs::remove_file(entry.path())?;
                        removed_files += 1;
                    }
                }
            }
        }
    }

    println!(
        "{} Usunięto {} plików ({}).",
        "✓".green(),
        removed_files,
        human_size(removed_bytes)
    );

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
