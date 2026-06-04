//! Fetches per-repository stats and activity from the GitHub REST API.
#![expect(
    clippy::indexing_slicing,
    reason = "serde_json Value indexing returns Null for missing keys, not a panic"
)]

use chrono::Utc;
use futures::future;

use crate::integration::github::projects::model::CommitInfo;
use crate::integration::github::projects::model::ProjectsStats;
use crate::integration::github::projects::model::ReleaseInfo;
use crate::integration::github::projects::model::RepoStats;

/// Errors that can occur while fetching projects statistics.
#[non_exhaustive]
#[derive(Debug)]
pub enum FetchError {
    /// An HTTP-level error.
    Http(String),
    /// A JSON parsing or field extraction error.
    Parse(String),
}

impl std::fmt::Display for FetchError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::Http(e) => write!(f, "HTTP error: {e}"),
            FetchError::Parse(e) => write!(f, "Parse error: {e}"),
        }
    }
}

impl std::error::Error for FetchError {}

// MARK: Pure parse functions

/// Parses `stargazers_count`, `forks_count`, `open_issues_count`, and
/// `watchers_count` from a GitHub REST repo response.
///
/// # Arguments
///
/// * `slug` - Repository slug (used in error messages).
/// * `body` - Parsed JSON from `GET /repos/{owner}/{repo}`.
///
/// # Returns
///
/// `(stars, forks, open_issues, watchers)` on success.
///
/// # Errors
///
/// Returns [`FetchError::Parse`] if any required field is missing.
#[must_use = "the parsed stats are discarded if not used"]
#[inline]
pub fn parse_repo_response(
    slug: &str,
    body: &serde_json::Value,
) -> Result<(u32, u32, u32, u32), FetchError> {
    let stars = u32::try_from(
        body["stargazers_count"]
            .as_u64()
            .ok_or_else(|| FetchError::Parse(format!("{slug}: stargazers_count missing")))?,
    )
    .map_err(|e| FetchError::Parse(format!("{slug}: stargazers_count exceeds u32: {e}")))?;
    let forks = u32::try_from(
        body["forks_count"]
            .as_u64()
            .ok_or_else(|| FetchError::Parse(format!("{slug}: forks_count missing")))?,
    )
    .map_err(|e| FetchError::Parse(format!("{slug}: forks_count exceeds u32: {e}")))?;
    let open_issues = u32::try_from(
        body["open_issues_count"]
            .as_u64()
            .ok_or_else(|| FetchError::Parse(format!("{slug}: open_issues_count missing")))?,
    )
    .map_err(|e| FetchError::Parse(format!("{slug}: open_issues_count exceeds u32: {e}")))?;
    let watchers = u32::try_from(
        body["watchers_count"]
            .as_u64()
            .ok_or_else(|| FetchError::Parse(format!("{slug}: watchers_count missing")))?,
    )
    .map_err(|e| FetchError::Parse(format!("{slug}: watchers_count exceeds u32: {e}")))?;
    Ok((stars, forks, open_issues, watchers))
}

/// Parses a GitHub releases/latest response into [`ReleaseInfo`].
///
/// Returns `None` if any required field is absent (including 404-style
/// `{ "message": "Not Found" }` responses).
///
/// # Arguments
///
/// * `body` - Parsed JSON from `GET /repos/{owner}/{repo}/releases/latest`.
///
/// # Returns
///
/// `Some(ReleaseInfo)` on success, `None` otherwise.
#[must_use = "the parsed release info is discarded if not used"]
#[inline]
pub fn parse_release_response(body: &serde_json::Value) -> Option<ReleaseInfo> {
    let tag = body["tag_name"].as_str()?.to_owned();
    let date = body["published_at"].as_str()?.to_owned();
    let url = body["html_url"].as_str()?.to_owned();
    Some(ReleaseInfo { tag, date, url })
}

