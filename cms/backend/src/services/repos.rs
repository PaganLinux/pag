// Serwis repozytoriów — integracja z Gitea API
use crate::config::Config;
use crate::db::DbPool;
use crate::models::*;
use serde_json::Value;

pub struct RepoService;

impl RepoService {
    pub async fn list(pool: &DbPool) -> Result<Vec<Repo>, anyhow::Error> {
        let repos = sqlx::query_as::<_, Repo>(
            "SELECT * FROM repos WHERE active = 1 ORDER BY name ASC",
        )
        .fetch_all(pool)
        .await?;
        Ok(repos)
    }

    pub async fn get_by_id(pool: &DbPool, id: i64) -> Result<Option<Repo>, anyhow::Error> {
        let repo = sqlx::query_as::<_, Repo>("SELECT * FROM repos WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(repo)
    }

    /// Utwórz repozytorium w Gitea i zapisz lokalnie
    pub async fn create(
        pool: &DbPool,
        req: CreateRepoRequest,
        config: &Config,
    ) -> Result<Repo, anyhow::Error> {
        let owner = req.owner.as_deref().unwrap_or("PaganLinux");
        let full_name = format!("{}/{}", owner, req.name);

        // Wywołaj API Gitea
        let gitea_repo = Self::create_gitea_repo(config, &req.name, req.description.as_deref(), owner)
            .await?;

        let gitea_id = gitea_repo["id"].as_i64();
        let clone_url = gitea_repo["clone_url"].as_str().map(String::from);

        let repo = sqlx::query_as::<_, Repo>(
            r#"INSERT INTO repos (name, full_name, owner, description, gitea_id, clone_url, active)
               VALUES (?, ?, ?, ?, ?, ?, 1) RETURNING *"#,
        )
        .bind(&req.name)
        .bind(&full_name)
        .bind(owner)
        .bind(req.description.as_deref().unwrap_or(""))
        .bind(gitea_id)
        .bind(clone_url)
        .fetch_one(pool)
        .await?;

        Ok(repo)
    }

    /// Obsługa webhooka z Gitea (push → trigger build)
    pub async fn handle_webhook(
        pool: &DbPool,
        payload: &WebhookPayload,
    ) -> Result<(), anyhow::Error> {
        let repo_name = payload
            .repository
            .as_ref()
            .and_then(|r| r.full_name.as_deref())
            .unwrap_or("unknown");

        let branch = payload
            .r#ref
            .as_deref()
            .unwrap_or("")
            .replace("refs/heads/", "");

        tracing::info!("Webhook received: {} branch: {}", repo_name, branch);

        // Tylko push do main triggeruje build
        if branch != "main" {
            tracing::info!("Ignoring push to branch: {}", branch);
            return Ok(());
        }

        // Znajdź repo w bazie
        let repo = sqlx::query_as::<_, Repo>(
            "SELECT * FROM repos WHERE full_name = ? AND active = 1",
        )
        .bind(repo_name)
        .fetch_optional(pool)
        .await?;

        if let Some(repo) = repo {
            tracing::info!("Triggering build for repo: {}", repo.name);
            // TODO: Trigger build dla powiązanych pakietów
        }

        Ok(())
    }

    async fn create_gitea_repo(
        config: &Config,
        name: &str,
        description: Option<&str>,
        owner: &str,
    ) -> Result<Value, anyhow::Error> {
        let client = reqwest::Client::new();
        let url = if owner == "PaganLinux" {
            format!("{}/orgs/{}/repos", config.gitea_api_url, owner)
        } else {
            format!("{}/user/repos", config.gitea_api_url)
        };

        let mut body = serde_json::json!({
            "name": name,
            "private": false,
            "auto_init": false,
        });

        if let Some(desc) = description {
            body["description"] = serde_json::Value::String(desc.to_string());
        }

        let res = client
            .post(&url)
            .header("Authorization", format!("token {}", config.gitea_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await?;
            anyhow::bail!("Gitea API error: {} - {}", status, text);
        }

        Ok(res.json().await?)
    }
}
