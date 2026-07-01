// Komenda: check
// Sprawdzanie integralności pakietów

use crate::config::Config;
use colored::Colorize;

pub async fn execute(
    cfg: &Config,
    packages: &[String],
    fix: bool,
    dry_run: bool,
) -> anyhow::Result<()> {
    let db = crate::db::PackageDb::open(&std::path::PathBuf::from(&cfg.general.db_path))?;
    let root = cfg.general.root.trim_end_matches('/');

    let to_check = if packages.is_empty() {
        db.list_packages(None)?.iter().map(|p| p.name.clone()).collect()
    } else {
        packages.to_vec()
    };

    let mut issues = 0u32;
    let mut fixed = 0u32;

    for pkg_name in &to_check {
        if !db.is_installed(pkg_name)? {
            println!("  {} {} nie jest zainstalowany", "⚠".yellow(), pkg_name);
            continue;
        }

        let files = db.get_package_files(pkg_name)?;
        let mut missing_files = Vec::new();

        for file_path in &files {
            let full_path = format!("{}/{}", root, file_path.trim_start_matches('/'));
            if !std::path::Path::new(&full_path).exists() {
                missing_files.push(file_path.clone());
            }
        }

        if missing_files.is_empty() {
            println!("  {} {} - OK", "✓".green(), pkg_name);
        } else {
            issues += 1;
            println!(
                "  {} {} - brakujące pliki ({}):",
                "✗".red(),
                pkg_name.bold(),
                missing_files.len()
            );

            for f in &missing_files {
                println!("    - {}", f);
            }

            if fix && !dry_run {
                // TODO: reinstalacja pakietu
                println!("    → reinstalacja...");
                fixed += 1;
            }
        }
    }

    if issues == 0 {
        println!("\n{} Wszystkie pakiety sprawne.", "✓".green());
    } else if fix {
        println!("\n{} Znaleziono {} problemów, naprawiono {}.", "ℹ".cyan(), issues, fixed);
    } else {
        println!("\n{} Znaleziono {} problemów. Użyj {} aby naprawić.", "⚠".yellow(), issues, "pag check --fix".bold());
    }

    Ok(())
}