/// Parses a single commit object from the GitHub commits list API.
///
/// Returns `None` if the commit is authored by a bot (`author.type == "Bot"`)
/// or if required fields are missing. Commits with a `null` GitHub author
/// (not associated with a GitHub account) are treated as human commits.
///
/// # Arguments
///
/// * `commit` - A single element from `GET /repos/{owner}/{repo}/commits`.
///
/// # Returns
///
/// `Some(CommitInfo)` for human commits, `None` for bots or unparsable entries.
#[must_use = "the parsed commit info is discarded if not used"]
#[inline]
pub fn parse_commit(commit: &serde_json::Value) -> Option<CommitInfo> {
    // Filter bots: author object is present AND type is "Bot"
    if let Some(author_obj) = commit["author"].as_object()
        && author_obj.get("type").and_then(|t| t.as_str()) == Some("Bot")
    {
        return None;
    }

    let full_sha = commit["sha"].as_str()?;
    let sha = full_sha.get(..7).unwrap_or(full_sha).to_owned();

    let full_message = commit["commit"]["message"].as_str().unwrap_or("");
    let first_line = full_message.lines().next().unwrap_or("").trim();
    let message = if first_line.chars().count() > 72 {
        let truncated: String = first_line.chars().take(72).collect();
        format!("{truncated}…")
    } else {
        first_line.to_owned()
    };

    let date = commit["commit"]["author"]["date"]
        .as_str()
        .unwrap_or("")
        .to_owned();

    // For User accounts use the GitHub login; for null author (not linked
    // to a GitHub account) fall back to the git committer name.
    let author = if let Some(login) = commit["author"]["login"].as_str() {
        login.to_owned()
    } else {
        commit["commit"]["author"]["name"]
            .as_str()
            .unwrap_or("unknown")
            .to_owned()
    };

    let url = commit["html_url"].as_str().unwrap_or("").to_owned();

    Some(CommitInfo {
        sha,
        message,
        date,
        author,
        url,
    })
}

/// Parses a GitHub commits list response, filtering bots and capping at 5.
///
/// # Arguments
///
/// * `body` - Parsed JSON array from `GET /repos/{owner}/{repo}/commits`.
///
/// # Returns
///
/// Up to 5 [`CommitInfo`] entries for non-bot commits.
#[must_use = "the parsed commits are discarded if not used"]
#[inline]
pub fn parse_commits_response(body: &serde_json::Value) -> Vec<CommitInfo> {
    let Some(arr) = body.as_array() else {
        return vec![];
    };
    arr.iter().filter_map(parse_commit).take(5).collect()
}

/// Counts open pull requests from a GitHub pulls list response.
///
/// # Arguments
///
/// * `body` - Parsed JSON array from `GET /repos/{owner}/{repo}/pulls?state=open`.
///
/// # Returns
///
/// Number of open PRs (0 if response is not an array).
#[must_use = "the PR count is discarded if not used"]
#[inline]
pub fn parse_prs_count(body: &serde_json::Value) -> u32 {
    body.as_array()
        .map_or(0, |a| u32::try_from(a.len()).unwrap_or(u32::MAX))
}

// MARK: HTTP helpers

/// Performs a GET request and returns the parsed JSON body.
async fn get_json(
    client: &reqwest::Client,
    url: &str,
    token: &str,
) -> Result<serde_json::Value, FetchError> {
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .header("User-Agent", "jpellis-me/1.0")
        .send()
        .await
        .map_err(|e| FetchError::Http(e.to_string()))?;

    let status = resp.status();
    let body = resp
        .json::<serde_json::Value>()
        .await
        .map_err(|e| FetchError::Parse(e.to_string()))?;

    if !status.is_success() {
        return Err(FetchError::Http(format!("GET {url} returned {status}")));
    }

    Ok(body)
}

/// Fetches the latest release for a repository, returning `None` if none exists.
async fn fetch_latest_release(
    client: &reqwest::Client,
    slug: &str,
    token: &str,
) -> Option<ReleaseInfo> {
    let url = format!("https://api.github.com/repos/{slug}/releases/latest");
    // 404 is expected for repos with no releases — treat as None, not an error
    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {token}"))
        .header("User-Agent", "jpellis-me/1.0")
        .send()
        .await
        .ok()?;

    if resp.status().as_u16() == 404 {
        return None;
    }

    let body = resp.json::<serde_json::Value>().await.ok()?;
    parse_release_response(&body)
}

/// Fetches recent commits for a repository, returning an empty list on error.
async fn fetch_recent_commits(
    client: &reqwest::Client,
    slug: &str,
    token: &str,
) -> Vec<CommitInfo> {
    // Fetch 20 to provide a bot-filtering buffer; we cap the result at 5 human commits
    let url = format!("https://api.github.com/repos/{slug}/commits?per_page=20");
    let body = match get_json(client, &url, token).await {
        Ok(b) => b,
        Err(e) => {
            leptos::logging::warn!("projects::fetch commits {slug}: {e}");
            return vec![];
        }
    };
    parse_commits_response(&body)
}

