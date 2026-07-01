// pagbuild - PaganLinux Build System
//
// Buduje pakiety .pag z pagports (fork aports)
// Pliki budowania: pagbuild (inspirowane PKGBUILD/PAGbuild)
//
// pagbuild czyta plik pagbuild, parsuje zmienne i funkcje,
// a następnie wykonuje proces budowania w kontenerze/chroot.

mod parser;
mod builder;
mod signer;
mod repo;
mod pag_package_header;
mod crypto;

use clap::{Parser, Subcommand};
use colored::*;

/// PaganLinux Build System
#[derive(Parser)]
#[command(
    name = "pagbuild",
    version = env!("CARGO_PKG_VERSION"),
    about = "Buduje pakiety .pag z pagports",
    after_help = "Przykłady:\n  \
                  pagbuild build 7zip\n  \
                  pagbuild build --clean 7zip\n  \
                  pagbuild sign 7zip-26.01-r2-x86_64.pag\n  \
                  pagbuild upload 7zip-26.01-r2-x86_64.pag"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Poziom szczegółowości
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Katalog z pagports
    #[arg(short = 'P', long, default_value = "./pagports")]
    pagports_dir: String,

    /// Katalog wyjściowy na pakiety .pag
    #[arg(short = 'o', long, default_value = "./packages")]
    output_dir: String,

    /// Architektura docelowa
    #[arg(short = 'a', long, default_value = "x86_64")]
    arch: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Zbuduj pakiet(y)
    Build {
        /// Nazwa pakietu (lub ścieżka do pliku pagbuild)
        package: String,

        /// Wyczyść katalog build przed budowaniem
        #[arg(long)]
        clean: bool,

        /// Pomiń sprawdzanie checksum
        #[arg(long)]
        skip_checksums: bool,

        /// Pomiń testy
        #[arg(long)]
        skip_tests: bool,

        /// Nie instaluj zależności build-time
        #[arg(long)]
        no_deps: bool,

        /// Użyj kontenera Docker/Podman zamiast chroot
        #[arg(long)]
        container: bool,
    },

    /// Podpisz pakiet .pag
    Sign {
        /// Plik .pag lub nazwa pakietu
        package: String,

        /// ID klucza GPG
        #[arg(short = 'k', long)]
        key: Option<String>,
    },

    /// Wyślij pakiet do repozytorium
    Upload {
        /// Plik(i) .pag
        packages: Vec<String>,

        /// Nazwa repozytorium docelowego
        #[arg(short = 'r', long, default_value = "core")]
        repo: String,

        /// URL serwera repo
        #[arg(long, default_value = "https://repos.paganlinux.eu")]
        server: String,

        /// Token API
        #[arg(long, env = "PAGBUILD_API_TOKEN")]
        token: Option<String>,
    },

    /// Parsuj plik pagbuild (bez budowania)
    Parse {
        /// Ścieżka do pliku pagbuild
        file: String,

        /// Format wyjścia (json, toml)
        #[arg(short = 'f', long, default_value = "json")]
        format: String,
    },

    /// Sprawdź integralność źródła (checksums)
    Verify {
        /// Nazwa pakietu
        package: String,
    },

    /// Wygeneruj szablon pagbuild
    New {
        /// Nazwa nowego pakietu
        name: String,

        /// Wersja pakietu
        #[arg(short = 'v', long, default_value = "0.1.0")]
        version: String,
    },

    /// Wyczyść katalogi build
    Clean {
        /// Nazwa pakietu (opcjonalnie)
        package: Option<String>,

        /// Wyczyść wszystko (src, build, packages)
        #[arg(short = 'a', long)]
        all: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(match cli.verbose {
            0 => "info",
            1 => "debug",
            _ => "trace",
        })
        .init();

    match &cli.command {
        Commands::Build { package, clean, skip_checksums, skip_tests, no_deps, container } => {
            builder::build(
                &cli.pagports_dir,
                &cli.output_dir,
                &cli.arch,
                package,
                *clean,
                *skip_checksums,
                *skip_tests,
                *no_deps,
                *container,
            ).await
        }
        Commands::Sign { package, key } => {
            signer::sign_package(package, key.as_deref(), &cli.output_dir)
        }
        Commands::Upload { packages, repo, server, token } => {
            repo::upload_packages(packages, repo, server, token.as_deref()).await
        }
        Commands::Parse { file, format } => {
            parser::parse_file(file, format)
        }
        Commands::Verify { package } => {
            builder::verify_source(&cli.pagports_dir, package)
        }
        Commands::New { name, version } => {
            generate_template(name, version)
        }
        Commands::Clean { package, all } => {
            builder::clean(&cli.pagports_dir, package.as_deref(), *all)
        }
    }
}

/// Generuje szablon pliku pagbuild
fn generate_template(name: &str, version: &str) -> anyhow::Result<()> {
    let dir = std::path::PathBuf::from(name);
    std::fs::create_dir_all(&dir)?;

    let template = format!(
        r#"# PaganLinux Build Script (pagbuild)
# Generated by pagbuild new

maintainer="Your Name <email@paganlinux.eu>"
pkgname={name}
pkgver={version}
pkgrel=1
pkgdesc="Short description of {name}"
url="https://example.com/{name}"
arch="all"
license="custom"
# makedepends="build-dependency1 build-dependency2"
# depends="runtime-dependency1"
# subpackages="$pkgname-doc"

source="https://example.com/{name}-$pkgver.tar.gz"
builddir="$srcdir/$pkgname-$pkgver"

build() {{
    cd "$builddir"
    ./configure --prefix=/usr
    make
}}

check() {{
    cd "$builddir"
    make check
}}

package() {{
    cd "$builddir"
    make DESTDIR="$pkgdir" install
}}

# sha512sums="
# ...
# "
"#,
        name = name,
        version = version,
    );

    let file_path = dir.join("pagbuild");
    std::fs::write(&file_path, template)?;

    println!("{} Utworzono szablon: {}", "✓".green(), file_path.display());
    println!("  Edytuj plik i uruchom: {}", "pagbuild build".bold());
    Ok(())
}
