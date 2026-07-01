// Parser plików pagbuild
//
// Pagbuild to skrypt bash-podobny definiujący jak zbudować pakiet.
// Struktura jest inspirowana PKGBUILD (Arch Linux) i APKBUILD (Alpine).
//
// Pagbuild ma ściśle określone zmienne i funkcje:
//   Zmienne: pkgname, pkgver, pkgrel, pkgdesc, url, arch, license,
//            makedepends, depends, source, sha512sums, subpackages...
//   Funkcje: build(), check(), package()

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Sparsowana zawartość pliku pagbuild
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagBuild {
    /// Opiekun pakietu
    pub maintainer: Option<String>,

    /// Nazwa pakietu
    pub pkgname: String,

    /// Wersja
    pub pkgver: String,

    /// Numer wydania
    pub pkgrel: u32,

    /// Opis
    pub pkgdesc: String,

    /// URL projektu
    pub url: Option<String>,

    /// Architektura
    pub arch: String,

    /// Licencja
    pub license: String,

    /// Zależności build-time
    pub makedepends: Vec<String>,

    /// Zależności runtime
    pub depends: Vec<String>,

    /// Opcjonalne zależności
    pub optdepends: Vec<String>,

    /// Sub-pakiety
    pub subpackages: Vec<String>,

    /// Źródła do pobrania
    pub source: Vec<String>,

    /// SHA512 sumy źródeł
    pub sha512sums: Vec<String>,

    /// Dostarczane wirtualne pakiety
    pub provides: Vec<String>,

    /// Zastępowane pakiety
    pub replaces: Vec<String>,

    /// Konflikty
    pub conflicts: Vec<String>,

    /// Priorytet providera
    pub provider_priority: Option<u32>,

    /// Argumenty dla patcha
    pub patch_args: Option<String>,

    /// Katalog budowania
    pub builddir: String,

    /// Funkcja build (treść)
    pub build_func: Option<String>,

    /// Funkcja check (treść)
    pub check_func: Option<String>,

    /// Funkcja package (treść)
    pub package_func: Option<String>,

    /// Dodatkowe zmienne
    pub extra_vars: HashMap<String, String>,

    /// Niestandardowe funkcje
    pub extra_funcs: HashMap<String, String>,

    /// Poprawki bezpieczeństwa
    pub secfixes: Vec<String>,
}

/// Parsuje plik pagbuild
pub fn parse_file(file_path: &str, output_format: &str) -> anyhow::Result<()> {
    let path = Path::new(file_path);

    if !path.exists() {
        anyhow::bail!("Plik {} nie istnieje", file_path);
    }

    let content = std::fs::read_to_string(path)?;
    let pagbuild = parse_content(&content)?;

    match output_format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&pagbuild)?);
        }
        "toml" => {
            // Konwersja do TOML przez wartość pośrednią
            let value: toml::Value = toml::from_str(&toml::to_string_pretty(&pagbuild)?)?;
            println!("{}", toml::to_string_pretty(&value)?);
        }
        _ => anyhow::bail!("Nieobsługiwany format: {}", output_format),
    }

    Ok(())
}

/// Parsuje zawartość pliku pagbuild
pub fn parse_content(content: &str) -> anyhow::Result<PagBuild> {
    let mut pagbuild = PagBuild {
        maintainer: None,
        pkgname: String::new(),
        pkgver: String::new(),
        pkgrel: 1,
        pkgdesc: String::new(),
        url: None,
        arch: "all".to_string(),
        license: "custom".to_string(),
        makedepends: Vec::new(),
        depends: Vec::new(),
        optdepends: Vec::new(),
        subpackages: Vec::new(),
        source: Vec::new(),
        sha512sums: Vec::new(),
        provides: Vec::new(),
        replaces: Vec::new(),
        conflicts: Vec::new(),
        provider_priority: None,
        patch_args: None,
        builddir: "$srcdir".to_string(),
        build_func: None,
        check_func: None,
        package_func: None,
        extra_vars: HashMap::new(),
        extra_funcs: HashMap::new(),
        secfixes: Vec::new(),
    };

    let mut current_func: Option<String> = None;
    let mut func_body = String::new();
    let mut in_secfixes = false;
    let mut secfixes_body = String::new();

    for line in content.lines() {
        let line = line.trim();

        // Pomiń puste linie i komentarze
        if line.is_empty() || line.starts_with('#') {
            if line.starts_with("# secfixes:") {
                in_secfixes = true;
                secfixes_body.clear();
            }
            continue;
        }

        // Zamknij funkcję
        if line == "}" {
            if let Some(func_name) = current_func.take() {
                match func_name.as_str() {
                    "build" => pagbuild.build_func = Some(func_body.trim().to_string()),
                    "check" => pagbuild.check_func = Some(func_body.trim().to_string()),
                    "package" => pagbuild.package_func = Some(func_body.trim().to_string()),
                    _ => { pagbuild.extra_funcs.insert(func_name, func_body.trim().to_string()); }
                }
                func_body.clear();
            }
            if in_secfixes {
                in_secfixes = false;
                pagbuild.secfixes = secfixes_body.lines()
                    .filter(|l| !l.trim().is_empty())
                    .map(|l| l.trim().to_string())
                    .collect();
            }
            continue;
        }

        // Jeśli jesteśmy w ciele funkcji
        if let Some(ref _func_name) = current_func {
            func_body.push_str(line);
            func_body.push('\n');
            continue;
        }

        // Jeśli jesteśmy w secfixes
        if in_secfixes {
            secfixes_body.push_str(line);
            secfixes_body.push('\n');
            continue;
        }

        // Sprawdź początek funkcji
        if let Some(func_name) = detect_function_start(line) {
            current_func = Some(func_name);
            func_body.clear();
            continue;
        }

        // Parsuj zmienne: nazwa="wartość" lub nazwa=wartość
        if let Some((key, value)) = parse_variable(line) {
            match key {
                "maintainer" => pagbuild.maintainer = Some(value.to_string()),
                "pkgname" => pagbuild.pkgname = value.to_string(),
                "pkgver" => pagbuild.pkgver = value.to_string(),
                "pkgrel" => pagbuild.pkgrel = value.parse().unwrap_or(1),
                "pkgdesc" => pagbuild.pkgdesc = value.to_string(),
                "url" => pagbuild.url = Some(value.to_string()),
                "arch" => pagbuild.arch = value.to_string(),
                "license" => pagbuild.license = value.to_string(),
                "makedepends" => pagbuild.makedepends = parse_list(value),
                "depends" => pagbuild.depends = parse_list(value),
                "optdepends" => pagbuild.optdepends = parse_list(value),
                "subpackages" => pagbuild.subpackages = parse_list(value),
                "source" => pagbuild.source = parse_multiline_list(value, content),
                "sha512sums" => pagbuild.sha512sums = parse_multiline_list(value, content),
                "provides" => pagbuild.provides = parse_list(value),
                "replaces" => pagbuild.replaces = parse_list(value),
                "conflicts" => pagbuild.conflicts = parse_list(value),
                "provider_priority" => pagbuild.provider_priority = value.parse().ok(),
                "patch_args" => pagbuild.patch_args = Some(value.to_string()),
                "builddir" => pagbuild.builddir = value.to_string(),
                _ => {
                    // Nieznana zmienna - zapisz jako dodatkową
                    pagbuild.extra_vars.insert(key.to_string(), value.to_string());
                }
            }
        }
    }

    Ok(pagbuild)
}

