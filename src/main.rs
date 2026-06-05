use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{BufReader, Read};
use std::os::unix::fs as unix_fs;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, anyhow, bail};
use chrono::Utc;
use gettextrs::{LocaleCategory, bind_textdomain_codeset, bindtextdomain, gettext, setlocale, textdomain};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tar::Archive;
use tempfile::TempDir;
use walkdir::WalkDir;

const INDEX_FILE: &str = "index.json";
const INSTALLED_FILE: &str = "installed.json";

#[derive(Debug)]
enum CliCommand {
    FlatpakInstall { app: String },
    FlatpakSearch { query: String },
    RepoSearch { query: String },
    RepoInstall { package: String },
    UpdateDb,
    UpdateAll,
}

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default)]
    general: GeneralConfig,
    #[serde(default)]
    repositories: Vec<RepoConfig>,
}

#[derive(Debug, Deserialize)]
struct GeneralConfig {
    state_dir: Option<PathBuf>,
    install_root: Option<PathBuf>,
    flatpak_remote: Option<String>,
    bubblewrap_level: Option<u8>,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            state_dir: None,
            install_root: None,
            flatpak_remote: None,
            bubblewrap_level: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct RepoConfig {
    name: String,
    index_url: Option<String>,
    base_url: Option<String>,
    repo_dir: Option<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RepoIndex {
    packages: Vec<PackageManifest>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PackageManifest {
    name: String,
    version: String,
    description: Option<String>,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    provides: Vec<String>,
    #[serde(default)]
    replaces: Vec<String>,
    source_url: Option<String>,
    source_sha256: Option<String>,
    #[serde(default)]
    distfiles: Vec<String>,
    checksum: Option<String>,
    build_steps: Vec<String>,
    #[serde(default)]
    install_map: Vec<InstallMap>,
}

impl PackageManifest {
    fn all_names(&self) -> impl Iterator<Item = &str> {
        std::iter::once(self.name.as_str())
            .chain(self.aliases.iter().map(String::as_str))
            .chain(self.provides.iter().map(String::as_str))
            .chain(self.replaces.iter().map(String::as_str))
    }

    fn source_url_resolved(&self) -> Result<String> {
        if let Some(url) = self.source_url.as_deref() {
            return Ok(self.expand_template(url));
        }
        self.distfiles
            .first()
            .map(|s| self.expand_template(s))
            .ok_or_else(|| {
                anyhow!(
                    "{} {}",
                    tr("package has neither source_url nor distfiles"),
                    self.name
                )
            })
    }

    fn expand_template(&self, input: &str) -> String {
        input
            .replace("${version//./}", &self.version.replace('.', ""))
            .replace("${version}", &self.version)
            .replace("${pkgname}", &self.name)
            .replace("${name}", &self.name)
    }

    fn checksum_resolved(&self) -> Result<&str> {
        if let Some(sum) = self.source_sha256.as_deref() {
            return Ok(sum);
        }
        self.checksum
            .as_deref()
            .ok_or_else(|| {
                anyhow!(
                    "{} {}",
                    tr("package has neither source_sha256 nor checksum"),
                    self.name
                )
            })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct InstallMap {
    from: String,
    to: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct InstalledPackage {
    version: String,
    repo: String,
    updated_at: String,
}

type InstalledDb = HashMap<String, InstalledPackage>;

#[derive(Debug)]
struct RuntimePaths {
    state_dir: PathBuf,
    install_root: PathBuf,
    indices_dir: PathBuf,
    cache_dir: PathBuf,
    build_dir: PathBuf,
    generations_dir: PathBuf,
    current_link: PathBuf,
    installed_db: PathBuf,
}

fn main() {
    init_i18n();
    if let Err(err) = run() {
        eprintln!("pag: {err:#}");
        std::process::exit(1);
    }
}

fn init_i18n() {
    let _ = setlocale(LocaleCategory::LcAll, "");
    let locale_dir = env::var("PAG_LOCALE_DIR").unwrap_or_else(|_| "locale".to_string());
    let _ = bindtextdomain("pag", &locale_dir);
    let _ = bind_textdomain_codeset("pag", "UTF-8");
    let _ = textdomain("pag");
}

fn tr(msgid: &str) -> String {
    gettext(msgid).to_string()
}

fn run() -> Result<()> {
    let cmd = parse_cli(env::args().collect())?;
    let cfg = load_config()?;
    let paths = build_runtime_paths(&cfg)?;
    ensure_runtime_layout(&paths)?;

    match cmd {
        CliCommand::FlatpakInstall { app } => flatpak_install(&cfg, &app),
        CliCommand::FlatpakSearch { query } => flatpak_search(&query),
        CliCommand::RepoSearch { query } => {
            ensure_repositories_configured(&cfg)?;
            repo_search(&paths, &query)
        }
        CliCommand::RepoInstall { package } => {
            ensure_repositories_configured(&cfg)?;
            repo_install(&cfg, &paths, &package)
        }
        CliCommand::UpdateDb => {
            ensure_repositories_configured(&cfg)?;
            update_db(&cfg, &paths)
        }
        CliCommand::UpdateAll => {
            ensure_repositories_configured(&cfg)?;
            update_all(&cfg, &paths)
        }
    }
}

fn ensure_repositories_configured(cfg: &Config) -> Result<()> {
    if cfg.repositories.is_empty() {
        bail!("{}", tr("no repositories configured in pag.conf"));
    }
    Ok(())
}

fn parse_cli(args: Vec<String>) -> Result<CliCommand> {
    if args.len() < 2 {
        bail!(usage());
    }

    let cmd = args[1].as_str();
    match cmd {
        "-f" => {
            let app = args
                .get(2)
                .context(tr("missing app name for -f"))?
                .to_string();
            Ok(CliCommand::FlatpakInstall { app })
        }
        "-fs" => {
            let query = args
                .get(2)
                .context(tr("missing search query for -fs"))?
                .to_string();
            Ok(CliCommand::FlatpakSearch { query })
        }
        "-s" => {
            let query = args
                .get(2)
                .context(tr("missing search query for -s"))?
                .to_string();
            Ok(CliCommand::RepoSearch { query })
        }
        "-i" => {
            let package = args
                .get(2)
                .context(tr("missing package name for -i"))?
                .to_string();
            Ok(CliCommand::RepoInstall { package })
        }
        "-u" => Ok(CliCommand::UpdateDb),
        "-uall" => Ok(CliCommand::UpdateAll),
        _ => bail!(usage()),
    }
}

fn usage() -> String {
    tr(
        "usage:\n  pag -f <app-id>   install app from Flatpak\n  pag -fs <query>   search in Flatpak\n  pag -s <query>    search packages in pag repositories\n  pag -i <package>  install package from repository\n  pag -u            refresh local package indices\n  pag -uall         atomically update installed packages",
    )
}

fn load_config() -> Result<Config> {
    let path = resolve_config_path()?;
    let config_dir = path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("{}: {}", tr("cannot read config file"), path.display()))?;
    let mut cfg: Config = toml::from_str(&raw)
        .with_context(|| format!("{}: {}", tr("invalid TOML file"), path.display()))?;

    cfg.general.state_dir = cfg
        .general
        .state_dir
        .take()
        .map(|p| expand_and_resolve_path(&p, &config_dir));
    cfg.general.install_root = cfg
        .general
        .install_root
        .take()
        .map(|p| expand_and_resolve_path(&p, &config_dir));
    for repo in &mut cfg.repositories {
        repo.repo_dir = repo
            .repo_dir
            .take()
            .map(|p| expand_and_resolve_path(&p, &config_dir));
    }

    Ok(cfg)
}

fn resolve_config_path() -> Result<PathBuf> {
    if let Ok(path) = env::var("PAG_CONFIG") {
        return Ok(PathBuf::from(path));
    }

    let local = PathBuf::from("pag.conf");
    if local.exists() {
        return Ok(local);
    }

    if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
        let xdg = PathBuf::from(xdg_config_home).join("pag/pag.conf");
        if xdg.exists() {
            return Ok(xdg);
        }
    }

    if let Ok(home) = env::var("HOME") {
        let user_cfg = PathBuf::from(home).join(".config/pag/pag.conf");
        if user_cfg.exists() {
            return Ok(user_cfg);
        }
    }

    let etc_dir = PathBuf::from("/etc/pag/pag.conf");
    if etc_dir.exists() {
        return Ok(etc_dir);
    }

    let etc = PathBuf::from("/etc/pag.conf");
    if etc.exists() {
        return Ok(etc);
    }

    bail!(
        "{}",
        tr("pag.conf not found (checked: ./pag.conf, /etc/pag.conf, $PAG_CONFIG)")
    );
}

fn build_runtime_paths(cfg: &Config) -> Result<RuntimePaths> {
    let home = env::var("HOME").context(tr("missing HOME environment variable"))?;
    let default_state = if let Ok(xdg_state_home) = env::var("XDG_STATE_HOME") {
        PathBuf::from(xdg_state_home).join("pag")
    } else {
        PathBuf::from(&home).join(".local/state/pag")
    };

    let state_dir = cfg
        .general
        .state_dir
        .clone()
        .unwrap_or(default_state);
    let install_root = cfg
        .general
        .install_root
        .clone()
        .unwrap_or_else(|| state_dir.join("rootfs"));

    let indices_dir = state_dir.join("indices");
    let cache_dir = state_dir.join("cache");
    let build_dir = state_dir.join("build");
    let generations_dir = state_dir.join("generations");
    let current_link = state_dir.join("current");
    let installed_db = state_dir.join(INSTALLED_FILE);

    Ok(RuntimePaths {
        state_dir,
        install_root,
        indices_dir,
        cache_dir,
        build_dir,
        generations_dir,
        current_link,
        installed_db,
    })
}

fn expand_and_resolve_path(path: &Path, base_dir: &Path) -> PathBuf {
    let raw = path.to_string_lossy();
    if raw == "~" {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home);
        }
    }

    if let Some(rest) = raw.strip_prefix("~/") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }

    if path.is_relative() {
        return base_dir.join(path);
    }

    path.to_path_buf()
}

fn ensure_runtime_layout(paths: &RuntimePaths) -> Result<()> {
    fs::create_dir_all(&paths.state_dir)?;
    fs::create_dir_all(&paths.indices_dir)?;
    fs::create_dir_all(&paths.cache_dir)?;
    fs::create_dir_all(&paths.build_dir)?;
    fs::create_dir_all(&paths.generations_dir)?;
    fs::create_dir_all(&paths.install_root)?;
    Ok(())
}

fn flatpak_search(query: &str) -> Result<()> {
    ensure_command_exists("flatpak")?;
    let status = Command::new("flatpak")
        .arg("search")
        .arg(query)
        .status()
        .context(tr("failed to execute flatpak search"))?;

    if !status.success() {
        bail!("{}", tr("flatpak search failed"));
    }
    Ok(())
}

fn flatpak_install(cfg: &Config, app: &str) -> Result<()> {
    ensure_command_exists("flatpak")?;
    let remote = cfg
        .general
        .flatpak_remote
        .as_deref()
        .unwrap_or("flathub");

    let status = Command::new("flatpak")
        .arg("install")
        .arg("-y")
        .arg(remote)
        .arg(app)
        .status()
        .context(tr("failed to execute flatpak install"))?;

    if !status.success() {
        bail!("{}", tr("flatpak install failed"));
    }
    Ok(())
}

fn update_db(cfg: &Config, paths: &RuntimePaths) -> Result<()> {
    for repo in &cfg.repositories {
        let index = if let Some(repo_dir) = &repo.repo_dir {
            load_repo_from_paginfo(repo_dir)
                .with_context(|| {
                    format!("{}: {}", tr("failed to build index from pag.info"), repo.name)
                })?
        } else {
            let index_url = repo
                .index_url
                .as_deref()
                .ok_or_else(|| anyhow!("{}: {}", tr("repo has neither index_url nor repo_dir"), repo.name))?;
            let body = http_get_to_string(index_url)
                .with_context(|| format!("{}: {}", tr("failed to fetch repository index"), repo.name))?;
            serde_json::from_str(&body)
                .with_context(|| format!("{} {}", tr("invalid repository index"), repo.name))?
        };

        let body = serde_json::to_string_pretty(&index)?;

        let target = paths.indices_dir.join(format!("{}.{}", repo.name, INDEX_FILE));
        atomic_write(&target, body.as_bytes())?;
        println!("{}: {}", tr("updated index"), repo.name);
    }
    Ok(())
}

fn repo_search(paths: &RuntimePaths, query: &str) -> Result<()> {
    let indices = load_all_indices(paths)?;
    let query_l = query.to_lowercase();
    let mut found = 0usize;

    for (repo, index) in indices {
        for pkg in index.packages {
            let hit_name = pkg.name.to_lowercase().contains(&query_l)
                || pkg.aliases.iter().any(|a| a.to_lowercase().contains(&query_l))
                || pkg.provides.iter().any(|p| p.to_lowercase().contains(&query_l))
                || pkg.replaces.iter().any(|r| r.to_lowercase().contains(&query_l));
            let hit_desc = pkg
                .description
                .as_deref()
                .map(|d| d.to_lowercase().contains(&query_l))
                .unwrap_or(false);

            if hit_name || hit_desc {
                found += 1;
                println!(
                    "[{repo}] {} {} - {}",
                    pkg.name,
                    pkg.version,
                    pkg.description.unwrap_or_else(|| tr("no description"))
                );
            }
        }
    }

    if found == 0 {
        println!("{}: {query}", tr("no results for"));
    }
    Ok(())
}

fn repo_install(cfg: &Config, paths: &RuntimePaths, package: &str) -> Result<()> {
    let (repo_cfg, manifest) = find_package(cfg, paths, package)?;
    if manifest.name != package {
        println!("{} '{}' -> '{}'", tr("alias mapping"), package, manifest.name);
    }
    let gen_dir = active_or_new_generation(paths)?;
    build_and_install_package(paths, &repo_cfg, &manifest, &gen_dir, bubblewrap_level(cfg))?;
    mark_installed(paths, &manifest, &repo_cfg.name)?;
    println!("{}: {} {}", tr("installed"), manifest.name, manifest.version);
    Ok(())
}

fn update_all(cfg: &Config, paths: &RuntimePaths) -> Result<()> {
    let installed = load_installed_db(paths)?;
    if installed.is_empty() {
        println!("{}", tr("no packages to update"));
        return Ok(());
    }

    let indices = load_all_indices(paths)?;
    let stage = TempDir::new_in(&paths.generations_dir).context(tr("failed to create stage directory"))?;
    let stage_root = stage.path().join("root");
    fs::create_dir_all(&stage_root)?;

    if let Some(current) = resolve_current_generation(paths)? {
        copy_tree(&current, &stage_root)?;
    }

    let mut new_db = installed.clone();
    let mut updated = 0usize;

    for (name, old_meta) in &installed {
        if let Some((repo_cfg, latest)) = find_package_in_indices(cfg, &indices, name) {
            if latest.version != old_meta.version {
                build_and_install_package(
                    paths,
                    &repo_cfg,
                    &latest,
                    &stage_root,
                    bubblewrap_level(cfg),
                )?;
                new_db.insert(
                    name.to_string(),
                    InstalledPackage {
                        version: latest.version.clone(),
                        repo: repo_cfg.name.clone(),
                        updated_at: Utc::now().to_rfc3339(),
                    },
                );
                updated += 1;
                println!("{}: {} -> {}", tr("updated"), name, latest.version);
            }
        }
    }

    if updated == 0 {
        println!("{}", tr("system is already up to date"));
        return Ok(());
    }

    let gen_name = format!("gen-{}", Utc::now().format("%Y%m%d%H%M%S"));
    let final_gen = paths.generations_dir.join(gen_name);
    fs::rename(stage_root, &final_gen).context(tr("failed to move staged generation"))?;

    atomic_switch_symlink(&paths.current_link, &final_gen)?;
    write_installed_db(paths, &new_db)?;
    println!("{}, {}: {updated}", tr("atomic update completed"), tr("packages"));
    Ok(())
}

fn active_or_new_generation(paths: &RuntimePaths) -> Result<PathBuf> {
    if let Some(current) = resolve_current_generation(paths)? {
        return Ok(current);
    }

    let gen_name = format!("gen-{}", Utc::now().format("%Y%m%d%H%M%S"));
    let gen_dir = paths.generations_dir.join(gen_name);
    fs::create_dir_all(&gen_dir)?;
    atomic_switch_symlink(&paths.current_link, &gen_dir)?;
    Ok(gen_dir)
}

fn resolve_current_generation(paths: &RuntimePaths) -> Result<Option<PathBuf>> {
    if !paths.current_link.exists() {
        return Ok(None);
    }

    let target = fs::read_link(&paths.current_link)
        .with_context(|| format!("{}: {}", tr("failed to read symlink"), paths.current_link.display()))?;
    let absolute = if target.is_absolute() {
        target
    } else {
        paths.state_dir.join(target)
    };
    Ok(Some(absolute))
}

fn find_package(cfg: &Config, paths: &RuntimePaths, package: &str) -> Result<(RepoConfig, PackageManifest)> {
    let indices = load_all_indices(paths)?;
    if let Some(found) = find_package_in_indices(cfg, &indices, package) {
        return Ok(found);
    }

    let req = package.to_lowercase();
    let mut hints = Vec::new();
    for index in indices.values() {
        for pkg in &index.packages {
            if pkg.name.to_lowercase().contains(&req)
                || pkg.aliases.iter().any(|a| a.to_lowercase().contains(&req))
            {
                hints.push(pkg.name.clone());
            }
        }
    }
    hints.sort();
    hints.dedup();

    if hints.is_empty() {
        bail!("{}: {package}", tr("package or alias not found"));
    }

    let sample = hints.into_iter().take(5).collect::<Vec<_>>().join(", ");
    bail!(
        "{}: {package}; {}: {sample}",
        tr("package or alias not found"),
        tr("maybe")
    )
}

fn find_package_in_indices(
    cfg: &Config,
    indices: &HashMap<String, RepoIndex>,
    package: &str,
) -> Option<(RepoConfig, PackageManifest)> {
    let req = package.to_lowercase();

    for repo in &cfg.repositories {
        let Some(index) = indices.get(&repo.name) else {
            continue;
        };
        if let Some(pkg) = index
            .packages
            .iter()
            .find(|p| p.name.to_lowercase() == req)
            .cloned()
        {
            return Some((repo.clone(), pkg));
        }
    }

    for repo in &cfg.repositories {
        let Some(index) = indices.get(&repo.name) else {
            continue;
        };
        if let Some(pkg) = index
            .packages
            .iter()
            .find(|p| p.all_names().any(|n| n.eq_ignore_ascii_case(&req)))
            .cloned()
        {
            return Some((repo.clone(), pkg));
        }
    }
    None
}

fn build_and_install_package(
    paths: &RuntimePaths,
    repo_cfg: &RepoConfig,
    manifest: &PackageManifest,
    install_destination: &Path,
    bubblewrap_lvl: u8,
) -> Result<()> {
    let temp = TempDir::new_in(&paths.build_dir)?;
    let manifest_source = manifest.source_url_resolved()?;
    let src_archive_name = manifest_source
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or("source.tar");
    let src_archive = temp.path().join(src_archive_name);

    fetch_source_to_file(repo_cfg, &manifest_source, &src_archive)?;
    verify_sha256(&src_archive, manifest.checksum_resolved()?)?;

    let src_dir = temp.path().join("src");
    fs::create_dir_all(&src_dir)?;
    safe_unpack_tar(&src_archive, &src_dir)?;

    for step in &manifest.build_steps {
        run_build_step(&src_dir, step, bubblewrap_lvl)?;
    }

    install_built_artifacts(&src_dir, install_destination, &manifest.install_map)?;
    Ok(())
}

fn install_built_artifacts(src_root: &Path, install_root: &Path, map: &[InstallMap]) -> Result<()> {
    if map.is_empty() {
        bail!("{}", tr("package does not contain install_map"));
    }

    for item in map {
        let from = safe_join(src_root, Path::new(&item.from))?;
        if !from.exists() {
            bail!("{}: {}", tr("missing artifact after build"), from.display());
        }

        let rel_to = Path::new(&item.to);
        if rel_to.is_absolute() {
            bail!("{}: {}", tr("install_map.to must be a relative path"), item.to);
        }

        let to = safe_join(install_root, rel_to)?;
        if from.is_dir() {
            copy_tree(&from, &to)?;
        } else {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

fn run_build_step(work_dir: &Path, step: &str, bubblewrap_lvl: u8) -> Result<()> {
    if bubblewrap_lvl == 0 {
        let status = Command::new("sh")
            .arg("-eu")
            .arg("-c")
            .arg(step)
            .current_dir(work_dir)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .stdin(Stdio::null())
            .status()
            .with_context(|| format!("{}: {step}", tr("failed to run build step")))?;

        if !status.success() {
            bail!("{}: {step}", tr("build step failed"));
        }
        return Ok(());
    }

    ensure_command_exists("bwrap")?;

    let mut cmd = Command::new("bwrap");
    cmd.arg("--die-with-parent")
        .arg("--new-session")
        .arg("--unshare-all")
        .arg("--proc")
        .arg("/proc")
        .arg("--dev")
        .arg("/dev")
        .arg("--tmpfs")
        .arg("/tmp")
        .arg("--bind")
        .arg(work_dir)
        .arg("/work")
        .arg("--chdir")
        .arg("/work");

    if bubblewrap_lvl <= 2 {
        cmd.arg("--share-net");
    }

    for p in ["/usr", "/bin", "/lib", "/lib64", "/etc"] {
        let host = Path::new(p);
        if host.exists() {
            cmd.arg("--ro-bind").arg(host).arg(host);
        }
    }

    if bubblewrap_lvl >= 4 {
        cmd.arg("--cap-drop").arg("ALL");
    }

    if bubblewrap_lvl >= 2 {
        cmd.arg("--clearenv")
            .arg("--setenv")
            .arg("PATH")
            .arg("/usr/bin:/bin")
            .arg("--setenv")
            .arg("HOME")
            .arg("/tmp")
            .arg("--setenv")
            .arg("USER")
            .arg("builder");
    }

    if bubblewrap_lvl >= 5 {
        cmd.arg("--unshare-user-try");
    }

    cmd.arg("sh").arg("-eu").arg("-c").arg(step);

    let status = cmd
        .stdin(Stdio::null())
        .status()
        .with_context(|| format!("{}: {step}", tr("failed to run build step in bubblewrap")))?;

    if !status.success() {
        bail!("{}: {step}", tr("build step failed"));
    }
    Ok(())
}

fn bubblewrap_level(cfg: &Config) -> u8 {
    cfg.general.bubblewrap_level.unwrap_or(0).clamp(0, 5)
}

fn load_all_indices(paths: &RuntimePaths) -> Result<HashMap<String, RepoIndex>> {
    let mut out = HashMap::new();
    for entry in fs::read_dir(&paths.indices_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension() != Some(OsStr::new("json")) {
            continue;
        }

        let raw = fs::read_to_string(&path)?;
        let idx: RepoIndex = serde_json::from_str(&raw)
            .with_context(|| format!("{}: {}", tr("invalid index"), path.display()))?;

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .trim_end_matches(".index")
            .to_string();

        out.insert(name, idx);
    }

    if out.is_empty() {
        bail!("{}", tr("no local indices found; run: pag -u"));
    }
    Ok(out)
}

fn ensure_command_exists(binary: &str) -> Result<()> {
    if !command_exists(binary)? {
        bail!("{}: {binary}", tr("required command is not available"));
    }
    Ok(())
}

fn command_exists(binary: &str) -> Result<bool> {
    let status = Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {} >/dev/null 2>&1", binary))
        .status()?;
    Ok(status.success())
}

fn load_repo_from_paginfo(packages_dir: &Path) -> Result<RepoIndex> {
    if !packages_dir.exists() {
        bail!("{}: {}", tr("repository directory does not exist"), packages_dir.display());
    }

    let mut packages = Vec::new();
    for entry in fs::read_dir(packages_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let info_path = path.join("pag.info");
        if !info_path.exists() {
            continue;
        }

        let raw = fs::read_to_string(&info_path)
            .with_context(|| format!("{} {}", tr("cannot read"), info_path.display()))?;
        let pkg: PackageManifest = toml::from_str(&raw)
            .with_context(|| format!("{} {}", tr("invalid TOML in"), info_path.display()))?;
        packages.push(pkg);
    }

    if packages.is_empty() {
        bail!("{} {}", tr("no pag.info files in"), packages_dir.display());
    }

    Ok(RepoIndex { packages })
}

fn fetch_source_to_file(repo_cfg: &RepoConfig, source_ref: &str, target: &Path) -> Result<()> {
    if source_ref.starts_with("http://") || source_ref.starts_with("https://") {
        return download_to_file(source_ref, target);
    }

    if let Some(repo_dir) = &repo_cfg.repo_dir {
        let src = safe_join(repo_dir, Path::new(source_ref))?;
        if !src.exists() {
            bail!("{}: {}", tr("local source does not exist"), src.display());
        }
        fs::copy(src, target)?;
        return Ok(());
    }

    if let Some(base_url) = repo_cfg.base_url.as_deref() {
        let source_url = format!("{}/{}", base_url.trim_end_matches('/'), source_ref);
        return download_to_file(&source_url, target);
    }

    bail!(
        "{} {}: {}",
        tr("cannot fetch source"),
        source_ref,
        tr("repo has neither repo_dir nor base_url")
    )
}

fn http_get_to_string(url: &str) -> Result<String> {
    let response = ureq::get(url)
        .call()
        .with_context(|| format!("{}: {url}", tr("HTTP GET failed")))?;
    let mut body = response.into_body();
    body.read_to_string()
        .map_err(|e| anyhow!("{} {url}: {e}", tr("failed to read HTTP response")))
}

fn download_to_file(url: &str, target: &Path) -> Result<()> {
    let response = ureq::get(url)
        .call()
        .with_context(|| format!("{}: {url}", tr("download failed")))?;
    let mut reader = response.into_body().into_reader();

    let tmp = target.with_extension("tmp");
    let mut out = fs::File::create(&tmp)?;
    std::io::copy(&mut reader, &mut out)?;
    fs::rename(tmp, target)?;
    Ok(())
}

fn verify_sha256(file: &Path, expected_hex: &str) -> Result<()> {
    let mut hasher = Sha256::new();
    let mut reader = BufReader::new(fs::File::open(file)?);
    let mut buf = [0u8; 16 * 1024];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let digest = hasher.finalize();
    let got = digest.iter().map(|b| format!("{b:02x}")).collect::<String>();
    if got != expected_hex.to_lowercase() {
        bail!(
            "{} {}: {} {}, {} {}",
            tr("SHA256 checksum mismatch for"),
            file.display(),
            tr("expected"),
            expected_hex,
            tr("got"),
            got
        );
    }
    Ok(())
}

fn safe_unpack_tar(archive_path: &Path, destination: &Path) -> Result<()> {
    let mut probe = fs::File::open(archive_path)?;
    let mut magic = [0u8; 2];
    let read = probe.read(&mut magic)?;
    drop(probe);

    let file = fs::File::open(archive_path)?;
    let reader: Box<dyn Read> = if read == 2 && magic == [0x1f, 0x8b] {
        Box::new(flate2::read::GzDecoder::new(file))
    } else {
        Box::new(file)
    };

    let mut archive = Archive::new(reader);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let rel = entry.path()?.to_path_buf();
        let out_path = safe_join(destination, &rel)?;

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)?;
        }
        entry.unpack(&out_path)?;
    }
    Ok(())
}

fn safe_join(base: &Path, rel: &Path) -> Result<PathBuf> {
    if rel.is_absolute() {
        bail!("{}: {}", tr("absolute path is not allowed"), rel.display());
    }

    for comp in rel.components() {
        if matches!(comp, Component::ParentDir | Component::RootDir | Component::Prefix(_)) {
            bail!("{}: {}", tr("forbidden path (path traversal)"), rel.display());
        }
    }
    Ok(base.join(rel))
}

fn atomic_write(target: &Path, content: &[u8]) -> Result<()> {
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = target.with_extension("tmp");
    fs::write(&tmp, content)?;
    fs::rename(tmp, target)?;
    Ok(())
}

fn atomic_switch_symlink(link_path: &Path, target: &Path) -> Result<()> {
    if let Some(parent) = link_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let tmp_link = link_path.with_extension("tmp");
    if tmp_link.exists() {
        fs::remove_file(&tmp_link)?;
    }
    unix_fs::symlink(target, &tmp_link)?;
    fs::rename(tmp_link, link_path)?;
    Ok(())
}

fn copy_tree(src: &Path, dst: &Path) -> Result<()> {
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let path = entry.path();
        let rel = path.strip_prefix(src)?;
        if rel.as_os_str().is_empty() {
            continue;
        }

        let target = dst.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &target)?;
        } else if entry.file_type().is_symlink() {
            let linked = fs::read_link(path)?;
            unix_fs::symlink(linked, &target)?;
        }
    }
    Ok(())
}

fn load_installed_db(paths: &RuntimePaths) -> Result<InstalledDb> {
    if !paths.installed_db.exists() {
        return Ok(HashMap::new());
    }
    let raw = fs::read_to_string(&paths.installed_db)?;
    let db: InstalledDb = serde_json::from_str(&raw)?;
    Ok(db)
}

fn write_installed_db(paths: &RuntimePaths, db: &InstalledDb) -> Result<()> {
    let raw = serde_json::to_vec_pretty(db)?;
    atomic_write(&paths.installed_db, &raw)
}

fn mark_installed(paths: &RuntimePaths, manifest: &PackageManifest, repo: &str) -> Result<()> {
    let mut db = load_installed_db(paths)?;
    db.insert(
        manifest.name.clone(),
        InstalledPackage {
            version: manifest.version.clone(),
            repo: repo.to_string(),
            updated_at: Utc::now().to_rfc3339(),
        },
    );
    write_installed_db(paths, &db)
}
