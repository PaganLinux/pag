// Komenda: key - zarządzanie kluczami GPG

use crate::cli_types::KeyCommand;
use crate::config::Config;
use colored::Colorize;

pub async fn execute(cfg: &Config, action: &KeyCommand) -> anyhow::Result<()> {
    let keyring = std::path::PathBuf::from(&cfg.security.keyring_path);

    match action {
        KeyCommand::Import { keyfile } => {
            let data = std::fs::read(keyfile)?;
            crate::crypto::import_key(&data, &keyring)?;
            println!("{} Zaimportowano klucz z {}", "✓".green(), keyfile);
        }
        KeyCommand::Export { keyid } => {
            println!("{} Eksportowanie klucza {}...", "::".bold(), keyid);
        }
        KeyCommand::List => {
            if !keyring.exists() {
                println!("{} Brak kluczy w keyringu.", "ℹ".cyan());
                return Ok(());
            }
            println!("{} Zaufane klucze:", "::".bold());
        }
        KeyCommand::Fetch { keyid } => {
            println!("{} Pobieranie klucza {}...", "::".bold(), keyid);
            let client = reqwest::Client::new();
            let url = format!("https://keyserver.ubuntu.com/pks/lookup?op=get&search=0x{}", keyid);
            let response = client.get(&url).send().await?;
            let data = response.bytes().await?;
            crate::crypto::import_key(&data, &keyring)?;
            println!("{} Zaimportowano klucz {}", "✓".green(), keyid);
        }
        KeyCommand::Remove { keyid } => {
            println!("{} Usuwanie klucza {}...", "::".bold(), keyid);
        }
    }

    Ok(())
}
