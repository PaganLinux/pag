// ────────────────────────────────────────────────────────────────────────────────
// Pag — Komenda zarządzania kluczami GPG
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use colored::*;
use std::path::Path;

use crate::config::Config;
use crate::crypto::gpg;
use crate::cli::{Cli, KeyringCommand};

pub async fn run(config: &Config, action: &KeyringCommand, _cli: &Cli) -> Result<()> {
    if !nix::unistd::Uid::effective().is_root() {
        anyhow::bail!("Keyring management requires root.");
    }

    match action {
        KeyringCommand::Init => {
            println!("{}", "Initializing GPG keyring...".bold());
            gpg::init_keyring(&config.gpg)?;
            println!("{} {}", "✓".green(), "Keyring initialized.".green());
        }

        KeyringCommand::Import { path } => {
            let key_path = Path::new(path);

            if !key_path.exists() {
                // Spróbuj pobrać z keyservera
                println!("{}", format!("Fetching key {} from {}...", path, config.gpg.keyserver).bold());
                let result = gpg::recv_key(path, &config.gpg.keyserver, &config.paths.keyring_path)?;
                println!("{} {}", "✓".green(), result);
            } else {
                println!("{}", format!("Importing key: {}", path).bold());
                let result = gpg::import_key(key_path, &config.paths.keyring_path)?;
                println!("{} {}", "✓".green(), result);
            }
        }

        KeyringCommand::List => {
            let keys = gpg::list_keys(&config.paths.keyring_path)?;

            if keys.is_empty() {
                println!("{}", "Keyring is empty.".dimmed());
                println!("{}", "Import keys: pag keyring import <path-or-id>".dimmed());
                return Ok(());
            }

            println!("{}", "=== Trusted GPG Keys ===".bold());
            println!();

            for key in &keys {
                println!("{}", "─".repeat(50).dimmed());
                if let Some(ref fp) = key.fingerprint {
                    println!("  Fingerprint: {}", fp.yellow());
                }
                if let Some(ref id) = key.key_id {
                    println!("  ID:          {}", id.cyan());
                }
                for uid in &key.uids {
                    println!("  UID:         {}", uid);
                }
                if let Some(ref created) = key.created {
                    println!("  Utworzony:   {}", created);
                }
                println!();
            }

            println!("Total: {} kluczy", keys.len());
        }

        KeyringCommand::Remove { key_id } => {
            println!("{}", format!("Removing key: {}", key_id).bold());
            gpg::remove_key(key_id, &config.paths.keyring_path)?;
            println!("{} {}", "✓".green(), "Key removed.".green());
        }
    }

    Ok(())
}
