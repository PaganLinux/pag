// ────────────────────────────────────────────────────────────────────────────────
// Pag — Parser i executor PAGBUILD (kompatybilny z PKGBUILD Arch Linux)
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Sparsowane dane z pliku PAGBUILD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagBuildInfo {
    /// Name pakietu
    pub pkgname: String,

    /// Version pakietu
    pub pkgver: String,

    /// Numer wydania
    pub pkgrel: u32,

    /// Description
    pub pkgdesc: String,

    /// Architektura (x86_64, aarch64, any)
    pub arch: Vec<String>,

    /// URL upstream
    pub url: Option<String>,

    /// Licencje (SPDX)
    pub license: Vec<String>,

    /// Grupy
    pub groups: Vec<String>,

    /// Zależności
    pub depends: Vec<String>,

    /// Opcjonalne zależności
    pub optdepends: Vec<String>,

    /// Zależności budowania
    pub makedepends: Vec<String>,

    /// Zależności testowe
    pub checkdepends: Vec<String>,

    /// Konflikty
    pub conflicts: Vec<String>,

    /// Zastępuje
    pub replaces: Vec<String>,

    /// Dostarcza
    pub provides: Vec<String>,

    /// Źródła (URL lub pliki)
    pub source: Vec<String>,

    /// Sumy kontrolne źródeł (SHA256)
    pub sha256sums: Vec<String>,

    /// Sumy kontrolne BLAKE3
    pub b3sums: Vec<String>,

    /// Backup plików
    pub backup: Vec<String>,

    /// Instalacyjne pliki .install
    pub install: Option<String>,

    /// Opcje (strip, docs, libtool, staticlibs, emptydirs, zipman, debug, lto)
    pub options: Vec<String>,

    /// Niestandardowe zmienne
    pub custom_vars: HashMap<String, String>,

    /// Funkcje (zawartość)
    pub functions: HashMap<String, String>,

    /// Ścieżka do oryginalnego pliku PAGBUILD
    pub source_path: PathBuf,
}

impl PagBuildInfo {
    /// Parsuj plik PAGBUILD
    pub fn parse(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read PAGBUILD: {:?}", path))?;

        Self::parse_str(&content, path)
    }

