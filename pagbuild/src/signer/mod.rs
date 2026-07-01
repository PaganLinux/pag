// Moduł podpisywania pakietów .pag
//
// Używa GPG do tworzenia podpisów cyfrowych dla pakietów

use std::path::{Path, PathBuf};

/// Podpisuje pakiet .pag
pub fn sign_package(package: &str, key_id: Option<&str>, output_dir: &str) -> anyhow::Result<()> {
    let pkg_path = find_package(package, output_dir)?;

    if !pkg_path.exists() {
        anyhow::bail!("Nie znaleziono pakietu: {}", package);
    }

    println!("🔐 Podpisywanie: {}", pkg_path.display());

    // Wczytaj pakiet
    let mut pkg = crate::pag_package_header::read_package(&pkg_path)?;

    // Generuj sygnaturę za pomocą GPG
    let signature_path = pkg_path.with_extension("pag.sig");

    let mut cmd = std::process::Command::new("gpg");
    cmd.arg("--detach-sign");
    cmd.arg("--armor");

    if let Some(key) = key_id {
        cmd.args(["--local-user", key]);
    }

    cmd.arg(&pkg_path);
    cmd.arg("-o");
    cmd.arg(&signature_path);

    let status = cmd.status()?;

    if !status.success() {
        anyhow::bail!("Nie udało się podpisać pakietu. Sprawdź konfigurację GPG.");
    }

    // Wczytaj sygnaturę
    let signature = std::fs::read_to_string(&signature_path)?;

    // Zapisz sygnaturę w nagłówku
    pkg.header.pgp_signature = Some(signature);

    // Zapisz zaktualizowany pakiet
    crate::pag_package_header::write_package(&pkg_path, &pkg.header, &pkg.raw_payload)?;

    println!("  ✓ Podpisano: {}", pkg_path.display());
    println!("  Sygnatura: {}", signature_path.display());

    Ok(())
}

/// Znajduje plik .pag w katalogu wyjściowym
fn find_package(name_or_path: &str, output_dir: &str) -> anyhow::Result<PathBuf> {
    // Sprawdź czy to bezpośrednia ścieżka
    let direct = Path::new(name_or_path);
    if direct.exists() && direct.extension().map_or(false, |e| e == "pag") {
        return Ok(direct.to_path_buf());
    }

    // Szukaj w katalogu wyjściowym
    let output = Path::new(output_dir);
    if output.exists() {
        for entry in std::fs::read_dir(output)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                if filename.starts_with(name_or_path) && filename.ends_with(".pag") {
                    return Ok(path);
                }
            }
        }
    }

    // Szukaj w bieżącym katalogu
    for entry in std::fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
            if filename.starts_with(name_or_path) && filename.ends_with(".pag") {
                return Ok(path);
            }
        }
    }

    anyhow::bail!("Nie znaleziono pliku .pag dla: {}", name_or_path)
}
