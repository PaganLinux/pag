// Komenda: init
// Generowanie konfiguracji

use crate::config::Config;
use colored::Colorize;

pub async fn execute(_cfg: &Config, config_path: &str) -> anyhow::Result<()> {
    let default_config = Config::default();

    // Sprawdź czy plik już istnieje
    if std::path::Path::new(config_path).exists() {
        println!(
            "{} Plik {} już istnieje. Użyj {} aby nadpisać.",
            "⚠".yellow(),
            config_path,
            "--force".bold()
        );
        return Ok(());
    }

    default_config.save(config_path)?;

    println!("{} Wygenerowano konfigurację: {}", "✓".green(), config_path);
    println!();
    println!("  Repozytoria:");
    for repo in &default_config.repositories {
        println!("    • {} ({})", repo.name.bold(), repo.url);
    }
    println!();
    println!("  Aby rozpocząć:");
    println!("    {}       - zaktualizuj listę pakietów", "pag update".bold());
    println!("    {}      - zainstaluj pakiet", "pag install <nazwa>".bold());
    println!("    {}   - wyszukaj pakiety", "pag search <zapytanie>".bold());

    Ok(())
}
