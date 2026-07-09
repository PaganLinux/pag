// ────────────────────────────────────────────────────────────────────────────────
// Pag — Lokalna baza danych pakietów (SQLite)
// ────────────────────────────────────────────────────────────────────────────────

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::package::metadata::{InstallReason, PackageMetadata, PackageFile};

/// Baza danych lokalnych pakietów
pub struct LocalDatabase {
    conn: Mutex<Connection>,
    db_path: PathBuf,
}

impl LocalDatabase {
    /// Otwórz lub utwórz bazę danych
    pub fn open(db_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(db_dir)?;
        let db_path = db_dir.join("local.db");

        let conn = Connection::open(&db_path)
            .with_context(|| format!("Failed to open database: {:?}", db_path))?;

        // Włącz WAL mode dla lepszej wydajności
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

        let db = LocalDatabase {
            conn: Mutex::new(conn),
            db_path,
        };

        db.initialize_tables()?;

        Ok(db)
    }

    /// Utwórz tabele
    fn initialize_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS packages (
                name            TEXT NOT NULL,
                version         TEXT NOT NULL,
                release         INTEGER NOT NULL DEFAULT 1,
                description     TEXT NOT NULL DEFAULT '',
                architecture    TEXT NOT NULL DEFAULT 'x86_64',
                url             TEXT,
                installed_size  INTEGER NOT NULL DEFAULT 0,
                archive_size    INTEGER NOT NULL DEFAULT 0,
                sha256          TEXT NOT NULL DEFAULT '',
                blake3          TEXT,
                pgp_signer      TEXT,
                build_date      TEXT NOT NULL,
                install_date    TEXT NOT NULL,
                install_reason  TEXT NOT NULL DEFAULT 'explicit',
                repository      TEXT,
                verified        INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (name)
            );

