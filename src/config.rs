// ────────────────────────────────────────────────────────────────────────────────
// Pag — Moduł konfiguracji
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Główna konfiguracja PAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Ścieżka do pliku konfiguracyjnego
    #[serde(skip)]
    pub config_path: PathBuf,

    /// Ogólne ustawienia
    #[serde(default)]
    pub general: GeneralConfig,

    /// Konfiguracja repozytoriów
    #[serde(default)]
    pub repositories: RepositoriesConfig,

    /// Konfiguracja budowania
    #[serde(default)]
    pub build: BuildConfig,

    /// Konfiguracja migawek
    #[serde(default)]
    pub snapshots: SnapshotConfig,

    /// Konfiguracja GPG
    #[serde(default)]
    pub gpg: GpgConfig,

    /// Ścieżki
    #[serde(default)]
    pub paths: PathsConfig,

    /// Sieć
    #[serde(default)]
    pub network: NetworkConfig,
}

impl Config {
    /// Wczytaj konfigurację z domyślnej ścieżki lub podanej
    pub fn load(custom_path: Option<&str>) -> Result<Self> {
        let config_path = if let Some(p) = custom_path {
            PathBuf::from(p)
        } else {
            let mut default = dirs::config_dir()
                .context("Cannot find config directory")?;
            default.push("pag");
            default.push("pag.conf");
            default
        };

        let mut config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .with_context(|| format!("Failed to read config file: {:?}", config_path))?;
            toml::from_str::<Config>(&content)
                .with_context(|| "Failed to parse config file")?
        } else {
            Config::default()
        };

        config.config_path = config_path;

        // Upewnij się, że katalogi istnieją
        config.ensure_directories()?;

