// Komenda: download
// Pobieranie pakietów bez instalacji

use crate::config::Config;
use colored::Colorize;

pub async fn execute(
    cfg: &Config,
    packages: &[String],
    output: &str,
) -> anyhow::Result<()> {
    if packages.is_empty() {
        anyhow::bail!("Nie podano pakietów do pobrania");
    }

    let db = crate::db::PackageDb::open(&std::path::PathBuf::from(&cfg.general.db_path))?;
    let client = crate::repo::RepoClient::new(cfg)?;
    let dest_dir = std::path::PathBuf::from(output);

    std::fs::create_dir_all(&dest_dir)?;

    for pkg_name in packages {
        if let Ok(Some((repo_name, filename))) = db.get_repo_package(pkg_name) {
            if let Some(repo) = cfg.repositories.iter().find(|r| r.name == repo_name) {
                let dest = dest_dir.join(&filename);
                client.download_package(repo, &filename, &dest).await?;
                println!("  {} {} -> {}", "✓".green(), pkg_name.bold(), dest.display());
                continue;
            }
        }
        println!("  {} {} nie znaleziono", "✗".red(), pkg_name);
    }

    Ok(())
}
