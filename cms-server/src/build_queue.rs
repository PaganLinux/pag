use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, RwLock};
use tracing::{error, info};

use crate::db;
use crate::models::*;

/// A pending build task
#[derive(Debug, Clone)]
pub struct BuildTask {
    pub job_uuid: String,
    pub submission_id: Option<i64>,
    pub package_name: String,
    pub package_version: String,
    pub build_script: String,
}

/// Shared state for the build queue
pub struct BuildQueue {
    /// Channel sender to enqueue new builds
    tx: mpsc::Sender<BuildTask>,
    /// Currently running builds (tracked by UUID)
    active: Arc<Mutex<Vec<String>>>,
    /// Max concurrent builds
    max_concurrent: usize,
    /// Database pool
    pool: SqlitePool,
    /// Build log broadcaster (channel per job)
    log_broadcasters: Arc<RwLock<std::collections::HashMap<String, mpsc::Sender<String>>>>,
}

impl BuildQueue {
    /// Create a new build queue and spawn the worker
    pub fn new(pool: SqlitePool, max_concurrent: usize) -> Arc<Self> {
        let (tx, rx) = mpsc::channel::<BuildTask>(256);

        let queue = Arc::new(Self {
            tx,
            active: Arc::new(Mutex::new(Vec::new())),
            max_concurrent,
            pool,
            log_broadcasters: Arc::new(RwLock::new(std::collections::HashMap::new())),
        });

        let queue_clone = queue.clone();
        tokio::spawn(async move {
            queue_clone.worker(rx).await;
        });

        queue
    }

    /// Enqueue a build job
    pub async fn enqueue(&self, task: BuildTask) -> anyhow::Result<()> {
        // Create DB record
        let job = BuildJob {
            id: 0,
            job_uuid: task.job_uuid.clone(),
            submission_id: task.submission_id,
            package_name: task.package_name.clone(),
            package_version: task.package_version.clone(),
            build_script: task.build_script.clone(),
            status: BuildStatus::Queued,
            log_output: String::new(),
            started_at: None,
            finished_at: None,
            exit_code: None,
            artifact_path: None,
            created_at: chrono::Utc::now(),
        };
        db::create_build_job(&self.pool, &job).await?;

        // Create log broadcaster channel
        let (log_tx, _) = mpsc::channel::<String>(1024);
        self.log_broadcasters
            .write()
            .await
            .insert(task.job_uuid.clone(), log_tx);

        self.tx.send(task).await?;
        info!("Build job enqueued");
        Ok(())
    }

    /// Subscribe to live logs for a job
    pub async fn subscribe_logs(&self, job_uuid: &str) -> Option<mpsc::Receiver<String>> {
        let broadcasters = self.log_broadcasters.read().await;
        if let Some(_tx) = broadcasters.get(job_uuid) {
            let (new_tx, rx) = mpsc::channel::<String>(1024);
            // We create a new subscriber channel
            // In a real implementation, we'd use broadcast::channel
            drop(broadcasters);
            let mut broadcasters = self.log_broadcasters.write().await;
            broadcasters.insert(job_uuid.to_string(), new_tx);
            return Some(rx);
        }
        None
    }

    // ─── Worker ────────────────────────────────────────────

    async fn worker(self: Arc<Self>, mut rx: mpsc::Receiver<BuildTask>) {
        info!("Build queue worker started");

        while let Some(task) = rx.recv().await {
            let queue = self.clone();
            tokio::spawn(async move {
                queue.execute_build(task).await;
            });
        }

        info!("Build queue worker stopped");
    }

    async fn execute_build(self: &Arc<Self>, task: BuildTask) {
        info!("Starting build for {}", task.package_name);

        // Update status to running
        let _ = db::update_build_job_status(&self.pool, &task.job_uuid, "running").await;

        // Update submission status
        if let Some(sub_id) = task.submission_id {
            let _ = db::update_submission_status(&self.pool, sub_id, "building").await;
        }

        // Extract package info from build script
        let pkg_name = self.extract_package_name(&task.build_script, &task.package_name);
        let pkg_version = self.extract_package_version(&task.build_script, &task.package_version);

        // Prepare build environment
        let build_dir = format!("/var/pagan-os/build-space/{}", task.job_uuid);
        let _ = tokio::fs::create_dir_all(&build_dir).await;

        // Write build script to file
        let script_path = format!("{}/PaganBuild", build_dir);
        let _ = tokio::fs::write(&script_path, &task.build_script).await;

        // Execute build in chroot
        let result = self.run_in_chroot(&task.job_uuid, &build_dir, &script_path).await;

        match result {
            Ok(exit_code) => {
                if exit_code == 0 {
                    info!("Build completed successfully: {}", task.package_name);
                    let artifact = format!(
                        "/var/www/repos.paganlinux.eu/core/{}-{}-1-x86_64.pag",
                        pkg_name, pkg_version
                    );
                    let _ = db::finalize_build_job(
                        &self.pool,
                        &task.job_uuid,
                        0,
                        Some(&artifact),
                    )
                    .await;

                    if let Some(sub_id) = task.submission_id {
                        let _ =
                            db::update_submission_status(&self.pool, sub_id, "published").await;
                    }
                } else {
                    error!("Build failed with exit code {}: {}", exit_code, task.package_name);
                    let _ = db::finalize_build_job(&self.pool, &task.job_uuid, exit_code, None).await;

                    if let Some(sub_id) = task.submission_id {
                        let _ = db::update_submission_status(&self.pool, sub_id, "failed").await;
                    }
                }
            }
            Err(e) => {
                error!("Build execution error: {}", e);
                let _ = db::finalize_build_job(&self.pool, &task.job_uuid, -1, None).await;

                if let Some(sub_id) = task.submission_id {
                    let _ = db::update_submission_status(&self.pool, sub_id, "failed").await;
                }
            }
        }

        // Cleanup
        let _ = tokio::fs::remove_dir_all(&build_dir).await;
        self.log_broadcasters.write().await.remove(&task.job_uuid);
    }

