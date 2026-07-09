// ────────────────────────────────────────────────────────────────────────────────
// Pag — Migawki Btrfs
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Informacje o migawce
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Snapshot {
    /// Unikalny identyfikator
    pub id: String,

    /// Name migawki
    pub name: String,

    /// Data utworzenia
    pub created: String,

    /// Description
    pub description: String,

    /// Ścieżka do migawki
    pub path: PathBuf,

    /// Typ migawki (pre/post/manual)
    pub snapshot_type: String,

    /// Czy to migawka tylko do odczytu
    pub read_only: bool,
}

/// Typ systemu migawek
pub enum SnapshotBackend {
    Btrfs,
    Zfs,
    Rsync,
}

impl SnapshotBackend {
    pub fn detect() -> Self {
        // Sprawdź Btrfs na /
        if let Ok(output) = Command::new("stat")
            .args(["-f", "-c", "%T", "/"])
            .output()
        {
            let fs_type = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if fs_type == "btrfs" {
                return SnapshotBackend::Btrfs;
            }
        }

        // Sprawdź ZFS
        if Command::new("zfs").arg("list").output().is_ok() {
            return SnapshotBackend::Zfs;
        }

        // Domyślnie Rsync
        SnapshotBackend::Rsync
    }

    pub fn is_supported(&self) -> bool {
        match self {
            SnapshotBackend::Btrfs => Command::new("btrfs").arg("version").output().is_ok(),
            SnapshotBackend::Zfs => Command::new("zfs").arg("version").output().is_ok(),
            SnapshotBackend::Rsync => Command::new("rsync").arg("--version").output().is_ok(),
        }
    }
}

/// Utwórz migawkę
pub fn create_snapshot(
    snapshot_dir: &Path,
    message: Option<&str>,
    snapshot_type: &str,
    prefix: &str,
) -> Result<Snapshot> {
    let backend = SnapshotBackend::detect();

    if !backend.is_supported() {
        anyhow::bail!(
            "Snapshot system ({}) is not supported. Install btrfs-progs, zfs, or rsync.",
            match backend {
                SnapshotBackend::Btrfs => "btrfs",
                SnapshotBackend::Zfs => "zfs",
                SnapshotBackend::Rsync => "rsync",
            }
        );
    }

    let now = Utc::now();
    let id = format!("{}_{}", prefix, now.format("%Y%m%d_%H%M%S"));
    let name = format!("{}_{}", snapshot_type, now.format("%Y-%m-%d %H:%M:%S"));

    match backend {
        SnapshotBackend::Btrfs => create_btrfs_snapshot(snapshot_dir, &id, &name, message),
        SnapshotBackend::Zfs => create_zfs_snapshot(snapshot_dir, &id, &name, message),
        SnapshotBackend::Rsync => create_rsync_snapshot(snapshot_dir, &id, &name, message),
    }
}

/// Utwórz migawkę Btrfs
fn create_btrfs_snapshot(
    base_dir: &Path,
    id: &str,
    name: &str,
    message: Option<&str>,
) -> Result<Snapshot> {
    let snap_path = base_dir.join(id);

    // btrfs subvolume snapshot -r / /snapshots/pag_XXXX
    let output = Command::new("btrfs")
        .args(["subvolume", "snapshot", "-r", "/"])
        .arg(&snap_path)
        .output()
        .context("Failed to create Btrfs snapshot. Is / a Btrfs subvolume?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Btrfs snapshot failed: {}", stderr);
    }

    Ok(Snapshot {
        id: id.to_string(),
        name: name.to_string(),
        created: Utc::now().to_rfc3339(),
        description: message.unwrap_or("Migawka systemu").to_string(),
        path: snap_path,
        snapshot_type: "btrfs".to_string(),
        read_only: true,
    })
}

/// Utwórz migawkę ZFS
fn create_zfs_snapshot(
    _base_dir: &Path,
    id: &str,
    name: &str,
    message: Option<&str>,
) -> Result<Snapshot> {
    let output = Command::new("zfs")
        .args(["snapshot", "-r", &format!("rpool/ROOT@paganlinux_{}", id)])
        .output()
        .context("Failed to create ZFS snapshot")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("ZFS snapshot failed: {}", stderr);
    }

    Ok(Snapshot {
        id: id.to_string(),
        name: name.to_string(),
        created: Utc::now().to_rfc3339(),
        description: message.unwrap_or("Migawka systemu").to_string(),
        path: PathBuf::from(format!("rpool/ROOT@paganlinux_{}", id)),
        snapshot_type: "zfs".to_string(),
        read_only: true,
    })
}

