// Komenda: update
// Aktualizacja listy pakietów i/lub pakietów

use crate::config::Config;
use colored::Colorize;

pub async fn execute(
    cfg: &Config,
    refresh_only: bool,
    packages: &[String],
    dry_run: bool,
) -> anyhow::Result<()> {
    let db = crate::db::PackageDb::open(&std::path::PathBuf::from(&cfg.general.db_path))?;
    let client = crate::repo::RepoClient::new(cfg)?;

    // 1. Odśwież indeksy repozytoriów
    println!("{} Synchronizacja baz pakietów...", "::".bold());

    for repo in cfg.active_repos() {
        print!("  {} {} ... ", "→".cyan(), repo.name.bold());

        match client.fetch_index(repo).await {
            Ok(index) => {
                // Konwertuj na RepoPackageEntry
                let entries: Vec<_> = index.packages.iter().map(|p| crate::package::RepoPackageEntry {
                    name: p.name.clone(),
                    version: p.version.clone(),
                    release: p.release,
                    arch: p.arch.clone(),
                    description: p.description.clone(),
                    installed_size: p.installed_size,
                    compressed_size: p.compressed_size,
                    depends: p.depends.clone(),
                    provides: p.provides.clone(),
                    conflicts: p.conflicts.clone(),
                    filename: p.filename.clone(),
                    sha512: p.sha512.clone(),
                    blake3: p.blake3.clone(),
                    pgp_signature: p.pgp_signature.clone(),
                }).collect();

                db.update_repo_packages(&repo.name, &entries)?;
                println!("{} ({} pakietów)", "✓".green(), entries.len());
            }
            Err(e) => {
                println!("{} ({})", "✗".red(), e);
            }
        }
    }

    if refresh_only {
        println!("\n{} Bazy pakietów zaktualizowane.", "✓".green());
        return Ok(());
    }

    // 2. Sprawdź dostępne aktualizacje
    let installed = db.list_packages(None)?;
    let mut updates: Vec<(String, String, String)> = Vec::new();

    for pkg in &installed {
        // Sprawdź czy są nowsze wersje w repo
        if let Ok(Some((repo_name, _filename))) = db.get_repo_package(&pkg.name) {
            // TODO: porównaj wersje
            let _ = repo_name;
        }
    }

    if updates.is_empty() {
        println!("\n{} System jest aktualny.", "✓".green());
        return Ok(());
    }

    // 3. Pokaż listę aktualizacji
    println!("\n{} Dostępne aktualizacje ({}):", "::".bold(), updates.len());
    for (name, old_ver, new_ver) in &updates {
        println!("  {} {} -> {}", name.bold(), old_ver.yellow(), new_ver.green());
    }

    if dry_run {
        return Ok(());
    }

    // 4. Wykonaj aktualizację
    // (implementacja podobna do install)

    Ok(())
}