            CREATE TABLE IF NOT EXISTS licenses (
                package_name TEXT NOT NULL,
                license      TEXT NOT NULL,
                PRIMARY KEY (package_name, license),
                FOREIGN KEY (package_name) REFERENCES packages(name) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS groups_pkg (
                package_name TEXT NOT NULL,
                group_name   TEXT NOT NULL,
                PRIMARY KEY (package_name, group_name),
                FOREIGN KEY (package_name) REFERENCES packages(name) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS depends (
                package_name TEXT NOT NULL,
                dependency   TEXT NOT NULL,
                dep_type     TEXT NOT NULL DEFAULT 'depends',
                PRIMARY KEY (package_name, dependency, dep_type),
                FOREIGN KEY (package_name) REFERENCES packages(name) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS files (
                package_name TEXT NOT NULL,
                file_path    TEXT NOT NULL,
                file_size    INTEGER NOT NULL DEFAULT 0,
                file_mode    INTEGER NOT NULL DEFAULT 0,
                file_uid     INTEGER NOT NULL DEFAULT 0,
                file_gid     INTEGER NOT NULL DEFAULT 0,
                is_symlink   INTEGER NOT NULL DEFAULT 0,
                symlink_target TEXT,
                is_config    INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (package_name, file_path),
                FOREIGN KEY (package_name) REFERENCES packages(name) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS backups (
                package_name TEXT NOT NULL,
                file_path    TEXT NOT NULL,
                PRIMARY KEY (package_name, file_path),
                FOREIGN KEY (package_name) REFERENCES packages(name) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_depends_dep ON depends(dependency);
            CREATE INDEX IF NOT EXISTS idx_files_path ON files(file_path);
            CREATE INDEX IF NOT EXISTS idx_packages_install_reason ON packages(install_reason);
            ",
        )?;

        Ok(())
    }

    /// Dodaj pakiet do bazy
    pub fn add_package(&self, metadata: &PackageMetadata) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let tx = conn.unchecked_transaction()?;

        // Dodaj główny wpis
        tx.execute(
            "INSERT OR REPLACE INTO packages
             (name, version, release, description, architecture, url,
              installed_size, archive_size, sha256, blake3, pgp_signer,
              build_date, install_date, install_reason, repository, verified)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
            params![
                metadata.name,
                metadata.version,
                metadata.release,
                metadata.description,
                metadata.architecture,
                metadata.url,
                metadata.installed_size,
                metadata.archive_size,
                metadata.sha256,
                metadata.blake3,
                metadata.pgp_signer,
                metadata.build_date.to_rfc3339(),
                metadata.install_date.unwrap_or_else(Utc::now).to_rfc3339(),
                match metadata.install_reason {
                    InstallReason::Explicit => "explicit",
                    InstallReason::Dependency => "dependency",
                },
                metadata.repository,
                metadata.verified as i32,
            ],
        )?;

        // Dodaj licencje
        for lic in &metadata.license {
            tx.execute(
                "INSERT OR REPLACE INTO licenses (package_name, license) VALUES (?1, ?2)",
                params![metadata.name, lic],
            )?;
        }

        // Dodaj grupy
        for group in &metadata.groups {
            tx.execute(
                "INSERT OR REPLACE INTO groups_pkg (package_name, group_name) VALUES (?1, ?2)",
                params![metadata.name, group],
            )?;
        }

        // Dodaj zależności
        for dep in &metadata.depends {
            tx.execute(
                "INSERT OR REPLACE INTO depends (package_name, dependency, dep_type) VALUES (?1, ?2, 'depends')",
                params![metadata.name, dep],
            )?;
        }
        for dep in &metadata.opt_depends {
            tx.execute(
                "INSERT OR REPLACE INTO depends (package_name, dependency, dep_type) VALUES (?1, ?2, 'optdepends')",
                params![metadata.name, dep],
            )?;
        }
        for dep in &metadata.makedepends {
            tx.execute(
                "INSERT OR REPLACE INTO depends (package_name, dependency, dep_type) VALUES (?1, ?2, 'makedepends')",
                params![metadata.name, dep],
            )?;
        }

        // Dodaj konflikty
        for conflict in &metadata.conflicts {
            tx.execute(
                "INSERT OR REPLACE INTO depends (package_name, dependency, dep_type) VALUES (?1, ?2, 'conflicts')",
                params![metadata.name, conflict],
            )?;
        }

        // Dodaj provides
        for provide in &metadata.provides {
            tx.execute(
                "INSERT OR REPLACE INTO depends (package_name, dependency, dep_type) VALUES (?1, ?2, 'provides')",
                params![metadata.name, provide],
            )?;
        }

        // Dodaj pliki
        for file in &metadata.files {
            tx.execute(
                "INSERT OR REPLACE INTO files
                 (package_name, file_path, file_size, file_mode, file_uid, file_gid, is_symlink, symlink_target, is_config)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    metadata.name,
                    file.path,
                    file.size,
                    file.mode,
                    file.uid,
                    file.gid,
                    file.is_symlink as i32,
                    file.symlink_target,
                    file.is_config as i32,
                ],
            )?;
        }

        // Dodaj backup plików
        for backup in &metadata.backup {
            tx.execute(
                "INSERT OR REPLACE INTO backups (package_name, file_path) VALUES (?1, ?2)",
                params![metadata.name, backup],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    /// Usuń pakiet z bazy
    pub fn remove_package(&self, name: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM packages WHERE name = ?1", params![name])?;
        Ok(())
    }

    /// Pobierz metadane pakietu
    pub fn get_package(&self, name: &str) -> Result<Option<PackageMetadata>> {
        let conn = self.conn.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT version, release, description, architecture, url,
                    installed_size, archive_size, sha256, blake3, pgp_signer,
                    build_date, install_date, install_reason, repository, verified
             FROM packages WHERE name = ?1",
        )?;

        let result = stmt.query_row(params![name], |row| {
            let install_reason: String = row.get(12)?;
            Ok(PackageMetadata {
                name: name.to_string(),
                version: row.get(0)?,
                release: row.get(1)?,
                description: row.get(2)?,
                architecture: row.get(3)?,
                url: row.get(4)?,
                installed_size: row.get(5)?,
                archive_size: row.get(6)?,
                sha256: row.get(7)?,
                blake3: row.get(8)?,
                pgp_signer: row.get(9)?,
                build_date: row.get::<_, String>(10)?.parse().unwrap_or_else(|_| Utc::now()),
                install_date: row.get::<_, Option<String>>(11)?
                    .and_then(|s| s.parse().ok()),
                install_reason: match install_reason.as_str() {
                    "dependency" => InstallReason::Dependency,
                    _ => InstallReason::Explicit,
                },
                repository: row.get(13)?,
                verified: row.get::<_, i32>(14)? != 0,
                license: vec![],    // wypełnione niżej
                groups: vec![],
                depends: vec![],
                opt_depends: vec![],
                makedepends: vec![],
                checkdepends: vec![],
                conflicts: vec![],
                replaces: vec![],
                provides: vec![],
                files: vec![],
                backup: vec![],
                scripts: std::collections::HashMap::new(),
                extra: std::collections::HashMap::new(),
                pagbuild_source: None,
            })
        });

        match result {
            Ok(mut meta) => {
                // Pobierz powiązane dane
                meta.license = self.get_licenses_internal(&conn, name)?;
                meta.groups = self.get_groups_internal(&conn, name)?;
                meta.depends = self.get_deps_internal(&conn, name, "depends")?;
                meta.opt_depends = self.get_deps_internal(&conn, name, "optdepends")?;
                meta.makedepends = self.get_deps_internal(&conn, name, "makedepends")?;
                meta.conflicts = self.get_deps_internal(&conn, name, "conflicts")?;
                meta.provides = self.get_deps_internal(&conn, name, "provides")?;
                meta.files = self.get_files_internal(&conn, name)?;
                meta.backup = self.get_backups_internal(&conn, name)?;
                Ok(Some(meta))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Listuj wszystkie zainstalowane pakiety
    pub fn list_packages(&self) -> Result<Vec<PackageMetadata>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT name FROM packages ORDER BY name")?;
        let names: Vec<String> = stmt
            .query_map([], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        drop(stmt);
        // Niestety nie możemy trzymać pożyczki conn i wołać get_package,
        // więc pobieramy ręcznie
        let mut packages = Vec::new();
        for name in names {
            if let Some(pkg) = self.get_package_with_conn(&conn, &name)? {
                packages.push(pkg);
            }
        }

        Ok(packages)
    }

    /// Pobierz pakiet z istniejącym połączeniem
    fn get_package_with_conn(&self, conn: &Connection, name: &str) -> Result<Option<PackageMetadata>> {
        let mut stmt = conn.prepare(
            "SELECT version, release, description, architecture, url,
                    installed_size, archive_size, sha256, blake3, pgp_signer,
                    build_date, install_date, install_reason, repository, verified
             FROM packages WHERE name = ?1",
        )?;

        let result = stmt.query_row(params![name], |row| {
            let install_reason: String = row.get(12)?;
            Ok(PackageMetadata {
                name: name.to_string(),
                version: row.get(0)?,
                release: row.get(1)?,
                description: row.get(2)?,
                architecture: row.get(3)?,
                url: row.get(4)?,
                installed_size: row.get(5)?,
                archive_size: row.get(6)?,
                sha256: row.get(7)?,
                blake3: row.get(8)?,
                pgp_signer: row.get(9)?,
                build_date: row.get::<_, String>(10)?.parse().unwrap_or_else(|_| Utc::now()),
                install_date: row.get::<_, Option<String>>(11)?
                    .and_then(|s| s.parse().ok()),
                install_reason: match install_reason.as_str() {
                    "dependency" => InstallReason::Dependency,
                    _ => InstallReason::Explicit,
                },
                repository: row.get(13)?,
                verified: row.get::<_, i32>(14)? != 0,
                license: vec![],
                groups: vec![],
                depends: vec![],
                opt_depends: vec![],
                makedepends: vec![],
                checkdepends: vec![],
                conflicts: vec![],
                replaces: vec![],
                provides: vec![],
                files: vec![],
                backup: vec![],
                scripts: std::collections::HashMap::new(),
                extra: std::collections::HashMap::new(),
                pagbuild_source: None,
            })
        });

        match result {
            Ok(mut meta) => {
                meta.license = self.get_licenses_internal(conn, name)?;
                meta.groups = self.get_groups_internal(conn, name)?;
                meta.depends = self.get_deps_internal(conn, name, "depends")?;
                meta.opt_depends = self.get_deps_internal(conn, name, "optdepends")?;
                meta.makedepends = self.get_deps_internal(conn, name, "makedepends")?;
                meta.conflicts = self.get_deps_internal(conn, name, "conflicts")?;
                meta.provides = self.get_deps_internal(conn, name, "provides")?;
                meta.files = self.get_files_internal(conn, name)?;
                meta.backup = self.get_backups_internal(conn, name)?;
                Ok(Some(meta))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Sprawdź czy pakiet jest installed
    pub fn is_installed(&self, name: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let count: i32 = conn.query_row(
            "SELECT COUNT(*) FROM packages WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    /// Znajdź osierocone pakiety (zależności bez właściciela)
    pub fn find_orphans(&self) -> Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT p.name FROM packages p
             WHERE p.install_reason = 'dependency'
             AND p.name NOT IN (
                 SELECT DISTINCT d.dependency FROM depends d
                 WHERE d.dep_type = 'depends'
                 AND d.dependency IN (SELECT name FROM packages)
             )",
        )?;

        let orphans = stmt
            .query_map([], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(orphans)
    }

    /// Znajdź pakiety które zależą od danego
    pub fn find_dependents(&self, name: &str) -> Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT DISTINCT package_name FROM depends
             WHERE dependency LIKE ?1 AND dep_type = 'depends'",
        )?;

        let dependents = stmt
            .query_map(params![format!("{}%", name)], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(dependents)
    }

    // ── Metody pomocnicze wewnętrzne ──

    fn get_licenses_internal(&self, conn: &Connection, name: &str) -> Result<Vec<String>> {
        let mut stmt = conn.prepare("SELECT license FROM licenses WHERE package_name = ?1")?;
        let licenses = stmt
            .query_map(params![name], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(licenses)
    }

    fn get_groups_internal(&self, conn: &Connection, name: &str) -> Result<Vec<String>> {
        let mut stmt = conn.prepare("SELECT group_name FROM groups_pkg WHERE package_name = ?1")?;
        let groups = stmt
            .query_map(params![name], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(groups)
    }

    fn get_deps_internal(&self, conn: &Connection, name: &str, dep_type: &str) -> Result<Vec<String>> {
        let mut stmt = conn.prepare(
            "SELECT dependency FROM depends WHERE package_name = ?1 AND dep_type = ?2",
        )?;
        let deps = stmt
            .query_map(params![name, dep_type], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(deps)
    }

    fn get_files_internal(&self, conn: &Connection, name: &str) -> Result<Vec<PackageFile>> {
        let mut stmt = conn.prepare(
            "SELECT file_path, file_size, file_mode, file_uid, file_gid,
                    is_symlink, symlink_target, is_config
             FROM files WHERE package_name = ?1",
        )?;

        let files = stmt
            .query_map(params![name], |row| {
                Ok(PackageFile {
                    path: row.get(0)?,
                    size: row.get::<_, i64>(1)? as u64,
                    mode: row.get::<_, i64>(2)? as u32,
                    uid: row.get::<_, i64>(3)? as u32,
                    gid: row.get::<_, i64>(4)? as u32,
                    md5: None,
                    is_symlink: row.get::<_, i32>(5)? != 0,
                    symlink_target: row.get(6)?,
                    is_config: row.get::<_, i32>(7)? != 0,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(files)
    }

    fn get_backups_internal(&self, conn: &Connection, name: &str) -> Result<Vec<String>> {
        let mut stmt = conn.prepare("SELECT file_path FROM backups WHERE package_name = ?1")?;
        let backups = stmt
            .query_map(params![name], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(backups)
    }
}
