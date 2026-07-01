// Moduł bazy danych - SQLite
//
// Schemat bazy:
// - packages: zainstalowane pakiety
// - files: pliki należące do pakietów
// - deps: zależności między pakietami
// - repos: informacje o repozytoriach
// - history: historia operacji

use rusqlite::{params, Connection, Transaction};
use std::path::Path;
use std::sync::Mutex;

use crate::package::{FileEntry, FileType, PackageHeader};

/// Menedżer bazy danych pakietów
pub struct PackageDb {
    conn: Mutex<Connection>,
}

/// Informacje o zainstalowanym pakiecie
#[derive(Debug, Clone)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub release: u32,
    pub description: String,
    pub arch: String,
    pub license: String,
    pub installed_size: u64,
    pub install_date: i64,
    pub install_reason: InstallReason,
    pub group_name: Option<String>,
    pub url: Option<String>,
    pub maintainer: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InstallReason {
    /// Jawnie zainstalowany przez użytkownika
    Explicit = 0,
    /// Zainstalowany jako zależność
    Dependency = 1,
}

impl PackageDb {
    /// Otwiera (lub tworzy) bazę danych
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        // Upewnij się, że katalog istnieje
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path)?;

        // Włącz WAL mode dla lepszej wydajności
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

        let db = Self { conn: Mutex::new(conn) };
        db.init_schema()?;
        Ok(db)
    }

    /// Tworzy schemat bazy danych
    fn init_schema(&self) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS packages (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                name        TEXT NOT NULL,
                version     TEXT NOT NULL,
                release     INTEGER NOT NULL DEFAULT 1,
                description TEXT DEFAULT '',
                arch        TEXT NOT NULL,
                license     TEXT DEFAULT 'custom',
                installed_size INTEGER DEFAULT 0,
                install_date   INTEGER NOT NULL,
                install_reason INTEGER NOT NULL DEFAULT 0,
                group_name     TEXT,
                url            TEXT,
                maintainer     TEXT,
                sha512         TEXT,
                blake3         TEXT,
                UNIQUE(name)
            );

            CREATE TABLE IF NOT EXISTS files (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                package_id  INTEGER NOT NULL,
                path        TEXT NOT NULL,
                size        INTEGER DEFAULT 0,
                mode        INTEGER DEFAULT 0,
                uid         INTEGER DEFAULT 0,
                gid         INTEGER DEFAULT 0,
                file_type   TEXT NOT NULL DEFAULT 'file',
                link_target TEXT,
                sha256      TEXT,
                FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE,
                UNIQUE(path)
            );

            CREATE INDEX IF NOT EXISTS idx_files_path ON files(path);
            CREATE INDEX IF NOT EXISTS idx_files_package ON files(package_id);

            CREATE TABLE IF NOT EXISTS depends (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                package_id  INTEGER NOT NULL,
                dependency  TEXT NOT NULL,
                dep_type    TEXT NOT NULL DEFAULT 'depends',
                dep_version TEXT,
                FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS provides (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                package_id  INTEGER NOT NULL,
                provide     TEXT NOT NULL,
                FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS conflicts (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                package_id  INTEGER NOT NULL,
                conflict    TEXT NOT NULL,
                FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS repos (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                name        TEXT NOT NULL UNIQUE,
                url         TEXT NOT NULL,
                mirror      TEXT,
                enabled     INTEGER NOT NULL DEFAULT 1,
                priority    INTEGER NOT NULL DEFAULT 0,
                arch        TEXT NOT NULL,
                branch      TEXT NOT NULL DEFAULT 'main',
                last_update INTEGER DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS repo_packages (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                repo_name   TEXT NOT NULL,
                pkg_name    TEXT NOT NULL,
                version     TEXT NOT NULL,
                release     INTEGER NOT NULL DEFAULT 1,
                arch        TEXT NOT NULL,
                description TEXT DEFAULT '',
                installed_size INTEGER DEFAULT 0,
                compressed_size INTEGER DEFAULT 0,
                filename    TEXT NOT NULL,
                sha512      TEXT,
                blake3      TEXT,
                pgp_sig     TEXT,
                UNIQUE(repo_name, pkg_name)
            );

            CREATE INDEX IF NOT EXISTS idx_repo_pkg_name ON repo_packages(pkg_name);

            CREATE TABLE IF NOT EXISTS history (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp   INTEGER NOT NULL,
                operation   TEXT NOT NULL,
                package     TEXT,
                version     TEXT,
                details     TEXT
            );

            CREATE TABLE IF NOT EXISTS config (
                key   TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            "
        )?;

        // Ustaw wersję schematu
        conn.execute(
            "INSERT OR IGNORE INTO config (key, value) VALUES ('schema_version', '3')",
            [],
        )?;

        Ok(())
    }

    // ─── Operacje na pakietach ───────────────────────────────────

    /// Dodaje zainstalowany pakiet do bazy
    pub fn add_package(&self, header: &PackageHeader, reason: InstallReason) -> anyhow::Result<i64> {
        let conn = self.conn.lock().unwrap();
        let tx = conn.unchecked_transaction()?;

        // Dodaj pakiet
        tx.execute(
            "INSERT INTO packages (name, version, release, description, arch, license,
             installed_size, install_date, install_reason, group_name, url, maintainer, sha512, blake3)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
             ON CONFLICT(name) DO UPDATE SET
                version = ?2, release = ?3, description = ?4,
                installed_size = ?7, install_date = ?8, sha512 = ?13, blake3 = ?14",
            params![
                header.name,
                header.version,
                header.release,
                header.description,
                header.arch,
                header.license,
                header.installed_size,
                chrono::Utc::now().timestamp(),
                reason as u8,
                header.group,
                header.url,
                header.maintainer,
                header.sha512,
                header.blake3,
            ],
        )?;

        let pkg_id = tx.last_insert_rowid();

        // Dodaj pliki
        let mut file_stmt = tx.prepare(
            "INSERT OR REPLACE INTO files (package_id, path, size, mode, uid, gid, file_type, link_target, sha256)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"
        )?;

        for file in &header.files {
            file_stmt.execute(params![
                pkg_id,
                file.path,
                file.size,
                file.mode,
                file.uid,
                file.gid,
                match file.file_type {
                    FileType::File => "file",
                    FileType::Dir => "dir",
                    FileType::Symlink => "symlink",
                    FileType::Hardlink => "hardlink",
                },
                file.link_target,
                file.sha256,
            ])?;
        }
        // Drop statement before commit
        drop(file_stmt);

        // Dodaj zależności
        for dep in &header.depends {
            tx.execute(
                "INSERT INTO depends (package_id, dependency, dep_type) VALUES (?1, ?2, 'depends')",
                params![pkg_id, dep],
            )?;
        }

        // Dodaj provides
        for provide in &header.provides {
            tx.execute(
                "INSERT INTO provides (package_id, provide) VALUES (?1, ?2)",
                params![pkg_id, provide],
            )?;
        }

        // Dodaj konflikty
        for conflict in &header.conflicts {
            tx.execute(
                "INSERT INTO conflicts (package_id, conflict) VALUES (?1, ?2)",
                params![pkg_id, conflict],
            )?;
        }

        tx.commit()?;
        Ok(pkg_id)
    }

    /// Usuwa pakiet z bazy
    pub fn remove_package(&self, name: &str) -> anyhow::Result<bool> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute("DELETE FROM packages WHERE name = ?1", params![name])?;
        Ok(rows > 0)
    }

    /// Sprawdza czy pakiet jest zainstalowany
    pub fn is_installed(&self, name: &str) -> anyhow::Result<bool> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM packages WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    /// Pobiera zainstalowany pakiet
    pub fn get_package(&self, name: &str) -> anyhow::Result<Option<InstalledPackage>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT name, version, release, description, arch, license, installed_size,
                    install_date, install_reason, group_name, url, maintainer
             FROM packages WHERE name = ?1"
        )?;

        let result = stmt.query_row(params![name], |row| {
            Ok(InstalledPackage {
                name: row.get(0)?,
                version: row.get(1)?,
                release: row.get(2)?,
                description: row.get(3)?,
                arch: row.get(4)?,
                license: row.get(5)?,
                installed_size: row.get(6)?,
                install_date: row.get(7)?,
                install_reason: match row.get::<_, u8>(8)? {
                    0 => InstallReason::Explicit,
                    _ => InstallReason::Dependency,
                },
                group_name: row.get(9)?,
                url: row.get(10)?,
                maintainer: row.get(11)?,
            })
        });

        match result {
            Ok(pkg) => Ok(Some(pkg)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Lista wszystkich zainstalowanych pakietów
    pub fn list_packages(&self, reason: Option<InstallReason>) -> anyhow::Result<Vec<InstalledPackage>> {
        let conn = self.conn.lock().unwrap();
        let query = if let Some(r) = reason {
            format!(
                "SELECT name, version, release, description, arch, license, installed_size,
                        install_date, install_reason, group_name, url, maintainer
                 FROM packages WHERE install_reason = {} ORDER BY name",
                r as u8
            )
        } else {
            "SELECT name, version, release, description, arch, license, installed_size,
                    install_date, install_reason, group_name, url, maintainer
             FROM packages ORDER BY name".to_string()
        };

        let mut stmt = conn.prepare(&query)?;
        let packages = stmt.query_map([], |row| {
            Ok(InstalledPackage {
                name: row.get(0)?,
                version: row.get(1)?,
                release: row.get(2)?,
                description: row.get(3)?,
                arch: row.get(4)?,
                license: row.get(5)?,
                installed_size: row.get(6)?,
                install_date: row.get(7)?,
                install_reason: match row.get::<_, u8>(8)? {
                    0 => InstallReason::Explicit,
                    _ => InstallReason::Dependency,
                },
                group_name: row.get(9)?,
                url: row.get(10)?,
                maintainer: row.get(11)?,
            })
        })?.filter_map(|r| r.ok()).collect();

        Ok(packages)
    }

    /// Znajduje sieroty (pakiety zależności, które nie są już potrzebne)
    pub fn find_orphans(&self) -> anyhow::Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT p.name FROM packages p
             WHERE p.install_reason = 1  -- Dependency
             AND p.name NOT IN (
                 SELECT DISTINCT d.dependency FROM depends d
                 INNER JOIN packages p2 ON d.package_id = p2.id
             )
             ORDER BY p.name"
        )?;

        let orphans = stmt.query_map([], |row| row.get::<_, String>(0))?
            .filter_map(|r| r.ok()).collect();

        Ok(orphans)
    }

    /// Zwraca pliki należące do pakietu
    pub fn get_package_files(&self, name: &str) -> anyhow::Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT f.path FROM files f
             INNER JOIN packages p ON f.package_id = p.id
             WHERE p.name = ?1 ORDER BY f.path"
        )?;

        let files = stmt.query_map(params![name], |row| row.get::<_, String>(0))?
            .filter_map(|r| r.ok()).collect();

        Ok(files)
    }

    /// Znajduje właściciela pliku
    pub fn find_file_owner(&self, path: &str) -> anyhow::Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let result = conn.query_row(
            "SELECT p.name FROM files f
             INNER JOIN packages p ON f.package_id = p.id
             WHERE f.path = ?1",
            params![path],
            |row| row.get(0),
        );

        match result {
            Ok(name) => Ok(Some(name)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    // ─── Operacje na repo ─────────────────────────────────────

    /// Aktualizuje indeks repozytorium
    pub fn update_repo_packages(&self, repo_name: &str, packages: &[crate::package::RepoPackageEntry]) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        let tx = conn.unchecked_transaction()?;

        // Usuń stare wpisy
        tx.execute("DELETE FROM repo_packages WHERE repo_name = ?1", params![repo_name])?;

        // Dodaj nowe
        let mut stmt = tx.prepare(
            "INSERT INTO repo_packages (repo_name, pkg_name, version, release, arch, description,
             installed_size, compressed_size, filename, sha512, blake3, pgp_sig)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)"
        )?;

        for pkg in packages {
            stmt.execute(params![
                repo_name,
                pkg.name,
                pkg.version,
                pkg.release,
                pkg.arch,
                pkg.description,
                pkg.installed_size,
                pkg.compressed_size,
                pkg.filename,
                pkg.sha512,
                pkg.blake3,
                pkg.pgp_signature,
            ])?;
        }
        // Drop statement before commit
        drop(stmt);

        // Aktualizuj timestamp
        tx.execute(
            "UPDATE repos SET last_update = ?1 WHERE name = ?2",
            params![chrono::Utc::now().timestamp(), repo_name],
        )?;

        tx.commit()?;
        Ok(())
    }

    /// Szuka pakietów w repozytoriach
    pub fn search_repo_packages(&self, query: &str, search_desc: bool) -> anyhow::Result<Vec<(String, String, String, String)>> {
        let conn = self.conn.lock().unwrap();
        let like = format!("%{}%", query);

        let sql = if search_desc {
            "SELECT rp.repo_name, rp.pkg_name, rp.version, rp.description
             FROM repo_packages rp
             WHERE rp.pkg_name LIKE ?1 OR rp.description LIKE ?1
             ORDER BY rp.pkg_name LIMIT 100"
        } else {
            "SELECT rp.repo_name, rp.pkg_name, rp.version, rp.description
             FROM repo_packages rp
             WHERE rp.pkg_name LIKE ?1
             ORDER BY rp.pkg_name LIMIT 100"
        };

        let mut stmt = conn.prepare(sql)?;
        let results = stmt.query_map(params![like], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?.filter_map(|r| r.ok()).collect();

        Ok(results)
    }

    /// Pobiera dane pakietu z repozytorium
    pub fn get_repo_package(&self, name: &str) -> anyhow::Result<Option<(String, String)>> {
        let conn = self.conn.lock().unwrap();
        let result = conn.query_row(
            "SELECT repo_name, filename FROM repo_packages WHERE pkg_name = ?1 ORDER BY repo_name LIMIT 1",
            params![name],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        );

        match result {
            Ok(data) => Ok(Some(data)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    // ─── Historia ─────────────────────────────────────────────

    /// Dodaje wpis do historii
    pub fn add_history(&self, operation: &str, package: &str, version: &str, details: &str) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO history (timestamp, operation, package, version, details) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![chrono::Utc::now().timestamp(), operation, package, version, details],
        )?;
        Ok(())
    }

    /// Pobiera ostatnie N wpisów historii
    pub fn get_history(&self, limit: usize) -> anyhow::Result<Vec<(i64, String, String, String)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT timestamp, operation, package, details FROM history ORDER BY id DESC LIMIT ?1"
        )?;

        let history = stmt.query_map(params![limit as i64], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?.filter_map(|r| r.ok()).collect();

        Ok(history)
    }

    /// Zwraca statystyki bazy
    pub fn stats(&self) -> anyhow::Result<DbStats> {
        let conn = self.conn.lock().unwrap();

        let total_packages: i64 = conn.query_row(
            "SELECT COUNT(*) FROM packages", [], |row| row.get(0),
        )?;

        let explicit_packages: i64 = conn.query_row(
            "SELECT COUNT(*) FROM packages WHERE install_reason = 0", [], |row| row.get(0),
        )?;

        let dep_packages: i64 = conn.query_row(
            "SELECT COUNT(*) FROM packages WHERE install_reason = 1", [], |row| row.get(0),
        )?;

        let total_files: i64 = conn.query_row(
            "SELECT COUNT(*) FROM files", [], |row| row.get(0),
        )?;

        let total_size: i64 = conn.query_row(
            "SELECT COALESCE(SUM(installed_size), 0) FROM packages", [], |row| row.get(0),
        )?;

        let orphans: i64 = conn.query_row(
            "SELECT COUNT(*) FROM packages p
             WHERE p.install_reason = 1
             AND p.name NOT IN (SELECT DISTINCT d.dependency FROM depends d
                                INNER JOIN packages p2 ON d.package_id = p2.id)",
            [], |row| row.get(0),
        )?;

        Ok(DbStats {
            total_packages: total_packages as u64,
            explicit_packages: explicit_packages as u64,
            dep_packages: dep_packages as u64,
            total_files: total_files as u64,
            total_size_bytes: total_size as u64,
            orphans: orphans as u64,
        })
    }
}

#[derive(Debug)]
pub struct DbStats {
    pub total_packages: u64,
    pub explicit_packages: u64,
    pub dep_packages: u64,
    pub total_files: u64,
    pub total_size_bytes: u64,
    pub orphans: u64,
}

impl DbStats {
    pub fn total_size_human(&self) -> String {
        human_size(self.total_size_bytes)
    }
}

fn human_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_idx])
}