    /// Parsuj zawartość PAGBUILD jako string
    pub fn parse_str(content: &str, source_path: &Path) -> Result<Self> {
        // Wyciągnij zmienne bashowe
        let mut pkgname = String::new();
        let mut pkgver = String::new();
        let mut pkgrel: u32 = 1;
        let mut pkgdesc = String::new();
        let mut arch = vec!["x86_64".to_string()];
        let mut url = None;
        let mut license = Vec::new();
        let mut groups = Vec::new();
        let mut depends = Vec::new();
        let mut optdepends = Vec::new();
        let mut makedepends = Vec::new();
        let mut checkdepends = Vec::new();
        let mut conflicts = Vec::new();
        let mut replaces = Vec::new();
        let mut provides = Vec::new();
        let mut source = Vec::new();
        let mut sha256sums = Vec::new();
        let mut b3sums = Vec::new();
        let mut backup = Vec::new();
        let mut install = None;
        let mut options: Vec<String> = vec![
            "strip".into(), "docs".into(), "libtool".into(),
            "staticlibs".into(), "emptydirs".into(), "zipman".into(),
        ];
        let mut custom_vars = HashMap::new();

        // Regex do wyciągania zmiennych bashowych
        let var_re = Regex::new(
            r#"(?m)^\s*(\w+)\s*=\s*(?:\(([^)]*)\)|"([^"]*)"|'([^']*)'|(\S+))\s*$"#
        )?;

        // Regex do wyciągania tablic bashowych
        let array_re = Regex::new(r#"["']([^"']*)["']"#)?;

        for line in content.lines() {
            let line = line.trim();

            // Pomiń komentarze i puste linie
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Sprawdź czy to przypisanie zmiennej
            if let Some(caps) = var_re.captures(line) {
                let var_name = caps.get(1).unwrap().as_str();
                let array_value = caps.get(2).map(|m| m.as_str());
                let string_value = caps.get(3).or_else(|| caps.get(4)).map(|m| m.as_str());
                let simple_value = caps.get(5).map(|m| m.as_str());

                match var_name {
                    "pkgname" => {
                        pkgname = string_value.or(simple_value).unwrap_or("").to_string();
                    }
                    "pkgver" => {
                        pkgver = string_value.or(simple_value).unwrap_or("").to_string();
                    }
                    "pkgrel" => {
                        pkgrel = string_value.or(simple_value).unwrap_or("1").parse().unwrap_or(1);
                    }
                    "pkgdesc" => {
                        pkgdesc = string_value.or(simple_value).unwrap_or("").to_string();
                    }
                    "arch" => {
                        if let Some(arr) = array_value {
                            arch = parse_bash_array(arr);
                        } else if let Some(s) = string_value.or(simple_value) {
                            arch = vec![s.to_string()];
                        }
                    }
                    "url" => {
                        url = string_value.or(simple_value).map(|s| s.to_string());
                    }
                    "license" => {
                        if let Some(arr) = array_value {
                            license = parse_bash_array(arr);
                        }
                    }
                    "groups" => {
                        if let Some(arr) = array_value {
                            groups = parse_bash_array(arr);
                        }
                    }
                    "depends" => {
                        if let Some(arr) = array_value {
                            depends = parse_bash_array(arr);
                        }
                    }
                    "optdepends" => {
                        if let Some(arr) = array_value {
                            optdepends = parse_bash_array(arr);
                        }
                    }
                    "makedepends" => {
                        if let Some(arr) = array_value {
                            makedepends = parse_bash_array(arr);
                        }
                    }
                    "checkdepends" => {
                        if let Some(arr) = array_value {
                            checkdepends = parse_bash_array(arr);
                        }
                    }
                    "conflicts" => {
                        if let Some(arr) = array_value {
                            conflicts = parse_bash_array(arr);
                        }
                    }
                    "replaces" => {
                        if let Some(arr) = array_value {
                            replaces = parse_bash_array(arr);
                        }
                    }
                    "provides" => {
                        if let Some(arr) = array_value {
                            provides = parse_bash_array(arr);
                        }
                    }
                    "source" => {
                        if let Some(arr) = array_value {
                            source = parse_bash_array(arr);
                        }
                    }
                    "sha256sums" => {
                        if let Some(arr) = array_value {
                            sha256sums = parse_bash_array(arr);
                        }
                    }
                    "b3sums" => {
                        if let Some(arr) = array_value {
                            b3sums = parse_bash_array(arr);
                        }
                    }
                    "backup" => {
                        if let Some(arr) = array_value {
                            backup = parse_bash_array(arr);
                        }
                    }
                    "install" => {
                        install = string_value.or(simple_value).map(|s| s.to_string());
                    }
                    "options" => {
                        if let Some(arr) = array_value {
                            options = parse_bash_array(arr);
                        }
                    }
                    _ => {
                        // Zapisz niestandardowe zmienne
                        if let Some(val) = string_value.or(simple_value) {
                            custom_vars.insert(var_name.to_string(), val.to_string());
                        }
                    }
                }
            }
        }

        // Wyciągnij funkcje bashowe (build(), package(), prepare(), check())
        let functions = extract_bash_functions(content);

        Ok(PagBuildInfo {
            pkgname,
            pkgver,
            pkgrel,
            pkgdesc,
            arch,
            url,
            license,
            groups,
            depends,
            optdepends,
            makedepends,
            checkdepends,
            conflicts,
            replaces,
            provides,
            source,
            sha256sums,
            b3sums,
            backup,
            install,
            options,
            custom_vars,
            functions,
            source_path: source_path.to_path_buf(),
        })
    }

    /// Pobierz pełną nazwę pakietu
    pub fn full_pkgname(&self) -> String {
        format!("{}-{}-{}", self.pkgname, self.pkgver, self.pkgrel)
    }

    /// Pobierz nazwę pliku .pag
    pub fn pag_filename(&self) -> String {
        let arch = self.arch.first().map(|a| a.as_str()).unwrap_or("x86_64");
        format!("{}-{}-{}-{}.pag", self.pkgname, self.pkgver, self.pkgrel, arch)
    }

    /// Sprawdź czy architektura jest obsługiwana
    pub fn is_arch_supported(&self, target_arch: &str) -> bool {
        self.arch.contains(&"any".to_string()) || self.arch.contains(&target_arch.to_string())
    }
}

/// Parsuj tablicę bashową
fn parse_bash_array(input: &str) -> Vec<String> {
    let re = Regex::new(r#""([^"]*)""#).unwrap();
    re.captures_iter(input)
        .map(|c| c.get(1).unwrap().as_str().to_string())
        .collect()
}

/// Wyciągnij funkcje bashowe z PAGBUILD
fn extract_bash_functions(content: &str) -> HashMap<String, String> {
    let mut functions = HashMap::new();
    let func_re = Regex::new(
        r"(?ms)^(\w+)\s*\(\s*\)\s*\{([^}]*(?:\{[^}]*\}[^}]*)*)\}"
    ).unwrap();

    for caps in func_re.captures_iter(content) {
        let name = caps.get(1).unwrap().as_str().to_string();
        let body = caps.get(2).unwrap().as_str().to_string();
        functions.insert(name, body);
    }

    functions
}

/// Wykonaj PAGBUILD w bashu
pub fn execute_pagbuild(
    pagbuild: &PagBuildInfo,
    build_dir: &Path,
    pkg_dir: &Path,
    config: &crate::config::BuildConfig,
) -> Result<()> {
    // Stwórz tymczasowy skrypt bash do wykonania
    let script_path = build_dir.join("_pag_exec.sh");

    let makepkg_script = format!(
        r#"#!/usr/bin/env bash
set -euo pipefail

# Ustawienia budowania
export CFLAGS="{}"
export CXXFLAGS="{}"
export MAKEFLAGS="-j{}"
export PKGDEST="{}"
export SRCDEST="{}"
export BUILDDIR="{}"
export PKGEXT=".pag"

# Zmienne z PAGBUILD
pkgname="{}"
pkgver="{}"
pkgrel="{}"
pkgdesc="{}"
arch=({})

# Źródło PAGBUILD
source "{}"

# Wykonaj funkcje budowania
if declare -f prepare > /dev/null 2>&1; then
    echo ":: Executing prepare()..."
    prepare
fi

if declare -f build > /dev/null 2>&1; then
    echo ":: Executing build()..."
    build
fi

if declare -f check > /dev/null 2>&1; then
    echo ":: Executing check()..."
    check || echo ":: Warning: check() returned an error"
fi

if declare -f package > /dev/null 2>&1; then
    echo ":: Executing package()..."
    package
fi

echo ":: PAGBUILD executed successfully"
"#,
        config.cflags,
        config.cxxflags,
        config.makeopts,
        pkg_dir.display(),
        build_dir.join("src").display(),
        build_dir.display(),
        pagbuild.pkgname,
        pagbuild.pkgver,
        pagbuild.pkgrel,
        pagbuild.pkgdesc,
        pagbuild.arch.join(" "),
        pagbuild.source_path.display(),
    );

    fs::write(&script_path, &makepkg_script)?;

    // Nadaj uprawnienia wykonywania
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755))?;
    }

    // Uruchom bash
    let output = Command::new("bash")
        .arg(&script_path)
        .current_dir(build_dir)
        .output()
        .context("Failed to execute PAGBUILD")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        anyhow::bail!(
            "PAGBUILD failed:\nstdout:\n{}\nstderr:\n{}",
            stdout,
            stderr
        );
    }

    log::info!("PAGBUILD executed successfully");
    Ok(())
}

