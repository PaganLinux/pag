use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::json;

use crate::db;
use crate::models::*;
use crate::state::AppState;

pub async fn forgejo_webhook(
    State(state): State<AppState>,
    Json(payload): Json<ForgejoWebhookPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!("Forgejo webhook: action={:?}", payload.action);

    if payload.action.as_deref() != Some("opened") {
        return Ok(Json(json!({"message": "Ignored"})));
    }

    let pr = match &payload.pull_request {
        Some(pr) => pr,
        None => return Ok(Json(json!({"message": "No PR data"}))),
    };

    let title = pr.title.as_deref().unwrap_or("Unknown");
    let body = pr.body.as_deref().unwrap_or("");
    let submitter = pr.user.as_ref().map(|u| u.login.as_str()).unwrap_or("unknown");
    let pr_url = pr.html_url.clone().unwrap_or_default();

    // Parse title: "pkgname: version"
    let (pkg_name, pkg_version, description) = parse_pr_title(title, body);
    let build_script = extract_build_script(body);

    let submission = PackageSubmission {
        id: 0,
        forgejo_pr_id: pr.id,
        forgejo_pr_url: pr_url,
        package_name: pkg_name,
        package_version: pkg_version,
        description,
        submitter: submitter.to_string(),
        build_script,
        status: SubmissionStatus::Pending,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let id = db::create_submission(&state.pool, &submission).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    tracing::info!("Created submission #{} from PR #{}", id, pr.id);

    Ok(Json(json!({"message": "Submission created", "submission_id": id})))
}

fn parse_pr_title(title: &str, body: &str) -> (String, String, String) {
    if let Some((name, rest)) = title.split_once(':') {
        return (name.trim().to_string(), rest.trim().to_string(), first_line(body));
    }
    if title.starts_with("Add ") || title.starts_with("add ") {
        let rest = &title[4..];
        let parts: Vec<&str> = rest.split_whitespace().collect();
        if parts.len() >= 2 {
            return (parts[0].to_string(), parts[1].to_string(), first_line(body));
        }
        return (rest.to_string(), "unknown".into(), first_line(body));
    }
    (title.to_string(), "unknown".into(), first_line(body))
}

fn first_line(s: &str) -> String {
    s.lines().next().unwrap_or("").to_string()
}

fn extract_build_script(body: &str) -> String {
    if let Some(start) = body.find("```") {
        let rest = &body[start + 3..];
        if let Some(end) = rest.find("```") {
            let content = rest[..end].trim();
            if content.contains("name=") || content.contains("source=") {
                return content.to_string();
            }
        }
    }
    body.to_string()
}
