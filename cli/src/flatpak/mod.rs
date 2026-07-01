// Moduł integracji Flatpak
//
// pag zapewnia natywną obsługę Flatpak:
// - Instalacja/usuwanie flatpaków
// - Wyszukiwanie
// - Zarządzanie remote'ami

use std::process::Command;

/// Sprawdza czy flatpak jest dostępny w systemie
pub fn is_flatpak_available() -> bool {
    Command::new("flatpak")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Instaluje flatpaka
pub fn install(packages: &[String], remote: Option<&str>) -> anyhow::Result<()> {
    for pkg in packages {
        let mut cmd = Command::new("flatpak");
        cmd.arg("install");
        cmd.arg("--noninteractive");

        if let Some(r) = remote {
            cmd.arg(r);
        }

        cmd.arg(pkg);

        let status = cmd.status()?;
        if !status.success() {
            anyhow::bail!("Flatpak: nie udało się zainstalować {}", pkg);
        }
    }
    Ok(())
}

/// Usuwa flatpaka
pub fn remove(packages: &[String]) -> anyhow::Result<()> {
    for pkg in packages {
        let status = Command::new("flatpak")
            .args(["uninstall", "--noninteractive", pkg])
            .status()?;

        if !status.success() {
            anyhow::bail!("Flatpak: nie udało się usunąć {}", pkg);
        }
    }
    Ok(())
}

/// Wyszukuje flatpaki
pub fn search(query: &[String]) -> anyhow::Result<Vec<FlatpakInfo>> {
    let query_str = query.join(" ");
    let output = Command::new("flatpak")
        .args(["search", "--columns=name,application,version,branch,description", &query_str])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("Flatpak: wyszukiwanie nie powiodło się");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();

    for line in stdout.lines().skip(1) {
        // Flatpak search zwraca tab-separowane kolumny
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 4 {
            results.push(FlatpakInfo {
                name: parts[0].to_string(),
                app_id: parts[1].to_string(),
                version: parts[2].to_string(),
                branch: parts[3].to_string(),
                description: parts.get(4).map(|s| s.to_string()).unwrap_or_default(),
            });
        }
    }

    Ok(results)
}

/// Aktualizuje flatpaki
pub fn update() -> anyhow::Result<()> {
    let status = Command::new("flatpak")
        .args(["update", "--noninteractive"])
        .status()?;

    if !status.success() {
        anyhow::bail!("Flatpak: aktualizacja nie powiodła się");
    }
    Ok(())
}

/// Lista zainstalowanych flatpaków
pub fn list_installed() -> anyhow::Result<Vec<FlatpakInfo>> {
    let output = Command::new("flatpak")
        .args(["list", "--columns=name,application,version,branch,origin"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("Flatpak: nie udało się pobrać listy");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();

    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 4 {
            results.push(FlatpakInfo {
                name: parts[0].to_string(),
                app_id: parts[1].to_string(),
                version: parts[2].to_string(),
                branch: parts[3].to_string(),
                description: parts.get(4).map(|s| s.to_string()).unwrap_or_default(),
            });
        }
    }

    Ok(results)
}

/// Dodaje flatpak remote
pub fn add_remote(name: &str, url: &str) -> anyhow::Result<()> {
    let status = Command::new("flatpak")
        .args(["remote-add", "--if-not-exists", name, url])
        .status()?;

    if !status.success() {
        anyhow::bail!("Flatpak: nie udało się dodać remote {}", name);
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct FlatpakInfo {
    pub name: String,
    pub app_id: String,
    pub version: String,
    pub branch: String,
    pub description: String,
}

/// Sprawdza czy dana aplikacja jest dostępna jako flatpak
pub fn is_available_as_flatpak(app_id: &str) -> anyhow::Result<bool> {
    let results = search(&[app_id.to_string()])?;
    Ok(results.iter().any(|r| r.app_id == app_id))
}
