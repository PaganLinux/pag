mod build;
mod cli;
mod commands;
mod config;
mod crypto;
mod db;
mod i18n;
mod package;
mod repo;
mod snapshot;
mod utils;

use clap::Parser;
use colored::*;
use std::process;

use crate::cli::{Cli, Commands};
use crate::config::Config;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let log_level = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level)),
        )
        .with_target(false)
        .without_time()
        .init();
    let config = match Config::load(cli.config_path.as_deref()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} {}", "error:".red().bold(), e);
            process::exit(1);
        }
    };
    let result = match &cli.command {
        Commands::Install { packages, as_deps, overwrite } => {
            commands::install::run(&config, packages, *as_deps, overwrite, &cli).await
        }
        Commands::Remove { packages, cascade, no_save } => {
            commands::remove::run(&config, packages, *cascade, *no_save, &cli).await
        }
        Commands::Upgrade { sys, packages, ignore_sig, download_only } => {
            commands::upgrade::run(&config, *sys, packages, *ignore_sig, *download_only, &cli).await
        }
        Commands::Sync { refresh } => {
            commands::sync::run(&config, *refresh, &cli).await
        }
        Commands::Search { query, search_desc, names_only } => {
            commands::search::run(&config, query, *search_desc, *names_only, &cli).await
        }
        Commands::Info { packages, extended } => {
            commands::info::run(&config, packages, *extended, &cli).await
        }
        Commands::Build { path, clean, install, sign } => {
            commands::build::run(&config, path, *clean, *install, *sign, &cli).await
        }
        Commands::ListInstalled { explicit_only, orphans } => {
            commands::list::run(&config, *explicit_only, *orphans, &cli).await
        }
        Commands::Check { packages, fix } => {
            commands::check::run(&config, packages, *fix, &cli).await
        }
        Commands::Snapshot { action } => {
            commands::snapshot::run(&config, action, &cli).await
        }
        Commands::Clean { all } => {
            commands::clean::run(&config, *all, &cli).await
        }
        Commands::Keyring { action } => {
            commands::keyring::run(&config, action, &cli).await
        }
        Commands::Stats => {
            commands::stats::run(&config, &cli).await
        }
        Commands::Kernel { action } => {
            commands::kernel::run(&config, action, &cli).await
        }
    };
    if let Err(e) = result {
        if !cli.quiet {
            eprintln!("{} {}", "error:".red().bold(), e);
        }
        process::exit(1);
    }
}