        Ok(config)
    }

    /// Zapisz konfigurację
    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    /// Twórz wymagane katalogi
    pub fn ensure_directories(&self) -> Result<()> {
        let dirs = [
            &self.paths.db_path,
            &self.paths.cache_path,
            &self.paths.pkg_cache_path,
            &self.paths.build_path,
            &self.paths.keyring_path,
            &self.paths.snapshot_path,
            &self.paths.lock_path,
        ];

        for dir in &dirs {
            fs::create_dir_all(dir)?;
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            config_path: PathBuf::from("/etc/pag/pag.conf"),
            general: GeneralConfig::default(),
            repositories: RepositoriesConfig::default(),
            build: BuildConfig::default(),
            snapshots: SnapshotConfig::default(),
            gpg: GpgConfig::default(),
            paths: PathsConfig::default(),
            network: NetworkConfig::default(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────────
// Podkonfiguracje
// ────────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Domyślny język
    #[serde(default = "default_lang")]
    pub lang: String,

    /// Architektura systemu
    #[serde(default = "default_arch")]
    pub arch: String,

    /// Czy używać kolorów
    #[serde(default = "default_true")]
    pub color: bool,

    /// Równoległe pobieranie
    #[serde(default = "default_parallel")]
    pub parallel_downloads: usize,

    /// Poziom kompresji pakietów
    #[serde(default)]
    pub compression_level: u32,

    /// Algorytm kompresji (zstd, xz, gzip, bzip2)
    #[serde(default = "default_compression")]
    pub compression: String,

    /// Automatyczne czyszczenie pamięci podręcznej po instalacji
    #[serde(default)]
    pub auto_clean_cache: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        GeneralConfig {
            lang: default_lang(),
            arch: default_arch(),
            color: true,
            parallel_downloads: default_parallel(),
            compression_level: 19,
            compression: default_compression(),
            auto_clean_cache: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoriesConfig {
    /// Lista repozytoriów
    #[serde(default = "default_repos")]
    pub repos: Vec<Repository>,

    /// Sprawdzaj dostępność aktualizacji przy starcie
    #[serde(default)]
    pub check_updates_on_start: bool,

    /// Interwał sprawdzania aktualizacji (w godzinach)
    #[serde(default = "default_check_interval")]
    pub check_interval_hours: u64,
}

impl Default for RepositoriesConfig {
    fn default() -> Self {
        RepositoriesConfig {
            repos: default_repos(),
            check_updates_on_start: true,
            check_interval_hours: default_check_interval(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    /// Name repozytorium
    pub name: String,

    /// URL bazy danych repozytorium
    pub url: String,

    /// Czy repozytorium jest enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Poziom zaufania (0-100)
    #[serde(default = "default_trust")]
    pub trust_level: u8,

    /// Sygnatariusz GPG (opcjonalny)
    #[serde(default)]
    pub sig_level: String,

    /// Klucz GPG do weryfikacji
    pub gpg_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Optymalizacje CFLAGS
    #[serde(default = "default_cflags")]
    pub cflags: String,

    /// Optymalizacje CXXFLAGS
    #[serde(default = "default_cxxflags")]
    pub cxxflags: String,

    /// MAKEOPTS (-j)
    #[serde(default = "default_makeopts")]
    pub makeopts: String,

    /// Katalog roboczy budowania
    #[serde(default)]
    pub build_dir: String,

    /// Użytkownik budujący (nie root)
    pub build_user: Option<String>,

    /// Czy używać sandboxa/namespace
    #[serde(default = "default_true")]
    pub use_namespace: bool,

    /// Maksymalny rozmiar logu budowania (MB)
    #[serde(default = "default_log_size")]
    pub max_log_size: u64,
}

impl Default for BuildConfig {
    fn default() -> Self {
        BuildConfig {
            cflags: default_cflags(),
            cxxflags: default_cxxflags(),
            makeopts: default_makeopts(),
            build_dir: String::new(),
            build_user: None,
            use_namespace: true,
            max_log_size: default_log_size(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotConfig {
    /// Czy migawki są enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Typ migawek (btrfs, zfs, rsync)
    #[serde(default = "default_snapshot_type")]
    pub snapshot_type: String,

    /// Automatyczne migawki przed operacjami
    #[serde(default = "default_true")]
    pub auto_pre: bool,

    /// Automatyczne migawki po operacjach
    #[serde(default = "default_true")]
    pub auto_post: bool,

    /// Maksymalna liczba migawek
    #[serde(default = "default_max_snapshots")]
    pub max_snapshots: u32,

    /// Prefix nazwy migawki
    #[serde(default = "default_snapshot_prefix")]
    pub prefix: String,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        SnapshotConfig {
            enabled: true,
            snapshot_type: default_snapshot_type(),
            auto_pre: true,
            auto_post: true,
            max_snapshots: default_max_snapshots(),
            prefix: default_snapshot_prefix(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpgConfig {
    /// Ścieżka do keyringu
    #[serde(default)]
    pub keyring_dir: String,

    /// Domyślny klucz do podpisywania
    pub default_key: Option<String>,

    /// Automatycznie pobieraj brakujące klucze
    #[serde(default)]
    pub auto_key_retrieve: bool,

    /// Serwer kluczy
    #[serde(default = "default_keyserver")]
    pub keyserver: String,
}

impl Default for GpgConfig {
    fn default() -> Self {
        GpgConfig {
            keyring_dir: String::new(),
            default_key: None,
            auto_key_retrieve: false,
            keyserver: default_keyserver(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    /// Baza danych pakietów
    #[serde(default = "default_db_path")]
    pub db_path: PathBuf,

    /// Pamięć podręczna
    #[serde(default = "default_cache_path")]
    pub cache_path: PathBuf,

    /// Pamięć podręczna pobranych pakietów
    #[serde(default = "default_pkg_cache_path")]
    pub pkg_cache_path: PathBuf,

    /// Katalog budowania
    #[serde(default = "default_build_path")]
    pub build_path: PathBuf,

    /// Keyring GPG
    #[serde(default = "default_keyring_path")]
    pub keyring_path: PathBuf,

    /// Katalog migawek
    #[serde(default = "default_snapshot_path")]
    pub snapshot_path: PathBuf,

    /// Plik blokady
    #[serde(default = "default_lock_path")]
    pub lock_path: PathBuf,

    /// Katalog tymczasowy
    #[serde(default = "default_tmp_path")]
    pub tmp_path: PathBuf,

    /// Katalog logów
    #[serde(default = "default_log_path")]
    pub log_path: PathBuf,
}

impl Default for PathsConfig {
    fn default() -> Self {
        PathsConfig {
            db_path: default_db_path(),
            cache_path: default_cache_path(),
            pkg_cache_path: default_pkg_cache_path(),
            build_path: default_build_path(),
            keyring_path: default_keyring_path(),
            snapshot_path: default_snapshot_path(),
            lock_path: default_lock_path(),
            tmp_path: default_tmp_path(),
            log_path: default_log_path(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Timeout połączenia (sekundy)
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,

    /// Maksymalna liczba ponownych prób
    #[serde(default = "default_retries")]
    pub max_retries: u32,

    /// Serwer proxy (opcjonalny)
    pub proxy: Option<String>,

    /// User-Agent
    #[serde(default = "default_user_agent")]
    pub user_agent: String,

    /// Weryfikuj certyfikaty SSL
    #[serde(default = "default_true")]
    pub verify_ssl: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            timeout_secs: default_timeout(),
            max_retries: default_retries(),
            proxy: None,
            user_agent: default_user_agent(),
            verify_ssl: true,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────────
// Domyślne wartości
// ────────────────────────────────────────────────────────────────────────────────

fn default_lang() -> String { "pl".into() }
fn default_arch() -> String { std::env::consts::ARCH.into() }
fn default_true() -> bool { true }
fn default_parallel() -> usize { 5 }
fn default_compression() -> String { "zstd".into() }
fn default_trust() -> u8 { 80 }
fn default_cflags() -> String { "-march=native -O2 -pipe -fstack-protector-strong".into() }
fn default_cxxflags() -> String { "-march=native -O2 -pipe -fstack-protector-strong".into() }
fn default_makeopts() -> String {
    num_cpus::get().to_string()
}
fn default_log_size() -> u64 { 100 }
fn default_snapshot_type() -> String { "btrfs".into() }
fn default_max_snapshots() -> u32 { 20 }
fn default_snapshot_prefix() -> String { "pag".into() }
fn default_keyserver() -> String { "hkps://keys.openpgp.org".into() }
fn default_check_interval() -> u64 { 24 }

fn default_db_path() -> PathBuf { PathBuf::from("/var/lib/pag/db") }
fn default_cache_path() -> PathBuf { PathBuf::from("/var/cache/pag") }
fn default_pkg_cache_path() -> PathBuf { PathBuf::from("/var/cache/pag/pkgs") }
fn default_build_path() -> PathBuf { PathBuf::from("/var/tmp/pag/build") }
fn default_keyring_path() -> PathBuf { PathBuf::from("/etc/pag/gnupg") }
fn default_snapshot_path() -> PathBuf { PathBuf::from("/var/lib/pag/snapshots") }
fn default_lock_path() -> PathBuf { PathBuf::from("/var/lib/pag/db.lock") }
fn default_tmp_path() -> PathBuf { PathBuf::from("/tmp/pag") }
fn default_log_path() -> PathBuf { PathBuf::from("/var/log/pag") }
fn default_timeout() -> u64 { 30 }
fn default_retries() -> u32 { 3 }
fn default_user_agent() -> String { format!("pag/{} (PaganLinux)", env!("CARGO_PKG_VERSION")) }

fn default_repos() -> Vec<Repository> {
    vec![
        Repository {
            name: "core".into(),
            url: "https://pagrepos.paganlinux.eu/x86_64".into(),
            enabled: true,
            trust_level: 100,
            sig_level: "required".into(),
            gpg_key: Some("0xPAGANLINUX2024".into()),
        },
        Repository {
            name: "community".into(),
            url: "https://pagrepos.paganlinux.eu/community/x86_64".into(),
            enabled: true,
            trust_level: 80,
            sig_level: "optional".into(),
            gpg_key: None,
        },
    ]
}