/// Fetches the number of open pull requests for a repository.
async fn fetch_open_prs_count(client: &reqwest::Client, slug: &str, token: &str) -> u32 {
    let url = format!("https://api.github.com/repos/{slug}/pulls?state=open&per_page=100");
    let body = match get_json(client, &url, token).await {
        Ok(b) => b,
        Err(e) => {
            leptos::logging::warn!("projects::fetch prs {slug}: {e}");
            return 0;
        }
    };
    parse_prs_count(&body)
}

/// Fetches all stats and activity for a single repository.
async fn fetch_single_repo(
    client: &reqwest::Client,
    slug: &str,
    token: &str,
) -> Result<RepoStats, FetchError> {
    use futures::join;
    let repo_url = format!("https://api.github.com/repos/{slug}");
    let repo_body = get_json(client, &repo_url, token).await?;
    let (stars, forks, open_issues, watchers) = parse_repo_response(slug, &repo_body)?;

    // Fetch activity concurrently after we know the repo exists
    let (latest_release, recent_commits, open_prs) = join!(
        fetch_latest_release(client, slug, token),
        fetch_recent_commits(client, slug, token),
        fetch_open_prs_count(client, slug, token),
    );

    Ok(RepoStats {
        slug: slug.to_owned(),
        stars,
        forks,
        open_issues,
        watchers,
        latest_release,
        recent_commits,
        open_prs,
    })
}

/// Fetches stats and activity for the given repository slugs concurrently.
///
/// Repos that fail to fetch are logged and skipped.
///
/// # Arguments
///
/// * `token` - A GitHub personal access token with at least `public_repo` read scope.
/// * `slugs` - Repository slugs from [`crate::config::projects::projects_config`].
///
/// # Returns
///
/// A [`ProjectsStats`] with one [`RepoStats`] per successfully fetched repo.
#[must_use = "the fetched stats are discarded if not used"]
#[inline]
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

