// Konfiguracja menedżera pakietów pag

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Główna konfiguracja pag
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Konfiguracja ogólna
    #[serde(default)]
    pub general: GeneralConfig,

    /// Lista repozytoriów
    #[serde(default = "default_repositories")]
    pub repositories: Vec<Repository>,

    /// Konfiguracja sieci
    #[serde(default)]
    pub network: NetworkConfig,

    /// Konfiguracja bezpieczeństwa
    #[serde(default)]
    pub security: SecurityConfig,

    /// Konfiguracja Flatpak
    #[serde(default)]
    pub flatpak: FlatpakConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Katalog główny instalacji
    #[serde(default = "default_root")]
    pub root: String,

    /// Katalog bazy danych
    #[serde(default = "default_db_path")]
    pub db_path: String,

    /// Katalog cache
    #[serde(default = "default_cache_path")]
    pub cache_path: String,

    /// Katalog logów
    #[serde(default = "default_log_path")]
    pub log_path: String,

    /// Równoległość pobierania
    #[serde(default = "default_parallel_downloads")]
    pub parallel_downloads: usize,

    /// Domyślny język
    #[serde(default)]
    pub lang: String,

    /// Automatyczne czyszczenie cache po instalacji
    #[serde(default)]
    pub autoclean: bool,

    /// Sprawdzaj miejsce na dysku przed instalacją
    #[serde(default = "default_true")]
    pub check_disk_space: bool,

    /// Pokaż progres bary
    #[serde(default = "default_true")]
    pub show_progress: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    /// Nazwa repozytorium
    pub name: String,

    /// URL repozytorium
    pub url: String,

    /// Opcjonalny URL mirrora
    pub mirror: Option<String>,

    /// Czy repozytorium jest aktywne
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Priorytet (niższy = wyższy priorytet)
    #[serde(default)]
    pub priority: u32,

    /// Architektura (domyślnie: auto)
    #[serde(default = "default_arch")]
    pub arch: String,

    /// Gałąź (main, testing, community...)
    #[serde(default = "default_branch")]
    pub branch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Timeout w sekundach
    #[serde(default = "default_timeout")]
    pub timeout: u64,

    /// Liczba prób ponowienia
    #[serde(default)]
    pub retries: u32,

    /// Serwer proxy
    pub proxy: Option<String>,

    /// Limit prędkości pobierania (B/s, 0 = bez limitu)
    #[serde(default)]
    pub rate_limit: u64,

    /// User-Agent
    #[serde(default = "default_user_agent")]
    pub user_agent: String,

    /// Używaj IPv6
    #[serde(default = "default_true")]
    pub ipv6: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Wymagaj podpisów GPG
    #[serde(default = "default_true")]
    pub require_signatures: bool,

    /// Ścieżka do keyringa
    #[serde(default = "default_keyring_path")]
    pub keyring_path: String,

    /// Lista zaufanych fingerprintów
    #[serde(default)]
    pub trusted_keys: Vec<String>,

    /// Sprawdzaj checksumy
    #[serde(default = "default_true")]
    pub verify_checksums: bool,

    /// Używaj sandboxa przy instalacji
    #[serde(default = "default_true")]
    pub use_sandbox: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlatpakConfig {
    /// Czy integracja Flatpak jest włączona
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Flatpak remotes
    #[serde(default = "default_flatpak_remotes")]
    pub remotes: Vec<FlatpakRemote>,

    /// Automatyczne aktualizacje Flatpak
    #[serde(default)]
    pub auto_update: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlatpakRemote {
    pub name: String,
    pub url: String,
}

// Domyślne wartości

fn default_root() -> String { "/".into() }
fn default_db_path() -> String { "/var/lib/pag/db.sqlite".into() }
fn default_cache_path() -> String { "/var/cache/pag".into() }
fn default_log_path() -> String { "/var/log/pag".into() }
fn default_parallel_downloads() -> usize { 5 }
fn default_arch() -> String { std::env::consts::ARCH.into() }
fn default_branch() -> String { "main".into() }
fn default_timeout() -> u64 { 30 }
fn default_user_agent() -> String { format!("pag/{}", env!("CARGO_PKG_VERSION")) }
fn default_keyring_path() -> String { "/etc/pag/trusted.gpg".into() }
const fn default_true() -> bool { true }

fn default_repositories() -> Vec<Repository> {
    vec![
        Repository {
            name: "core".into(),
            url: "https://repos.paganlinux.eu/core".into(),
            mirror: None,
            enabled: true,
            priority: 0,
            arch: default_arch(),
            branch: "main".into(),
        },
        Repository {
            name: "extra".into(),
            url: "https://repos.paganlinux.eu/extra".into(),
            mirror: None,
            enabled: true,
            priority: 10,
            arch: default_arch(),
            branch: "main".into(),
        },
        Repository {
            name: "community".into(),
            url: "https://repos.paganlinux.eu/community".into(),
            mirror: None,
            enabled: true,
            priority: 20,
            arch: default_arch(),
            branch: "main".into(),
        },
    ]
}

fn default_flatpak_remotes() -> Vec<FlatpakRemote> {
    vec![
        FlatpakRemote {
            name: "flathub".into(),
            url: "https://flathub.org/repo/flathub.flatpakrepo".into(),
        },
    ]
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            root: default_root(),
            db_path: default_db_path(),
            cache_path: default_cache_path(),
            log_path: default_log_path(),
            parallel_downloads: default_parallel_downloads(),
            lang: String::new(),
            autoclean: false,
            check_disk_space: true,
            show_progress: true,
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            timeout: default_timeout(),
            retries: 3,
            proxy: None,
            rate_limit: 0,
            user_agent: default_user_agent(),
            ipv6: true,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            require_signatures: true,
            keyring_path: default_keyring_path(),
            trusted_keys: vec![],
            verify_checksums: true,
            use_sandbox: true,
        }
    }
}

impl Default for FlatpakConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            remotes: default_flatpak_remotes(),
            auto_update: false,
        }
    }
}

impl Config {
    /// Ładuje konfigurację z domyślnych ścieżek
    pub fn load(root: &str) -> anyhow::Result<Self> {
        let config_paths = vec![
            PathBuf::from(format!("{}/etc/pag/config.toml", root.trim_end_matches('/'))),
            PathBuf::from("/etc/pag/config.toml"),
        ];

        for path in &config_paths {
            if path.exists() {
                let content = std::fs::read_to_string(path)?;
                let mut cfg: Config = toml::from_str(&content)?;
                // Nadpisz root jeśli podano z CLI
                if root != "/" {
                    cfg.general.root = root.to_string();
                }
                return Ok(cfg);
            }
        }

        // Domyślna konfiguracja
        Ok(Config {
            general: GeneralConfig {
                root: root.to_string(),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    /// Zapisuje konfigurację do pliku
    pub fn save(&self, path: &str) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        if let Some(parent) = PathBuf::from(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Generuje domyślną konfigurację
    pub fn generate_default() -> String {
        let cfg = Config::default();
        let mut toml_str = String::from("# PaganLinux Package Manager - Konfiguracja\n");
        toml_str += "# https://paganlinux.eu/docs/pag-config\n\n";
        toml_str += &toml::to_string_pretty(&cfg).unwrap_or_default();
        toml_str
    }

    /// Pobiera aktywne repozytoria posortowane po priorytecie
    pub fn active_repos(&self) -> Vec<&Repository> {
        let mut repos: Vec<_> = self.repositories.iter()
            .filter(|r| r.enabled)
            .collect();
        repos.sort_by_key(|r| r.priority);
        repos
    }
}
