// ────────────────────────────────────────────────────────────────────────────────
// Pag — Zarządzanie kluczami GPG
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use crate::config::GpgConfig;

/// Zainicjalizuj keyring GPG
pub fn init_keyring(config: &GpgConfig) -> Result<()> {
    let keyring_dir = if config.keyring_dir.is_empty() {
        Path::new("/etc/pag/gnupg")
    } else {
        Path::new(&config.keyring_dir)
    };

    std::fs::create_dir_all(keyring_dir)?;

    // Ustaw uprawnienia 700
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(keyring_dir, std::fs::Permissions::from_mode(0o700))?;
    }

    // Sprawdź czy gpg jest available
    let output = Command::new("gpg")
        .arg("--version")
        .output()
        .context("GPG (gnupg) is not installed. Install with: pag install gnupg")?;

    if !output.status.success() {
        anyhow::bail!("GPG is not working correctly");
    }

    log::info!("Keyring GPG zainicjalizowany w: {}", keyring_dir.display());

    Ok(())
}

/// Importuj klucz GPG
pub fn import_key(key_path: &Path, keyring_dir: &Path) -> Result<String> {
    let output = Command::new("gpg")
        .args(["--homedir"])
        .arg(keyring_dir)
        .args(["--import", "--batch", "--no-tty"])
        .arg(key_path)
        .output()
        .context("Failed to import GPG key")?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        anyhow::bail!("Key import failed: {}", stderr);
    }

    // Wyciągnij informację o kluczu
    let key_info = stderr
        .lines()
        .find(|l| l.contains("imported") || l.contains("key"))
        .map(|l| l.to_string())
        .unwrap_or_else(|| "Key imported".to_string());

    Ok(key_info)
}

/// Pobierz klucz z keyservera
pub fn recv_key(key_id: &str, keyserver: &str, keyring_dir: &Path) -> Result<String> {
    let output = Command::new("gpg")
        .args(["--homedir"])
        .arg(keyring_dir)
        .args(["--keyserver", keyserver])
        .args(["--recv-keys", "--batch", "--no-tty"])
        .arg(key_id)
        .output()
        .with_context(|| format!("Failed to fetch key {} from {}", key_id, keyserver))?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        anyhow::bail!("Key fetch failed: {}", stderr);
    }

    Ok(format!("Key {} fetched from {}", key_id, keyserver))
}

/// Usuń klucz z keyringu
pub fn remove_key(key_id: &str, keyring_dir: &Path) -> Result<()> {
    let output = Command::new("gpg")
        .args(["--homedir"])
        .arg(keyring_dir)
        .args(["--delete-key", "--batch", "--yes", "--no-tty"])
        .arg(key_id)
        .output()
        .context("Failed to remove key")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Key removal failed: {}", stderr);
    }

    Ok(())
}

/// Listuj klucze w keyringu
pub fn list_keys(keyring_dir: &Path) -> Result<Vec<KeyInfo>> {
    let output = Command::new("gpg")
        .args(["--homedir"])
        .arg(keyring_dir)
        .args(["--list-keys", "--with-colons", "--batch", "--fixed-list-mode"])
        .output()
        .context("Failed to list keys")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut keys = Vec::new();
    let mut current: Option<KeyInfo> = None;

    for line in stdout.lines() {
        if line.starts_with("pub:") {
            if let Some(k) = current.take() {
                keys.push(k);
            }
            let parts: Vec<&str> = line.split(':').collect();
            current = Some(KeyInfo {
                key_id: parts.get(4).map(|s| s.to_string()),
                fingerprint: None,
                uids: Vec::new(),
                created: parts.get(5).map(|s| s.to_string()),
                expires: parts.get(6).map(|s| s.to_string()),
                trust: parts.get(1).map(|s| s.to_string()),
            });
        } else if line.starts_with("fpr:") {
            if let Some(ref mut k) = current {
                let parts: Vec<&str> = line.split(':').collect();
                k.fingerprint = parts.get(9).map(|s| s.to_string());
            }
        } else if line.starts_with("uid:") {
            if let Some(ref mut k) = current {
                let parts: Vec<&str> = line.split(':').collect();
                if let Some(uid) = parts.get(9) {
                    k.uids.push(uid.to_string());
                }
            }
        }
    }

    if let Some(k) = current {
        keys.push(k);
    }

    Ok(keys)
}

#[derive(Debug, Clone)]
pub struct KeyInfo {
    pub key_id: Option<String>,
    pub fingerprint: Option<String>,
    pub uids: Vec<String>,
    pub created: Option<String>,
    pub expires: Option<String>,
    pub trust: Option<String>,
}

/// Wygeneruj nową parę kluczy GPG
pub fn generate_key(name: &str, email: &str, keyring_dir: &Path) -> Result<String> {
    let gpg_batch = format!(
        "Key-Type: ed25519\nKey-Curve: ed25519\nSubkey-Type: ed25519\nSubkey-Curve: ed25519\nName-Real: {}\nName-Email: {}\nExpire-Date: 2y\n%no-protection\n%commit\n",
        name, email
    );

    let mut child = Command::new("gpg")
        .args(["--homedir"])
        .arg(keyring_dir)
        .args(["--batch", "--gen-key"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    use std::io::Write;
    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(gpg_batch.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        anyhow::bail!("Key generation failed: {}", stderr);
    }

    Ok(stderr.to_string())
}

/// Wyeksportuj klucz publiczny
pub fn export_key(key_id: &str, output_path: &Path, keyring_dir: &Path) -> Result<()> {
    let output = Command::new("gpg")
        .args(["--homedir"])
        .arg(keyring_dir)
        .args(["--export", "--armor", "--batch", "--no-tty"])
        .arg(key_id)
        .arg("--output")
        .arg(output_path)
        .output()
        .context("Failed to export key")?;

    if !output.status.success() {
        anyhow::bail!("Key export failed");
    }

    Ok(())
}