/// Pobierz źródła zdefiniowane w PAGBUILD
pub async fn download_sources(
    pagbuild: &PagBuildInfo,
    src_dir: &Path,
    client: &reqwest::Client,
    config: &crate::config::Config,
) -> Result<()> {
    use crate::utils::net;

    fs::create_dir_all(src_dir)?;

    for (i, src) in pagbuild.source.iter().enumerate() {
        // Sprawdź czy to URL czy lokalny plik
        if src.starts_with("http://") || src.starts_with("https://") {
            // Wyciągnij nazwę pliku z URL
            let filename = src.rsplit('/').next().unwrap_or("source");
            let dest = src_dir.join(filename);

            log::info!("Downloading source: {}", src);
            net::download_file(client, src, &dest, &config.network).await?;

            // Zweryfikuj sumę kontrolną jeśli podana
            if let Some(expected) = pagbuild.sha256sums.get(i) {
                if !expected.is_empty() && expected != "SKIP" {
                    let actual = crate::utils::fs::sha256_file(&dest)?;
                    if actual != *expected {
                        anyhow::bail!(
                            "SHA256 mismatch for {}: expected {}, got {}",
                            filename,
                            expected,
                            actual
                        );
                    }
                }
            }
        } else if src.starts_with("git+") {
            // Klonuj repozytorium git
            let git_url = src.trim_start_matches("git+");
            let repo_name = git_url
                .rsplit('/')
                .next()
                .unwrap_or("repo")
                .trim_end_matches(".git");

            let dest = src_dir.join(repo_name);

            log::info!("Cloning repository: {}", git_url);

            let output = Command::new("git")
                .args(["clone", "--depth", "1", git_url])
                .arg(&dest)
                .output()
                .context("Failed to clone git repository")?;

            if !output.status.success() {
                anyhow::bail!(
                    "Clone failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
    }

    Ok(())
}