/// Wykrywa początek definicji funkcji
fn detect_function_start(line: &str) -> Option<String> {
    let line = line.trim();
    if line.ends_with("() {") || line.ends_with("() {") {
        let name = line.trim_end_matches("() {").trim_end_matches("() {").trim();
        if !name.is_empty() {
            return Some(name.to_string());
        }
    }
    None
}

/// Parsuje przypisanie zmiennej
fn parse_variable(line: &str) -> Option<(&str, &str)> {
    let line = line.trim();

    // Wzorzec: nazwa=wartość lub nazwa="wartość"
    if let Some(eq_pos) = line.find('=') {
        let key = line[..eq_pos].trim();
        let raw_value = line[eq_pos + 1..].trim();

        // Usuń cudzysłowy
        let value = if (raw_value.starts_with('"') && raw_value.ends_with('"'))
            || (raw_value.starts_with('\'') && raw_value.ends_with('\''))
        {
            &raw_value[1..raw_value.len() - 1]
        } else {
            raw_value
        };

        return Some((key, value));
    }
    None
}

/// Parsuje listę rozdzielaną spacjami
fn parse_list(value: &str) -> Vec<String> {
    value.split_whitespace()
        .map(|s| s.trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Parsuje listę wieloliniową (dla source i sha512sums)
fn parse_multiline_list(initial: &str, _full_content: &str) -> Vec<String> {
    // Jeśli wartość jest pusta, szukaj kontynuacji wieloliniowej
    if initial.is_empty() {
        // Znajdź następne linie aż do końca listy
        Vec::new()
    } else {
        parse_list(initial)
    }
}

/// Wczytuje pagbuild z katalogu pagports
pub fn load_from_pagports(pagports_dir: &str, package_name: &str) -> anyhow::Result<PagBuild> {
    let build_file = Path::new(pagports_dir)
        .join(package_name)
        .join("pagbuild");

    if !build_file.exists() {
        // Szukaj też bezpośrednio
        let alt = Path::new(pagports_dir).join(package_name);
        if alt.is_file() {
            let content = std::fs::read_to_string(&alt)?;
            return parse_content(&content);
        }
        anyhow::bail!("Nie znaleziono pagbuild dla {}", package_name);
    }

    let content = std::fs::read_to_string(&build_file)?;
    parse_content(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal() {
        let content = r#"
pkgname=test
pkgver=1.0.0
pkgrel=1
pkgdesc="Test package"
arch="x86_64"
license="MIT"

build() {
    make
}

package() {
    make install
}
"#;
        let result = parse_content(content).unwrap();
        assert_eq!(result.pkgname, "test");
        assert_eq!(result.pkgver, "1.0.0");
        assert_eq!(result.pkgdesc, "Test package");
        assert!(result.build_func.is_some());
        assert!(result.package_func.is_some());
    }

    #[test]
    fn test_parse_with_variables() {
        let content = r#"
pkgname=7zip
pkgver=26.01
_pkgver=${pkgver//./}
pkgrel=2
depends="glibc"
makedepends="make gcc"
"#;
        let result = parse_content(content).unwrap();
        assert_eq!(result.pkgname, "7zip");
        assert_eq!(result.pkgver, "26.01");
        assert_eq!(result.depends, vec!["glibc"]);
        assert_eq!(result.makedepends, vec!["make", "gcc"]);
        assert!(result.extra_vars.contains_key("_pkgver"));
    }
}