#[cfg(test)]
mod tests {
    #![expect(
        clippy::default_numeric_fallback,
        reason = "integer literals in serde_json::json! test fixtures — type inference is unambiguous in context"
    )]

    use pretty_assertions::assert_eq;

    use super::*;

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn parse_repo_response_extracts_all_four_fields() {
        let body = serde_json::json!({
            "stargazers_count": 158_i64,
            "forks_count": 22_i64,
            "open_issues_count": 5_i64,
            "watchers_count": 12_i64
        });
        let (stars, forks, open_issues, watchers) =
            parse_repo_response("JP-Ellis/tikz-feynman", &body).expect("valid");
        assert_eq!(stars, 158_u32);
        assert_eq!(forks, 22_u32);
        assert_eq!(open_issues, 5_u32);
        assert_eq!(watchers, 12_u32);
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[expect(
        clippy::unwrap_used,
        reason = "test assertion on expected error variant"
    )]
    fn parse_repo_response_errors_on_missing_stars() {
        let body = serde_json::json!({
            "forks_count": 5_i64,
            "open_issues_count": 0_i64,
            "watchers_count": 1_i64,
        });
        let result = parse_repo_response("owner/repo", &body);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("stargazers_count"));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[expect(
        clippy::unwrap_used,
        reason = "test assertion on expected error variant"
    )]
    fn parse_repo_response_errors_on_missing_forks() {
        let body = serde_json::json!({
            "stargazers_count": 100_i64,
            "open_issues_count": 0_i64,
            "watchers_count": 1_i64,
        });
        let result = parse_repo_response("owner/repo", &body);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("forks_count"));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn parse_release_response_extracts_release_info() {
        let body = serde_json::json!({
            "tag_name": "v3.1.0",
            "published_at": "2025-01-14T10:00:00Z",
            "html_url": "https://github.com/JP-Ellis/tikz-feynman/releases/tag/v3.1.0"
        });
        let info = parse_release_response(&body).expect("should parse");
        assert_eq!(info.tag, "v3.1.0");
        assert_eq!(info.date, "2025-01-14T10:00:00Z");
        assert_eq!(
            info.url,
            "https://github.com/JP-Ellis/tikz-feynman/releases/tag/v3.1.0"
        );
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn parse_release_response_returns_none_on_missing_fields() {
        let body = serde_json::json!({ "message": "Not Found" });
        assert!(parse_release_response(&body).is_none());
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn parse_commit_skips_bot_by_type() {
        let commit = serde_json::json!({
            "sha": "abc1234567890",
            "commit": {
                "message": "chore(deps): update deps\nMore details",
                "author": { "name": "renovate[bot]", "date": "2025-05-01T09:00:00Z" }
            },
            "author": { "login": "renovate[bot]", "type": "Bot" },
            "html_url": "https://github.com/owner/repo/commit/abc1234"
        });
        assert!(parse_commit(&commit).is_none());
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn parse_commit_accepts_null_author_as_human() {
        // Commits not associated with a GitHub account have null author
        let commit = serde_json::json!({
            "sha": "abc1234567890",
            "commit": {
                "message": "Initial commit",
                "author": { "name": "Someone", "date": "2025-05-01T09:00:00Z" }
            },
            "author": null,
            "html_url": "https://github.com/owner/repo/commit/abc1234"
        });
        // null author = not associated with a GitHub user; treat as non-bot
        let info = parse_commit(&commit).expect("null author is not a bot");
        assert_eq!(info.author, "Someone");
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn parse_commit_returns_commit_info_for_human() {
        let commit = serde_json::json!({
            "sha": "a1b2c3d4e5f6",
            "commit": {
                "message": "Fix diagram spacing\n\nLonger description here",
                "author": { "name": "Joshua Ellis", "date": "2025-05-01T09:00:00Z" }
            },
            "author": { "login": "JP-Ellis", "type": "User" },
            "html_url": "https://github.com/JP-Ellis/tikz-feynman/commit/a1b2c3d"
        });
        let info = parse_commit(&commit).expect("human commit");
        assert_eq!(info.sha, "a1b2c3d"); // 7-char short SHA
        assert_eq!(info.message, "Fix diagram spacing"); // first line only
        assert_eq!(info.author, "JP-Ellis"); // GitHub login for Users
        assert_eq!(info.date, "2025-05-01T09:00:00Z");
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn parse_commit_truncates_long_message() {
        // Use multi-byte characters to verify char-based (not byte-based) truncation
        let long_msg = format!("{}\nSecond line", "🚀".repeat(80));
        let commit = serde_json::json!({
            "sha": "a1b2c3d4e5f6",
            "commit": {
                "message": long_msg,
                "author": { "name": "JP-Ellis", "date": "2025-01-01T00:00:00Z" }
            },
            "author": { "login": "JP-Ellis", "type": "User" },
            "html_url": "https://example.com/commit/abc"
        });
        let info = parse_commit(&commit).expect("human commit");
        // 72 chars × 4 bytes each + 3 bytes for "…" = 291 bytes max
        // The important thing is it doesn't panic and ends with "…"
        assert!(info.message.ends_with('…'));
        // Confirm char count is exactly 73 (72 truncated chars + "…" ellipsis = 1 char)
        assert_eq!(info.message.chars().count(), 73);
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn parse_commits_response_filters_bots_and_caps_at_five() {
        // 7 commits: 2 bots + 5 humans → should return 5 humans
        let mut commits = Vec::new();
        for i in 0..5 {
            commits.push(serde_json::json!({
                "sha": format!("human{i}abcdef"),
                "commit": {
                    "message": format!("Human commit {i}"),
                    "author": { "name": "JP-Ellis", "date": "2025-05-01T09:00:00Z" }
                },
                "author": { "login": "JP-Ellis", "type": "User" },
                "html_url": "https://example.com/commit/abc"
            }));
        }
        for i in 0..2 {
            commits.push(serde_json::json!({
                "sha": format!("bot{i}abcdefgh"),
                "commit": {
                    "message": format!("chore(deps): bot commit {i}"),
                    "author": { "name": "renovate[bot]", "date": "2025-05-01T09:00:00Z" }
                },
                "author": { "login": "renovate[bot]", "type": "Bot" },
                "html_url": "https://example.com/commit/bot"
            }));
        }
        let body = serde_json::Value::Array(commits);
        let result = parse_commits_response(&body);
        assert_eq!(result.len(), 5);
        assert!(result.iter().all(|c| !c.author.contains("bot")));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn parse_prs_count_counts_array_length() {
        let body = serde_json::json!([
            { "number": 1_i64 },
            { "number": 2_i64 },
            { "number": 3_i64 }
        ]);
        assert_eq!(parse_prs_count(&body), 3_u32);
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn parse_prs_count_returns_zero_for_empty() {
        let body = serde_json::json!([]);
        assert_eq!(parse_prs_count(&body), 0);
    }
}