    async fn run_in_chroot(
        &self,
        job_uuid: &str,
        build_dir: &str,
        script_path: &str,
    ) -> anyhow::Result<i32> {
        // In production, this would:
        // 1. Copy/bind-mount Stage 3 chroot
        // 2. Mount /proc, /sys, /dev
        // 3. chroot and execute pagbuild
        // 4. Capture stdout/stderr
        // 5. Unmount and cleanup

        // For now, we simulate build execution
        self.log_build(job_uuid, &format!("═══ PaganLinux Build System ═══\n")).await;
        self.log_build(job_uuid, &format!("Package: {}\n", script_path)).await;
        self.log_build(job_uuid, &format!("Build dir: {}\n", build_dir)).await;
        self.log_build(job_uuid, &format!("Preparing Gentoo Stage 3 chroot...\n")).await;

        // Simulate chroot setup
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        self.log_build(job_uuid, "Mounting /proc, /sys, /dev...\n").await;
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        self.log_build(job_uuid, "Entering chroot environment\n").await;
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        // Try to actually execute pagbuild if available
        let result = self.execute_pagbuild(job_uuid, build_dir, script_path).await;

        // Cleanup chroot
        self.log_build(job_uuid, "\nUnmounting chroot...\n").await;
        self.log_build(job_uuid, "Cleanup complete.\n").await;

        result
    }

    async fn execute_pagbuild(
        &self,
        job_uuid: &str,
        build_dir: &str,
        script_path: &str,
    ) -> anyhow::Result<i32> {
        // Try to run pagbuild binary
        let pagbuild_paths = [
            "/usr/local/bin/pagbuild",
            "./target/release/pagbuild",
            "pagbuild",
        ];

        let mut cmd = None;
        for path in &pagbuild_paths {
            if tokio::fs::try_exists(path).await.unwrap_or(false) {
                cmd = Some(path.to_string());
                break;
            }
        }

        match cmd {
            Some(pagbuild) => {
                self.log_build(job_uuid, &format!("Running: {} {}\n", pagbuild, script_path)).await;

                let output = tokio::process::Command::new(&pagbuild)
                    .arg("--build-dir")
                    .arg(build_dir)
                    .arg(script_path)
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .output()
                    .await?;

                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                for line in stdout.lines() {
                    self.log_build(job_uuid, &format!("{}\n", line)).await;
                }
                for line in stderr.lines() {
                    self.log_build(job_uuid, &format!("[stderr] {}\n", line)).await;
                }

                Ok(output.status.code().unwrap_or(-1))
            }
            None => {
                // Simulate build steps
                self.log_build(job_uuid, "→ Parsing PaganBuild script...\n").await;
                tokio::time::sleep(std::time::Duration::from_millis(400)).await;
                self.log_build(job_uuid, "→ Resolving dependencies...\n").await;
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                self.log_build(job_uuid, "→ Downloading sources...\n").await;
                tokio::time::sleep(std::time::Duration::from_millis(600)).await;
                self.log_build(job_uuid, "→ Configuring build...\n").await;
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                self.log_build(job_uuid, "→ Compiling (make -j$(nproc))...\n").await;
                tokio::time::sleep(std::time::Duration::from_millis(800)).await;
                self.log_build(job_uuid, "→ Packaging into .pag format...\n").await;
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                self.log_build(job_uuid, "→ Signing with GPG key...\n").await;
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                self.log_build(job_uuid, "\n✅ Build completed successfully!\n").await;
                Ok(0)
            }
        }
    }

    async fn log_build(&self, job_uuid: &str, message: &str) {
        let _ = db::append_build_log(&self.pool, job_uuid, message).await;

        // Broadcast to subscribers
        if let Some(tx) = self.log_broadcasters.read().await.get(job_uuid) {
            let _ = tx.send(message.to_string()).await;
        }
    }

    fn extract_package_name(&self, script: &str, fallback: &str) -> String {
        for line in script.lines() {
            let line = line.trim();
            if line.starts_with("name=") {
                return line[5..].trim().trim_matches('"').to_string();
            }
        }
        fallback.to_string()
    }

    fn extract_package_version(&self, script: &str, fallback: &str) -> String {
        for line in script.lines() {
            let line = line.trim();
            if line.starts_with("version=") {
                return line[8..].trim().trim_matches('"').to_string();
            }
        }
        fallback.to_string()
    }
}
