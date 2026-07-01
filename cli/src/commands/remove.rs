// Komenda: remove
// Usuwanie pakietów

use crate::config::Config;
use colored::Colorize;

pub async fn execute(
    cfg: &Config,
    packages: &[String],
    recursive: bool,
    nosave: bool,
    dry_run: bool,
) -> anyhow::Result<()> {
    if packages.is_empty() {
        anyhow::bail!("Nie podano pakietów do usunięcia");
    }

    let db = crate::db::PackageDb::open(&std::path::PathBuf::from(&cfg.general.db_path))?;

    let mut to_remove = packages.to_vec();

    // Jeśli recursive, znajdź też zależności do usunięcia
    if recursive {
        let orphans = db.find_orphans()?;
        to_remove.extend(orphans);
        to_remove.sort();
    }

    for pkg_name in &to_remove {
        if !db.is_installed(pkg_name)? {
            println!("  {} {} nie jest zainstalowany", "⚠".yellow(), pkg_name);
            continue;
        }

        if dry_run {
            println!("  {} usunąłby: {}", "→".yellow(), pkg_name.bold());
            continue;
        }

        // Pobierz listę plików
        let files = db.get_package_files(pkg_name)?;
        let root = cfg.general.root.trim_end_matches('/');

        // Usuń pliki (ale zachowaj konfigurację jeśli nosave)
        for file_path in &files {
            let full_path = format!("{}/{}", root, file_path.trim_start_matches('/'));
            let path = std::path::Path::new(&full_path);

            if path.exists() {
                if nosave && is_config_file(file_path) {
                    println!("  {} zachowano konfigurację: {}", "ℹ".cyan(), file_path);
                    continue;
                }

                if path.is_dir() {
                    // Nie usuwaj katalogów które mogą być współdzielone
                    if !is_shared_dir(file_path) {
                        let _ = std::fs::remove_dir(path);
                    }
                } else {
                    std::fs::remove_file(path)?;
                }
            }
        }

        // Usuń z bazy danych
        db.remove_package(pkg_name)?;
        db.add_history("remove", pkg_name, "", "")?;

        println!("  {} usunięto: {}", "✓".green(), pkg_name.bold());
    }

    // Wyświetl informację o osieroconych zależnościach
    if !recursive {
        let orphans = db.find_orphans()?;
        if !orphans.is_empty() {
            println!();
            println!("  {} Osierocone pakiety ({} szt.):", "ℹ".cyan(), orphans.len());
            for orphan in &orphans {
                println!("    • {}", orphan);
            }
            println!("  Użyj {} aby je usunąć.", "pag remove --recursive".bold());
        }
    }

    Ok(())
}

fn is_config_file(path: &str) -> bool {
    path.starts_with("/etc/") && !path.ends_with('/')
}

fn is_shared_dir(path: &str) -> bool {
    let shared_dirs = [
        "/usr/", "/usr/bin/", "/usr/lib/", "/usr/share/",
        "/etc/", "/var/", "/opt/", "/bin/", "/lib/", "/sbin/",
    ];
    shared_dirs.iter().any(|d| path == *d)
}
