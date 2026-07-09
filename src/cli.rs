// ────────────────────────────────────────────────────────────────────────────────
// Pag — CLI type definitions (shared module)
// ────────────────────────────────────────────────────────────────────────────────

use clap::{Parser, Subcommand};
use colored::*;

/// 🐺 Pag — PaganLinux Package Manager
#[derive(Parser)]
#[command(
    name = "pag",
    version,
    about,
    long_about = None,
    after_help = format!(
        "Site: {}\nRepository: {}\nLicense: {}",
        "https://paganlinux.eu".cyan(),
        "https://github.com/PaganLinux/pag".cyan(),
        "GPL-3.0-or-later".cyan()
    )
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to config file
    #[arg(short = 'c', long = "config", global = true)]
    pub config_path: Option<String>,

    /// Working directory (default: /var/lib/pag)
    #[arg(short = 'r', long = "root", global = true)]
    pub root_dir: Option<String>,

    /// Disable colored output
    #[arg(long = "no-color", global = true)]
    pub no_color: bool,

    /// Quiet mode
    #[arg(short = 'q', long = "quiet", global = true)]
    pub quiet: bool,

    /// Verbose mode
    #[arg(short = 'v', long = "verbose", global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Interface language
    #[arg(short = 'l', long = "lang", global = true)]
    pub lang: Option<String>,

    /// Force operation
    #[arg(long = "force", global = true)]
    pub force: bool,

    /// Dry run mode
    #[arg(long = "dry-run", global = true)]
    pub dry_run: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install packages
    #[command(alias = "in")]
    Install {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(long = "as-deps")]
        as_deps: bool,
        #[arg(long = "overwrite")]
        overwrite: Vec<String>,
    },

    /// Remove packages
    #[command(alias = "rm")]
    Remove {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(long = "cascade", short = 's')]
        cascade: bool,
        #[arg(long = "no-save")]
        no_save: bool,
    },

    /// Upgrade packages or system
    #[command(alias = "up")]
    Upgrade {
        #[arg(long = "sys", short = 's')]
        sys: bool,
        packages: Vec<String>,
        #[arg(long = "ignore-sig")]
        ignore_sig: bool,
        #[arg(long = "download-only", short = 'w')]
        download_only: bool,
    },

    /// Sync repository databases
    #[command(alias = "sy")]
    Sync {
        #[arg(short = 'y', long = "refresh")]
        refresh: bool,
    },

    /// Search for packages
    #[command(alias = "se")]
    Search {
        #[arg(required = true)]
        query: Vec<String>,
        #[arg(short = 'd', long = "desc")]
        search_desc: bool,
        #[arg(short = 'q', long = "quiet")]
        names_only: bool,
    },

    /// Show package info
    #[command(alias = "if")]
    Info {
        #[arg(required = true)]
        packages: Vec<String>,
        #[arg(short = 'e', long = "extended")]
        extended: bool,
    },

    /// Build package from PAGBUILD
    Build {
        #[arg(default_value = ".")]
        path: String,
        #[arg(short = 'c', long = "clean")]
        clean: bool,
        #[arg(short = 'i', long = "install")]
        install: bool,
        #[arg(short = 's', long = "sign")]
        sign: bool,
    },

    /// Pokaż listę installedch pakietów
    #[command(alias = "ls")]
    ListInstalled {
        #[arg(short = 'e', long = "explicit")]
        explicit_only: bool,
        #[arg(short = 'o', long = "orphans")]
        orphans: bool,
    },

    /// Check package integrity
    Check {
        packages: Vec<String>,
        #[arg(long = "fix")]
        fix: bool,
    },

    /// Manage system snapshots
    Snapshot {
        #[command(subcommand)]
        action: SnapshotCommand,
    },

    /// Clean package cache
    Clean {
        #[arg(short = 'a', long = "all")]
        all: bool,
    },

    /// Manage GPG keys
    Keyring {
        #[command(subcommand)]
        action: KeyringCommand,
    },

    /// Show system statistics
    Stats,
}

#[derive(Subcommand)]
pub enum SnapshotCommand {
    Create {
        #[arg(short = 'm', long = "message")]
        message: Option<String>,
    },
    List,
    Restore {
        id: String,
    },
    Delete {
        id: String,
    },
}

#[derive(Subcommand)]
pub enum KeyringCommand {
    Import {
        path: String,
    },
    List,
    Remove {
        key_id: String,
    },
    Init,
}
