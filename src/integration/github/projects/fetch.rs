//! Fetches per-repository star and fork counts from the GitHub REST API.

use chrono::Utc;
use futures::future;

use crate::integration::github::projects::model::ProjectsStats;
use crate::integration::github::projects::model::RepoStats;

/// Errors that can occur while fetching projects statistics.
#[derive(Debug)]
pub enum FetchError {
    /// An HTTP-level error.
    Http(String),
    /// A JSON parsing or field extraction error.
    Parse(String),
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::Http(e) => write!(f, "HTTP error: {e}"),
            FetchError::Parse(e) => write!(f, "Parse error: {e}"),
        }
    }
}

impl std::error::Error for FetchError {}

/// Fetches stars and forks for a single repository slug.
///
/// # Arguments
///
/// * `client` - A shared HTTP client.
/// * `slug` - Repository slug, e.g. `"JP-Ellis/tikz-feynman"`.
/// * `token` - A GitHub personal access token.
///
/// # Returns
///
/// [`RepoStats`] on success.
///
/// # Errors
///
/// Returns [`FetchError`] if the request fails or the response cannot be parsed.
async fn fetch_single_repo(
    client: &reqwest::Client,
    slug: &str,
    token: &str,
) -> Result<RepoStats, FetchError> {
    let url = format!("https://api.github.com/repos/{slug}");
    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {token}"))
        .header("User-Agent", "jpellis-me/1.0")
        .send()
        .await
        .map_err(|e| FetchError::Http(e.to_string()))?;

    if !resp.status().is_success() {
        return Err(FetchError::Http(format!(
            "GET /repos/{slug} returned {}",
            resp.status()
        )));
    }

    let body = resp
        .json::<serde_json::Value>()
        .await
        .map_err(|e| FetchError::Parse(e.to_string()))?;

    let (stars, forks) = parse_repo_response(slug, &body)?;

    Ok(RepoStats {
        slug: slug.to_string(),
        stars,
        forks,
    })
}

/// Fetches stars and forks for the given repository slugs concurrently.
///
/// Repos that fail to fetch are logged and skipped — a single repo being
/// renamed, archived, or rate-limited does not fail the entire fetch.
///
/// # Arguments
///
/// * `token` - A GitHub personal access token with at least `public_repo` read scope.
/// * `slugs` - Repository slugs to fetch, sourced from [`crate::config::projects::projects_config`].
///
/// # Returns
///
/// A [`ProjectsStats`] with one [`RepoStats`] per successfully fetched repo.
pub async fn fetch_projects_stats(token: &str, slugs: &[String]) -> ProjectsStats {
    let client = reqwest::Client::new();
    let futs: Vec<_> = slugs
        .iter()
        .map(|slug| fetch_single_repo(&client, slug.as_str(), token))
        .collect();

    let repos: Vec<RepoStats> = future::join_all(futs)
        .await
        .into_iter()
        .filter_map(|result| match result {
            Ok(stats) => Some(stats),
            Err(e) => {
                leptos::logging::warn!("projects::fetch: {e}");
                None
            }
        })
        .collect();

    ProjectsStats {
        fetched_at: Utc::now(),
        repos,
    }
}

/// Parses `stargazers_count` and `forks_count` from a GitHub REST repo response.
///
/// Extracted as a testable pure function.
///
/// # Arguments
///
/// * `slug` - Repository slug (used in error messages).
/// * `body` - Parsed JSON from `GET /repos/{owner}/{repo}`.
///
/// # Returns
///
/// `(stars, forks)` on success.
///
/// # Errors
///
/// Returns [`FetchError::Parse`] if either field is missing.
pub fn parse_repo_response(slug: &str, body: &serde_json::Value) -> Result<(u32, u32), FetchError> {
    let stars = body["stargazers_count"]
        .as_u64()
        .ok_or_else(|| FetchError::Parse(format!("{slug}: stargazers_count missing")))?
        as u32;
    let forks = body["forks_count"]
        .as_u64()
        .ok_or_else(|| FetchError::Parse(format!("{slug}: forks_count missing")))?
        as u32;
    Ok((stars, forks))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn parse_repo_response_extracts_stars_and_forks() {
        let body = serde_json::json!({
            "full_name": "JP-Ellis/tikz-feynman",
            "stargazers_count": 158,
            "forks_count": 22,
            "archived": false
        });
        let (stars, forks) =
            parse_repo_response("JP-Ellis/tikz-feynman", &body).expect("valid response");
        assert_eq!(stars, 158);
        assert_eq!(forks, 22);
    }

    #[test]
    fn parse_repo_response_errors_on_missing_stars() {
        let body = serde_json::json!({ "forks_count": 5 });
        let result = parse_repo_response("owner/repo", &body);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("stargazers_count"));
    }

    #[test]
    fn parse_repo_response_errors_on_missing_forks() {
        let body = serde_json::json!({ "stargazers_count": 100 });
        let result = parse_repo_response("owner/repo", &body);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("forks_count"));
    }
}
