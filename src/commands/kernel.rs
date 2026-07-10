use std::path::Path;
use anyhow::Result;
use crate::utils::fs;
use crate::repo::download::DownloadManager;
use crate::cli::{KernelCommand, Cli};
use crate::config::Config;

/// Struktura reprezentująca jądro systemu
pub struct Kernel {
    pub version: String,
    pub package_name: String,
    pub installed: bool,
}

impl Kernel {
    /// Tworzy nową instancję jądra
    pub fn new(version: &str, package_name: &str) -> Self {
        Kernel {
            version: version.to_string(),
            package_name: package_name.to_string(),
            installed: false,
        }
    }

    /// Pobiera jądro z repozytorium
    pub async fn download(&self, url: &str) -> Result<()> {
        let manager = DownloadManager::new();
        let kernel_file = format!("{}.pkg.tar.zst", self.package_name);
        manager.download(url, &kernel_file).await?;
        Ok(())
    }

    /// Instaluje jądro
    pub fn install(&self) -> Result<()> {
        let kernel_file = format!("{}.pkg.tar.zst", self.package_name);
        let install_path = Path::new("/boot");
        fs::extract_archive(&kernel_file, install_path)?;
        self.update_bootloader()?;
        Ok(())
    }

    /// Aktualizuje bootloader (np. GRUB)
    fn update_bootloader(&self) -> Result<()> {
        // Tutaj można dodać logikę aktualizacji bootloadera
        fs::run_command("grub-mkconfig", &["-o", "/boot/grub/grub.cfg"])?;
        Ok(())
    }

    /// Tworzy kopię zapasową starego jądra
    pub fn backup_old_kernel(&self) -> Result<()> {
        let old_kernel = Path::new("/boot/vmlinuz-old");
        let current_kernel = Path::new("/boot/vmlinuz");
        fs::copy_file(current_kernel, old_kernel)?;
        Ok(())
    }
}

/// Uruchamia komendę kernel
pub async fn run(config: &Config, action: &KernelCommand, cli: &Cli) -> Result<()> {
    match action {
        KernelCommand::Install { version } => {
            let kernel = Kernel::new(version, &format!("linux-", version));
            kernel.backup_old_kernel()?;
            kernel.download(&format!("https://repo.paganlinux.eu/kernels/linux-", version, ".pkg.tar.zst")).await?;
            kernel.install()?;
            println!("Kernel {} installed successfully.", version);
        }
        KernelCommand::Remove { version } => {
            // Tutaj można dodać logikę usuwania jądra
            println!("Kernel {} removed successfully.", version);
        }
        KernelCommand::List => {
            // Tutaj można dodać logikę listowania zainstalowanych jąder
            println!("List of installed kernels:");
        }
        KernelCommand::Backup => {
            let kernel = Kernel::new("current", "linux-current");
            kernel.backup_old_kernel()?;
            println!("Current kernel backed up successfully.");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_kernel_creation() {
        let kernel = Kernel::new("5.15.0", "linux-5.15.0");
        assert_eq!(kernel.version, "5.15.0");
        assert_eq!(kernel.package_name, "linux-5.15.0");
    }

    #[test]
    fn test_kernel_backup() {
        let kernel = Kernel::new("5.15.0", "linux-5.15.0");
        assert!(kernel.backup_old_kernel().is_ok());
    }
}