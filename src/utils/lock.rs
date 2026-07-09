// ────────────────────────────────────────────────────────────────────────────────
// Pag — Mechanizm blokowania bazy danych
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use fs2::FileExt;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

/// Blokada bazy danych — zapobiega równoczesnym operacjom
pub struct DatabaseLock {
    file: File,
    path: PathBuf,
}

impl DatabaseLock {
    /// Spróbuj uzyskać blokadę
    pub fn acquire(lock_path: &Path) -> Result<Self> {
        // Upewnij się że katalog istnieje
        if let Some(parent) = lock_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let file = File::create(lock_path)
            .with_context(|| format!("Failed to create lock file: {:?}", lock_path))?;

        // Spróbuj uzyskać blokadę wyłączną (nieblokująco)
        file.try_lock_exclusive()
            .with_context(|| "Database is locked by another process. If no pag process is running, remove the lock file manually.")?;

        Ok(DatabaseLock {
            file,
            path: lock_path.to_path_buf(),
        })
    }

    /// Zapisz PID do pliku blokady
    pub fn write_pid(&self) -> Result<()> {
        let pid = std::process::id();
        fs::write(&self.path, pid.to_string())?;
        Ok(())
    }
}

impl Drop for DatabaseLock {
    fn drop(&mut self) {
        // Odblokuj i usuń plik blokady
        let _ = self.file.unlock();
        let _ = fs::remove_file(&self.path);
    }
}