/// Utwórz migawkę Rsync (backup)
fn create_rsync_snapshot(
    base_dir: &Path,
    id: &str,
    name: &str,
    message: Option<&str>,
) -> Result<Snapshot> {
    let snap_path = base_dir.join(id);
    fs::create_dir_all(&snap_path)?;

    log::info!("Creating rsync snapshot (this may take a while)...");

    let output = Command::new("rsync")
        .args([
            "-aAXv",
            "--delete",
            "--exclude=/dev/*",
            "--exclude=/proc/*",
            "--exclude=/sys/*",
            "--exclude=/tmp/*",
            "--exclude=/run/*",
            "--exclude=/mnt/*",
            "--exclude=/media/*",
            "--exclude=/lost+found",
            "--exclude=/var/cache/pag/*",
            "--exclude=/var/tmp/pag/*",
        ])
        .args(["/", snap_path.to_str().unwrap()])
        .output()
        .context("Failed to create rsync snapshot")?;

    if !output.status.success() {
        anyhow::bail!(
            "Rsync snapshot failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(Snapshot {
        id: id.to_string(),
        name: name.to_string(),
        created: Utc::now().to_rfc3339(),
        description: message.unwrap_or("Migawka systemu").to_string(),
        path: snap_path,
        snapshot_type: "rsync".to_string(),
        read_only: false,
    })
}

/// Przywróć migawkę Btrfs
pub fn restore_snapshot_btrfs(snap: &Snapshot) -> Result<()> {
    // Uwaga: przywracanie migawki Btrfs jest złożone i wymaga restartu
    // Ta funkcja przygotowuje przywracanie, które dokończy się po restarcie

    log::warn!("Restoring Btrfs snapshot requires a system reboot.");
    log::info!("Snapshot {} will be restored on next boot.", snap.id);

    // Zapisz informację o przywracaniu do /etc/pag/restore.conf
    let restore_conf = format!(
        "snapshot_id={}\nsnapshot_path={}\n",
        snap.id,
        snap.path.display()
    );

    fs::create_dir_all("/etc/pag")?;
    fs::write("/etc/pag/restore.conf", restore_conf)?;

    Ok(())
}

/// Listuj migawki
pub fn list_snapshots(snapshot_dir: &Path) -> Result<Vec<Snapshot>> {
    let mut snapshots = Vec::new();

    if !snapshot_dir.exists() {
        return Ok(snapshots);
    }

    // Listuj katalogi migawek
    for entry in fs::read_dir(snapshot_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let name = path.file_name().unwrap().to_string_lossy().to_string();

            let metadata = fs::metadata(&path)?;
            let created = metadata
                .modified()
                .ok()
                .and_then(|t| {
                    chrono::DateTime::from_timestamp(
                        t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
                        0,
                    )
                })
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default();

            snapshots.push(Snapshot {
                id: name.clone(),
                name,
                created,
                description: String::new(),
                path,
                snapshot_type: "unknown".into(),
                read_only: false,
            });
        }
    }

    // Sortuj od najnowszych
    snapshots.sort_by(|a, b| b.created.cmp(&a.created));

    Ok(snapshots)
}

/// Usuń migawkę
pub fn delete_snapshot(snap: &Snapshot) -> Result<()> {
    let backend = SnapshotBackend::detect();

    match backend {
        SnapshotBackend::Btrfs => {
            // btrfs subvolume delete
            let output = Command::new("btrfs")
                .args(["subvolume", "delete"])
                .arg(&snap.path)
                .output()
                .context("Failed to delete Btrfs snapshot")?;

            if !output.status.success() {
                anyhow::bail!(
                    "Snapshot deletion failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        SnapshotBackend::Zfs => {
            let output = Command::new("zfs")
                .args(["destroy", "-r"])
                .arg(&format!("rpool/ROOT@paganlinux_{}", snap.id))
                .output()
                .context("Failed to delete ZFS snapshot")?;

            if !output.status.success() {
                anyhow::bail!("ZFS snapshot deletion failed");
            }
        }
        SnapshotBackend::Rsync => {
            fs::remove_dir_all(&snap.path)?;
        }
    }

    Ok(())
}
