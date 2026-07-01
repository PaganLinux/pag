// PaganLinux Package Manager - Główny punkt wejścia

mod cli_types;
mod commands;
mod package;
mod repo;
mod db;
mod crypto;
mod flatpak;
mod deps;
mod i18n;
mod config;

use clap::Parser;
use cli_types::Commands;

#[derive(Parser)]
#[command(
    name = "pag",
    version = env!("CARGO_PKG_VERSION"),
    about = "PaganLinux Package Manager"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[arg(short, long)]
    quiet: bool,

    #[arg(short, long)]
    force: bool,

    #[arg(long)]
    dry_run: bool,

    #[arg(long, default_value = "/")]
    root: String,

    #[arg(long, default_value = "pl")]
    lang: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let filter = match cli.verbose {
        0 => if cli.quiet { "error" } else { "info" },
        1 => "debug",
        _ => "trace",
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();

    let _locale = i18n::init(&cli.lang);
    let cfg = config::Config::load(&cli.root)?;

    tracing::info!("PaganLinux Package Manager v{}", env!("CARGO_PKG_VERSION"));

    if !cli.dry_run {
        commands::check_permissions(&cli.root, &cli.command)?;
    }

    let result = commands::dispatch(&cfg, &cli.command, cli.dry_run).await;
    if let Err(e) = result {
        tracing::error!("{e}");
        std::process::exit(1);
    }

    Ok(())
}
