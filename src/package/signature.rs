// ────────────────────────────────────────────────────────────────────────────────
// Pag — Podpisy cyfrowe .pag.sig
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Podpisz plik .pag używając GPG
pub fn sign_package(pkg_path: &Path, key_id: Option<&str>) -> Result<PathBuf> {
    let sig_path = pkg_path.with_extension("pag.sig");

    let mut cmd = Command::new("gpg");
    cmd.arg("--detach-sign");
    cmd.arg("--no-armor");
    cmd.arg("--batch");

    if let Some(key) = key_id {
        cmd.args(["--local-user", key]);
    }

    // Użyj domyślnego keyringu
    cmd.arg("--output");
    cmd.arg(&sig_path);
    cmd.arg(pkg_path);

    let output = cmd.output().context("Failed to run gpg for signing")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Signing failed: {}", stderr);
    }

    Ok(sig_path)
}

/// Zweryfikuj podpis .pag.sig dla pliku .pag
pub fn verify_signature(pkg_path: &Path, keyring_dir: Option<&Path>) -> Result<VerificationResult> {
    let sig_path = pkg_path.with_extension("pag.sig");

    if !sig_path.exists() {
        return Ok(VerificationResult::NoSignature);
    }

    let mut cmd = Command::new("gpg");
    cmd.arg("--verify");
    cmd.arg("--batch");
    cmd.arg("--no-tty");
    cmd.arg("--status-fd=1");

    if let Some(kr) = keyring_dir {
        cmd.args(["--homedir"]);
        cmd.arg(kr);
    }

    cmd.arg(&sig_path);
    cmd.arg(pkg_path);

    let output = cmd.output().context("Failed to run gpg for verification")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Parsuj output GPG status-fd
    let result = if output.status.success() && stdout.contains("GOODSIG") {
        // Wyciągnij ID klucza
        let key_id = stdout
            .lines()
            .find(|l| l.contains("VALIDSIG"))
            .and_then(|l| l.split_whitespace().nth(2))
            .map(|s| s.to_string());

        let signer = stdout
            .lines()
            .find(|l| l.contains("GOODSIG"))
            .and_then(|l| {
                let parts: Vec<&str> = l.split_whitespace().collect();
                if parts.len() >= 3 {
                    Some(parts[2..].join(" "))
                } else {
                    None
                }
            });

        VerificationResult::Valid {
            key_id,
            signer,
        }
    } else if stdout.contains("BADSIG") {
        VerificationResult::Invalid {
            reason: "Invalid signature".into(),
        }
    } else if stdout.contains("ERRSIG") || stdout.contains("NO_PUBKEY") {
        VerificationResult::UnknownKey {
            key_id: stdout
                .lines()
                .find(|l| l.contains("ERRSIG") || l.contains("NO_PUBKEY"))
                .and_then(|l| l.split_whitespace().nth(2))
                .map(|s| s.to_string()),
        }
    } else {
        VerificationResult::Invalid {
            reason: format!("Verification failed: {}", stderr),
        }
    };

    Ok(result)
}

/// Wynik weryfikacji podpisu
#[derive(Debug, Clone)]
pub enum VerificationResult {
    /// Podpis prawidłowy
    Valid {
        key_id: Option<String>,
        signer: Option<String>,
    },
    /// Podpis nieprawidłowy
    Invalid {
        reason: String,
    },
    /// Brak podpisu
    NoSignature,
    /// Nieznany klucz
    UnknownKey {
        key_id: Option<String>,
    },
}

impl VerificationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, VerificationResult::Valid { .. })
    }

    pub fn is_ok_or_missing(&self) -> bool {
        matches!(self, VerificationResult::Valid { .. } | VerificationResult::NoSignature)
    }
}

/// Importuj klucz GPG do keyringu
pub fn import_key(key_path: &Path, keyring_dir: &Path) -> Result<String> {
    let mut cmd = Command::new("gpg");
    cmd.args(["--homedir"]);
    cmd.arg(keyring_dir);
    cmd.args(["--import", "--batch", "--no-tty"]);

    // Jeśli to plik
    if key_path.is_file() {
        cmd.arg(key_path);
    } else {
        // Załóż że to ID klucza do pobrania
        cmd.args(["--recv-keys"]);
        cmd.arg(key_path.to_string_lossy().as_ref());
    }

    let output = cmd
        .output()
        .context("Failed to import key")?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        anyhow::bail!("Key import failed: {}", stderr);
    }

    // Wyciągnij fingerprint
    let fingerprint = stderr
        .lines()
        .find(|l| l.contains("key"))
        .map(|l| l.to_string())
        .unwrap_or_else(|| "klucz zaimportowany".into());

    Ok(fingerprint)
}

/// Inicjalizuj keyring
pub fn init_keyring(keyring_dir: &Path) -> Result<()> {
    fs::create_dir_all(keyring_dir)?;

    // Ustaw odpowiednie uprawnienia
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(keyring_dir, fs::Permissions::from_mode(0o700))?;
    }

    // Sprawdź czy gpg jest available
    let output = Command::new("gpg")
        .arg("--version")
        .output()
        .context("GPG is not available on the system. Install with: pag install gnupg")?;

    if !output.status.success() {
        anyhow::bail!("GPG is not working correctly");
    }

    Ok(())
}

/// Listuj klucze w keyringu
pub fn list_keys(keyring_dir: &Path) -> Result<Vec<GpgKey>> {
    let mut cmd = Command::new("gpg");
    cmd.args(["--homedir"]);
    cmd.arg(keyring_dir);
    cmd.args(["--list-keys", "--with-colons", "--batch"]);

    let output = cmd.output().context("Failed to list keys")?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut keys = Vec::new();
    let mut current_key: Option<GpgKey> = None;

    for line in stdout.lines() {
        if line.starts_with("pub:") {
            if let Some(key) = current_key.take() {
                keys.push(key);
            }
            let parts: Vec<&str> = line.split(':').collect();
            current_key = Some(GpgKey {
                key_id: parts.get(4).map(|s| s.to_string()),
                fingerprint: None,
                uids: Vec::new(),
                created: parts.get(5).map(|s| s.to_string()),
                expires: parts.get(6).map(|s| s.to_string()),
                trust: parts.get(1).map(|s| s.to_string()),
            });
        } else if line.starts_with("fpr:") {
            if let Some(ref mut key) = current_key {
                let parts: Vec<&str> = line.split(':').collect();
                key.fingerprint = parts.get(9).map(|s| s.to_string());
            }
        } else if line.starts_with("uid:") {
            if let Some(ref mut key) = current_key {
                let parts: Vec<&str> = line.split(':').collect();
                if let Some(uid) = parts.get(9) {
                    key.uids.push(uid.to_string());
                }
            }
        }
    }

    if let Some(key) = current_key {
        keys.push(key);
    }

    Ok(keys)
}

/// Informacje o kluczu GPG
#[derive(Debug, Clone)]
pub struct GpgKey {
    pub key_id: Option<String>,
    pub fingerprint: Option<String>,
    pub uids: Vec<String>,
    pub created: Option<String>,
    pub expires: Option<String>,
    pub trust: Option<String>,
}
