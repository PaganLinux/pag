// Moduł uploadu pakietów do repozytorium

use std::path::Path;

/// Wysyła pakiety do repozytorium
pub async fn upload_packages(
    packages: &[String],
    repo: &str,
    server: &str,
    api_token: Option<&str>,
) -> anyhow::Result<()> {
    if packages.is_empty() {
        anyhow::bail!("Nie podano pakietów do wysłania");
    }

    let token = api_token.ok_or_else(|| {
        anyhow::anyhow!("Wymagany token API. Ustaw zmienną PAGBUILD_API_TOKEN lub użyj --token.")
    })?;

    let client = reqwest::Client::new();

    for pkg_file in packages {
        let path = Path::new(pkg_file);

        if !path.exists() {
            println!("  ✗ Nie znaleziono: {}", pkg_file);
            continue;
        }

        if path.extension().map_or(true, |e| e != "pag") {
            println!("  ⚠ {} nie jest plikiem .pag, pomijam", pkg_file);
            continue;
        }

        let filename = path.file_name().unwrap().to_string_lossy();
        let data = std::fs::read(path)?;

        let url = format!("{}/api/v1/upload/{}/{}", server.trim_end_matches('/'), repo, filename);

        println!("  📤 Wysyłanie {}...", filename);

        let response = client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/octet-stream")
            .body(data)
            .send()
            .await?;

        if response.status().is_success() {
            println!("  ✓ {} wysłany pomyślnie", filename);
        } else {
            let body = response.text().await?;
            println!("  ✗ Błąd wysyłania {}: {}", filename, body);
        }

        // Wyślij też sygnaturę jeśli istnieje
        let sig_path = path.with_extension("pag.sig");
        if sig_path.exists() {
            let sig_data = std::fs::read(&sig_path)?;
            let sig_filename = sig_path.file_name().unwrap().to_string_lossy();

            let sig_url = format!("{}/api/v1/upload/{}/{}", server.trim_end_matches('/'), repo, sig_filename);

            let sig_response = client
                .put(&sig_url)
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/octet-stream")
                .body(sig_data)
                .send()
                .await?;

            if sig_response.status().is_success() {
                println!("  ✓ Sygnatura {} wysłana", sig_filename);
            }
        }
    }

    Ok(())
}
