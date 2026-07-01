// Komenda: repo - zarządzanie repozytoriami

use crate::cli_types::RepoCommand;
use crate::config::Config;
use colored::Colorize;

pub async fn execute(cfg: &Config, action: &RepoCommand) -> anyhow::Result<()> {
    match action {
        RepoCommand::Add { name, url } => {
            let mut new_cfg = cfg.clone();
            new_cfg.repositories.push(crate::config::Repository {
                name: name.clone(),
                url: url.clone(),
                mirror: None,
                enabled: true,
                priority: (new_cfg.repositories.len() * 10) as u32,
                arch: std::env::consts::ARCH.into(),
                branch: "main".into(),
            });
            new_cfg.save(&format!("{}/etc/pag/config.toml", cfg.general.root.trim_end_matches('/')))?;
            println!("{} Dodano repozytorium: {} ({})", "✓".green(), name.bold(), url);
        }
        RepoCommand::Remove { name } => {
            let mut new_cfg = cfg.clone();
            new_cfg.repositories.retain(|r| r.name != *name);
            new_cfg.save(&format!("{}/etc/pag/config.toml", cfg.general.root.trim_end_matches('/')))?;
            println!("{} Usunięto repozytorium: {}", "✓".green(), name.bold());
        }
        RepoCommand::List => {
            println!("{} Repozytoria:", "::".bold());
            for repo in &cfg.repositories {
                let status = if repo.enabled { "✓".green() } else { "✗".red() };
                println!(
                    "  {} {:<20} {:<50} prio:{}",
                    status, repo.name.bold(), repo.url.dimmed(), repo.priority
                );
            }
        }
        RepoCommand::Update { name } => {
            let client = crate::repo::RepoClient::new(cfg)?;
            let db = crate::db::PackageDb::open(&std::path::PathBuf::from(&cfg.general.db_path))?;

            let repos: Vec<&crate::config::Repository> = if let Some(n) = name {
                cfg.repositories.iter().filter(|r| r.name == *n).collect()
            } else {
                cfg.repositories.iter().filter(|r| r.enabled).collect()
            };

            for repo in &repos {
                print!("  {} {} ... ", "→".cyan(), repo.name.bold());
                match client.fetch_index(repo).await {
                    Ok(index) => {
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
                        println!("{}", "✓".green());
                    }
                    Err(e) => println!("{} ({})", "✗".red(), e),
                }
            }
        }
    }
    Ok(())
}
