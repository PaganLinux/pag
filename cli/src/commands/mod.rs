// Dispatcher komend i sprawdzanie uprawnień

use crate::cli_types::{Commands, FlatpakCommand, KeyCommand, QueryCommand, RepoCommand};
use crate::config::Config;

pub async fn dispatch(cfg: &Config, command: &Commands, dry_run: bool) -> anyhow::Result<()> {
    match command {
        Commands::Install { packages, as_deps, overwrite, ignore_deps } => {
            install::execute(cfg, packages, *as_deps, *overwrite, *ignore_deps, dry_run).await
        }
        Commands::Remove { packages, recursive, nosave } => {
            remove::execute(cfg, packages, *recursive, *nosave, dry_run).await
        }
        Commands::Update { refresh_only, packages } => {
            update::execute(cfg, *refresh_only, packages, dry_run).await
        }
        Commands::Search { query, verbose, installed, quiet } => {
            search::execute(cfg, query, *verbose, *installed, *quiet).await
        }
        Commands::Info { package, local } => {
            info::execute(cfg, package, *local).await
        }
        Commands::List { explicit, deps, orphans, group } => {
            list::execute(cfg, *explicit, *deps, *orphans, group.as_deref()).await
        }
        Commands::Download { packages, output } => {
            download::execute(cfg, packages, output).await
        }
        Commands::Check { packages, fix } => {
            check::execute(cfg, packages, *fix, dry_run).await
        }
        Commands::Clean { all, unused } => {
            clean::execute(cfg, *all, *unused).await
        }
        Commands::Repo { action } => repo::execute(cfg, action).await,
        Commands::Flatpak { action } => flatpak::execute(cfg, action).await,
        Commands::Key { action } => key::execute(cfg, action).await,
        Commands::Query { action } => query::execute(cfg, action).await,
        Commands::Stats => stats::execute(cfg).await,
        Commands::Init { config } => init::execute(cfg, config).await,
    }
}

pub fn check_permissions(root: &str, command: &Commands) -> anyhow::Result<()> {
    let needs_root = matches!(
        command,
        Commands::Install { .. }
            | Commands::Remove { .. }
            | Commands::Update { .. }
            | Commands::Check { fix: true, .. }
    );

    if needs_root && root == "/" {
        if unsafe { libc::geteuid() } != 0 {
            anyhow::bail!("Ta operacja wymaga uprawnień roota. Użyj sudo.");
        }
    }
    Ok(())
}

pub mod install;
pub mod remove;
pub mod update;
pub mod search;
pub mod info;
pub mod list;
pub mod download;
pub mod check;
pub mod clean;
pub mod repo;
pub mod flatpak;
pub mod key;
pub mod query;
pub mod stats;
pub mod init;
