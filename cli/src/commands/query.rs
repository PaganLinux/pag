// Komenda: query
// Zapytania do bazy danych

use crate::cli_types::QueryCommand;
use crate::config::Config;
use colored::Colorize;

pub async fn execute(cfg: &Config, action: &QueryCommand) -> anyhow::Result<()> {
    let db = crate::db::PackageDb::open(&std::path::PathBuf::from(&cfg.general.db_path))?;

    match action {
        QueryCommand::Owner { file } => {
            match db.find_file_owner(file)? {
                Some(owner) => println!("{} {} należy do: {}", "📄".bold(), file, owner.bold()),
                None => println!("{} {} nie należy do żadnego pakietu.", "⚠".yellow(), file),
            }
        }
        QueryCommand::Files { package } => {
            let files = db.get_package_files(package)?;
            if files.is_empty() {
                println!("{} Brak plików lub pakiet nie istnieje.", "⚠".yellow());
            } else {
                println!("{} Pliki {} ({}):", "📁".bold(), package.bold(), files.len());
                for f in &files {
                    println!("  {}", f);
                }
            }
        }
        QueryCommand::Depends { package } => {
            if let Some(pkg) = db.get_package(package)? {
                // Pobierz zależności z dependencji
                println!("{} Zależności {}:", "🔗".bold(), package.bold());
                // TODO: dedykowana metoda do pobierania zależności
                let _ = pkg;
                println!("  (użyj --verbose dla szczegółów)");
            } else {
                println!("{} {} nie jest zainstalowany.", "⚠".yellow(), package);
            }
        }
        QueryCommand::RequiredBy { package } => {
            // Znajdź pakiety które zależą od tego
            println!("{} Pakiety zależne od {}:", "🔗".bold(), package.bold());
            // TODO: reverse dependency lookup
            println!("  (użyj --verbose dla szczegółów)");
        }
    }

    Ok(())
}
