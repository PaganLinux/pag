// ────────────────────────────────────────────────────────────────────────────────
// Pag — Komenda zarządzania migawkami
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use colored::*;
use dialoguer::Confirm;

use crate::config::Config;
use crate::cli::{Cli, SnapshotCommand};
use crate::snapshot::btrfs;

pub async fn run(config: &Config, action: &SnapshotCommand, cli: &Cli) -> Result<()> {
    if !nix::unistd::Uid::effective().is_root() {
        anyhow::bail!("Snapshot management requires root.");
    }

    if !config.snapshots.enabled {
        anyhow::bail!("Snapshots are disabled in config. Enable in /etc/pag/pag.conf.");
    }

    match action {
        SnapshotCommand::Create { message } => {
            println!("{}", "Creating system snapshot...".bold());

            let snap = btrfs::create_snapshot(
                &config.paths.snapshot_path,
                message.as_deref(),
                "manual",
                &config.snapshots.prefix,
            )?;

            println!();
            println!("{} {}", "✓".green(), "Snapshot created!".green().bold());
            println!("  ID:   {}", snap.id.cyan());
            println!("  Data: {}", snap.created);
            println!("  Typ:  {}", snap.snapshot_type);
            println!("  Path: {}", snap.path.display());

            // Sprawdź limit migawek
            let snapshots = btrfs::list_snapshots(&config.paths.snapshot_path)?;
            if snapshots.len() as u32 > config.snapshots.max_snapshots {
                log::warn!(
                    "Snapshot limit exceeded ({}/{}). Consider removing old snapshots.",
                    snapshots.len(),
                    config.snapshots.max_snapshots
                );
            }
        }

        SnapshotCommand::List => {
            let snapshots = btrfs::list_snapshots(&config.paths.snapshot_path)?;

            if snapshots.is_empty() {
                println!("{}", "No snapshots.".dimmed());
                return Ok(());
            }

            println!("{} ({}):", "System snapshots".bold(), snapshots.len());
            println!();

            for snap in &snapshots {
                println!(
                    "  {}  {}  {}",
                    snap.id.cyan(),
                    snap.created.dimmed(),
                    snap.snapshot_type.yellow()
                );
            }
        }

        SnapshotCommand::Restore { id } => {
            let snapshots = btrfs::list_snapshots(&config.paths.snapshot_path)?;
            let snap = snapshots
                .iter()
                .find(|s| s.id == *id || s.id.starts_with(id.as_str()))
                .context(format!("Snapshot not found: {}", id))?;

            println!("{}", "=== Restoring snapshot ===".bold().red());
            println!("  ID:   {}", snap.id.cyan());
            println!("  Data: {}", snap.created);
            println!();
            println!("{}", "WARNING: Snapshot restore requires a system reboot!".yellow().bold());
            println!();

            if !cli.force {
                let confirm = Confirm::new()
                    .with_prompt("Proceed with restore?")
                    .default(false)
                    .interact()?;

                if !confirm {
                    println!("{}", "Aborted.".dimmed());
                    return Ok(());
                }
            }

            btrfs::restore_snapshot_btrfs(snap)?;

            println!();
            println!("{}", "Snapshot will be restored after reboot.".green().bold());
            println!("{}", "Reboot the system: reboot".dimmed());
        }

        SnapshotCommand::Delete { id } => {
            let snapshots = btrfs::list_snapshots(&config.paths.snapshot_path)?;
            let snap = snapshots
                .iter()
                .find(|s| s.id == *id || s.id.starts_with(id.as_str()))
                .context(format!("Snapshot not found: {}", id))?;

            if !cli.force {
                let confirm = Confirm::new()
                    .with_prompt(format!("Delete snapshot {}?", snap.id))
                    .default(false)
                    .interact()?;

                if !confirm {
                    println!("{}", "Aborted.".dimmed());
                    return Ok(());
                }
            }

            btrfs::delete_snapshot(snap)?;
            println!("{} {}", "✓".green(), format!("Snapshot deleted: {}", snap.id));
        }
    }

    Ok(())
}
