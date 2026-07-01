use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmsConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub build: BuildConfig,
    pub forgejo: ForgejoConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub stage3_path: String,
    pub build_space: String,
    pub max_concurrent_builds: usize,
    pub build_timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgejoConfig {
    pub base_url: String,
    pub api_token: String,
    pub community_repo: String,
    pub webhook_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub session_secret: String,
    pub admin_username: String,
    pub admin_password_hash: String,
    pub token_expiry_days: u32,
}

impl Default for CmsConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                bind: "0.0.0.0".into(),
                port: 3005,
                cors_origins: vec!["http://localhost:4321".into(), "https://cms.paganlinux.eu".into()],
            },
            database: DatabaseConfig {
                path: "/opt/pagan-cms/cms.db".into(),
            },
            build: BuildConfig {
                stage3_path: "/var/pagan-os/stage3-base".into(),
                build_space: "/var/pagan-os/build-space".into(),
                max_concurrent_builds: 2,
                build_timeout_seconds: 7200,
            },
            forgejo: ForgejoConfig {
                base_url: "https://git.paganlinux.eu".into(),
                api_token: String::new(),
                community_repo: "pagan-community".into(),
                webhook_secret: String::new(),
            },
            auth: AuthConfig {
                session_secret: "change-me-in-production".into(),
                admin_username: "admin".into(),
                admin_password_hash: String::new(),
                token_expiry_days: 7,
            },
        }
    }
}

impl CmsConfig {
    pub fn load() -> anyhow::Result<Self> {
        let path = std::env::var("PAG_CMS_CONFIG")
            .unwrap_or_else(|_| "/etc/pag/cms.toml".into());

        if let Ok(content) = std::fs::read_to_string(&path) {
            Ok(toml::from_str(&content)?)
        } else {
            let cfg = Self::default();
            // Save defaults
            if let Some(parent) = PathBuf::from(&path).parent() {
                std::fs::create_dir_all(parent).ok();
            }
            std::fs::write(&path, toml::to_string_pretty(&cfg)?).ok();
            Ok(cfg)
        }
    }
}
