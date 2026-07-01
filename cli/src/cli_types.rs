// Typy CLI - współdzielone między main.rs a commands/
// Wyodrębnione dla uniknięcia cyklicznych zależności

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Zainstaluj pakiety
    #[command(visible_alias = "i")]
    Install {
        packages: Vec<String>,
        #[arg(long = "as-deps")]
        as_deps: bool,
        #[arg(long)]
        overwrite: bool,
        #[arg(long)]
        ignore_deps: bool,
    },
    /// Usuń pakiety
    #[command(visible_alias = "r")]
    Remove {
        packages: Vec<String>,
        #[arg(short = 's', long)]
        recursive: bool,
        #[arg(short = 'n', long)]
        nosave: bool,
    },
    /// Zaktualizuj pakiety
    #[command(visible_alias = "u")]
    Update {
        #[arg(short = 'y', long)]
        refresh_only: bool,
        packages: Vec<String>,
    },
    /// Wyszukaj pakiety
    #[command(visible_alias = "s")]
    Search {
        query: Vec<String>,
        #[arg(short = 'v', long)]
        verbose: bool,
        #[arg(short = 'i', long)]
        installed: bool,
        #[arg(short = 'q', long)]
        quiet: bool,
    },
    /// Pokaż informacje o pakiecie
    #[command(visible_alias = "if")]
    Info {
        package: String,
        #[arg(short = 'l', long)]
        local: bool,
    },
    /// Wylistuj pakiety
    #[command(visible_alias = "ls")]
    List {
        #[arg(short = 'e', long)]
        explicit: bool,
        #[arg(short = 'd', long)]
        deps: bool,
        #[arg(short = 't', long)]
        orphans: bool,
        #[arg(short = 'g', long)]
        group: Option<String>,
    },
    /// Pobierz pakiety bez instalacji
    #[command(visible_alias = "dl")]
    Download {
        packages: Vec<String>,
        #[arg(short = 'o', long, default_value = ".")]
        output: String,
    },
    /// Sprawdź integralność
    #[command(visible_alias = "chk")]
    Check {
        packages: Vec<String>,
        #[arg(long)]
        fix: bool,
    },
    /// Wyczyść cache
    Clean {
        #[arg(short = 'a', long)]
        all: bool,
        #[arg(short = 'u', long)]
        unused: bool,
    },
    /// Zarządzanie repozytoriami
    Repo {
        #[command(subcommand)]
        action: RepoCommand,
    },
    /// Zarządzanie Flatpak
    Flatpak {
        #[command(subcommand)]
        action: FlatpakCommand,
    },
    /// Zarządzanie kluczami
    Key {
        #[command(subcommand)]
        action: KeyCommand,
    },
    /// Zapytania do bazy
    Query {
        #[command(subcommand)]
        action: QueryCommand,
    },
    /// Statystyki
    Stats,
    /// Generuj konfigurację
    Init {
        #[arg(short = 'c', long, default_value = "/etc/pag/config.toml")]
        config: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum RepoCommand {
    Add { name: String, url: String },
    Remove { name: String },
    List,
    Update { name: Option<String> },
}

#[derive(Subcommand, Debug)]
pub enum FlatpakCommand {
    Install { packages: Vec<String> },
    Remove { packages: Vec<String> },
    Search { query: Vec<String> },
    Update,
    List,
    RemoteAdd { name: String, url: String },
}

#[derive(Subcommand, Debug)]
pub enum KeyCommand {
    Import { keyfile: String },
    Export { keyid: String },
    List,
    Fetch { keyid: String },
    Remove { keyid: String },
}

#[derive(Subcommand, Debug)]
pub enum QueryCommand {
    Owner { file: String },
    Files { package: String },
    Depends { package: String },
    RequiredBy { package: String },
}
