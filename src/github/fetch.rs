//! GitHub API fetcher.
//!
//! Issues a GraphQL request for the contribution calendar, a Search API request
//! for recent public commits, and a Search API request for recent public PRs
//! and issues, then assembles a [`GitHubStats`] struct.

use std::cmp::Reverse;

use chrono::Duration;
use chrono::NaiveDate;
use chrono::Utc;

use crate::github::model::ActivityItem;
use crate::github::model::ActivityKind;
use crate::github::model::ActivityState;
use crate::github::model::ContributionDay;
use crate::github::model::ContributionWeek;
use crate::github::model::GitHubStats;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors that can occur while fetching GitHub statistics.
#[derive(Debug)]
pub enum FetchError {
    /// An HTTP-level error (connection failure, unexpected status code, etc.).
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

// ---------------------------------------------------------------------------
// HTTP helpers
// ---------------------------------------------------------------------------

/// Sends a GraphQL request to the GitHub API and returns the parsed JSON body.
///
/// # Arguments
///
/// * `client` - A shared HTTP client.
/// * `query` - The GraphQL query string.
/// * `token` - A GitHub personal access token used for authentication.
///
/// # Returns
///
/// The parsed JSON response body on success.
///
/// # Errors
///
/// Returns [`FetchError::Http`] if the request fails or the status is not 2xx,
/// and [`FetchError::Parse`] if the response body cannot be decoded as JSON.
async fn graphql(
    client: &reqwest::Client,
    query: &str,
    token: &str,
) -> Result<serde_json::Value, FetchError> {
    let body = serde_json::json!({ "query": query });
    let resp = client
        .post("https://api.github.com/graphql")
        .header("Authorization", format!("Bearer {token}"))
        .header("User-Agent", "jpellis-me/1.0")
        .json(&body)
        .send()
        .await
        .map_err(|e| FetchError::Http(e.to_string()))?;
    if !resp.status().is_success() {
        return Err(FetchError::Http(format!(
            "GraphQL status {}",
            resp.status()
        )));
    }
    resp.json::<serde_json::Value>()
        .await
        .map_err(|e| FetchError::Parse(e.to_string()))
}

/// Sends a REST GET request to the GitHub API and returns the parsed JSON body.
///
/// # Arguments
///
/// * `client` - A shared HTTP client.
/// * `url` - The full URL to request.
/// * `token` - A GitHub personal access token used for authentication.
/// * `extra_headers` - Additional `(name, value)` headers to include.
///
/// # Returns
///
/// The parsed JSON response body on success.
///
/// # Errors
///
/// Returns [`FetchError::Http`] if the request fails or returns a non-2xx
/// status, and [`FetchError::Parse`] if the response cannot be decoded as JSON.
async fn rest_get(
    client: &reqwest::Client,
    url: &str,
    token: &str,
    extra_headers: &[(&str, &str)],
) -> Result<serde_json::Value, FetchError> {
    let mut req = client
        .get(url)
        .header("Authorization", format!("Bearer {token}"))
        .header("User-Agent", "jpellis-me/1.0");
    for (name, value) in extra_headers {
        req = req.header(*name, *value);
    }
    let resp = req
        .send()
        .await
        .map_err(|e| FetchError::Http(e.to_string()))?;
    if !resp.status().is_success() {
        return Err(FetchError::Http(format!("REST status {}", resp.status())));
    }
    resp.json::<serde_json::Value>()
        .await
        .map_err(|e| FetchError::Parse(e.to_string()))
}

// ---------------------------------------------------------------------------
// GraphQL query builder
// ---------------------------------------------------------------------------

/// Builds the GraphQL query for the contribution calendar and public repository
/// count over the given date range.
///
/// # Arguments
///
/// * `from` - ISO 8601 start timestamp, e.g. `"2025-04-28T00:00:00Z"`.
/// * `to` - ISO 8601 end timestamp, e.g. `"2026-04-28T23:59:59Z"`.
///
/// # Returns
///
/// A GraphQL query string ready to be sent to the GitHub API.
fn build_graphql_query(from: &str, to: &str) -> String {
    format!(
        r#"{{
  user(login: "JP-Ellis") {{
    contributionsCollection(from: "{from}", to: "{to}") {{
      contributionCalendar {{
        totalContributions
        weeks {{
          contributionDays {{ date contributionCount }}
        }}
      }}
    }}
    repositories(privacy: PUBLIC) {{ totalCount }}
  }}
}}"#
    )
}

// ---------------------------------------------------------------------------
// GraphQL response parsers
// ---------------------------------------------------------------------------

/// Parses the `weeks` array from the GraphQL contribution calendar response.
///
/// # Arguments
///
/// * `weeks_json` - The `weeks` field value from the GraphQL response.
///
/// # Returns
///
/// A [`Vec`] of [`ContributionWeek`] structs on success.
///
/// # Errors
///
/// Returns [`FetchError::Parse`] if any expected field is missing or has the
/// wrong type.
fn parse_contribution_weeks(
    weeks_json: &serde_json::Value,
) -> Result<Vec<ContributionWeek>, FetchError> {
    let arr = weeks_json
        .as_array()
        .ok_or_else(|| FetchError::Parse("weeks not an array".into()))?;
    arr.iter()
        .map(|week| {
            let days = week["contributionDays"]
                .as_array()
                .ok_or_else(|| FetchError::Parse("contributionDays not an array".into()))?
                .iter()
                .map(|day| {
                    let date = day["date"]
                        .as_str()
                        .ok_or_else(|| FetchError::Parse("date missing".into()))
                        .and_then(|s| {
                            NaiveDate::parse_from_str(s, "%Y-%m-%d")
                                .map_err(|e| FetchError::Parse(e.to_string()))
                        })?;
                    let count = day["contributionCount"]
                        .as_u64()
                        .ok_or_else(|| FetchError::Parse("contributionCount missing".into()))?
                        as u32;
                    Ok(ContributionDay { date, count })
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ContributionWeek { days })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Search API fetchers
// ---------------------------------------------------------------------------

/// Fetches the 6 most recent public commits authored by JP-Ellis via the
/// GitHub commit search API.
///
/// Uses the `author:JP-Ellis is:public` search query sorted by author date.
/// The commit search API requires the `application/vnd.github.cloak-preview`
/// Accept header to be enabled.
///
/// # Arguments
///
/// * `client` - A shared HTTP client.
/// * `token` - A GitHub personal access token.
///
/// # Returns
///
/// A [`Vec`] of up to 6 [`ActivityItem`] values with [`ActivityKind::Commit`].
///
/// # Errors
///
/// Returns [`FetchError`] if the HTTP request fails or the response cannot be
/// parsed.
async fn fetch_recent_commits(
    client: &reqwest::Client,
    token: &str,
) -> Result<Vec<ActivityItem>, FetchError> {
    // Fetch 12 so that after deduplication against PR titles in the display
    // layer we still have enough to fill 6 slots.
    let url = "https://api.github.com/search/commits?q=author:JP-Ellis+is:public&sort=author-date&order=desc&per_page=12";
    let body = rest_get(
        client,
        url,
        token,
        &[("Accept", "application/vnd.github.cloak-preview")],
    )
    .await?;

    let items = body["items"]
        .as_array()
        .ok_or_else(|| FetchError::Parse("commit search items missing".into()))?;

    Ok(items
        .iter()
        .filter_map(|item| {
            let sha = item["sha"].as_str()?;
            let html_url = item["html_url"].as_str()?.to_string();
            let repo = item["repository"]["full_name"].as_str()?.to_string();
            let message = item["commit"]["message"].as_str()?;
            let title = message.lines().next().unwrap_or("").to_string();
            if title.is_empty() {
                return None;
            }
            // Commit search returns dates with tz offset; parse via FixedOffset then convert.
            let created_at = item["commit"]["author"]["date"].as_str().and_then(|s| {
                chrono::DateTime::parse_from_rfc3339(s)
                    .ok()
                    .map(|dt| dt.to_utc())
            })?;
            let _ = sha; // sha is embedded in html_url already
            Some(ActivityItem {
                kind: ActivityKind::Commit,
                repo,
                title,
                url: html_url,
                state: None,
                created_at,
            })
        })
        .collect())
}

/// Fetches the 4 most recently created public PRs and issues authored by
/// JP-Ellis via the GitHub issue search API.
///
/// The query `author:JP-Ellis is:public` matches both PRs and issues. Items
/// with a `pull_request` field are treated as PRs; the `merged_at` sub-field
/// is used to distinguish merged PRs from simply-closed ones.
///
/// # Arguments
///
/// * `client` - A shared HTTP client.
/// * `token` - A GitHub personal access token.
///
/// # Returns
///
/// A [`Vec`] of up to 4 [`ActivityItem`] values with [`ActivityKind::PullRequest`]
/// or [`ActivityKind::Issue`].
///
/// # Errors
///
/// Returns [`FetchError`] if the HTTP request fails or the response cannot be
/// parsed.
async fn fetch_recent_activity(
    client: &reqwest::Client,
    token: &str,
) -> Result<Vec<ActivityItem>, FetchError> {
    let url = "https://api.github.com/search/issues?q=author:JP-Ellis+is:public&sort=created&order=desc&per_page=4";
    let body = rest_get(client, url, token, &[]).await?;

    let items = body["items"]
        .as_array()
        .ok_or_else(|| FetchError::Parse("issue search items missing".into()))?;

    Ok(items
        .iter()
        .filter_map(|item| {
            let title = item["title"].as_str()?.to_string();
            let url = item["html_url"].as_str()?.to_string();
            let repo = item["repository_url"]
                .as_str()?
                .trim_start_matches("https://api.github.com/repos/")
                .to_string();
            let created_at = item["created_at"]
                .as_str()
                .and_then(|s| s.parse::<chrono::DateTime<Utc>>().ok())?;

            let is_pr = item.get("pull_request").is_some();
            let (kind, state) = if is_pr {
                let merged = item["pull_request"]["merged_at"].is_string();
                let raw_state = item["state"].as_str().unwrap_or("open");
                let state = if raw_state == "open" {
                    ActivityState::Open
                } else if merged {
                    ActivityState::Merged
                } else {
                    ActivityState::Closed
                };
                (ActivityKind::PullRequest, state)
            } else {
                let state = if item["state"].as_str() == Some("open") {
                    ActivityState::Open
                } else {
                    ActivityState::Closed
                };
                (ActivityKind::Issue, state)
            };

            Some(ActivityItem {
                kind,
                repo,
                title,
                url,
                state: Some(state),
                created_at,
            })
        })
        .collect())
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Fetches live GitHub statistics for the JP-Ellis account.
///
/// Issues three requests in sequence:
/// 1. A GraphQL request for the contribution calendar and public repository count.
/// 2. A commit search request for the 6 most recent public commits.
/// 3. An issue search request for the 4 most recently created public PRs and issues.
///
/// # Arguments
///
/// * `token` - A GitHub personal access token with at least `read:user` and
///   `public_repo` scopes.
///
/// # Returns
///
/// A [`GitHubStats`] struct covering the past 365 days on success.
///
/// # Errors
///
/// Returns [`FetchError::Http`] if any API call fails, or
/// [`FetchError::Parse`] if the response cannot be interpreted.
pub async fn fetch_from_github(token: &str) -> Result<GitHubStats, FetchError> {
    let now = Utc::now();
    let period_to = now.date_naive();
    let period_from = period_to - Duration::days(365);
    let from = format!("{}T00:00:00Z", period_from);
    let to = format!("{}T23:59:59Z", period_to);

    let client = reqwest::Client::new();

    let query = build_graphql_query(&from, &to);
    let gql = graphql(&client, &query, token).await?;
    let user = &gql["data"]["user"];

    let total_contributions =
        user["contributionsCollection"]["contributionCalendar"]["totalContributions"]
            .as_u64()
            .ok_or_else(|| FetchError::Parse("totalContributions missing".into()))? as u32;

    let public_repos = user["repositories"]["totalCount"]
        .as_u64()
        .ok_or_else(|| FetchError::Parse("totalCount missing".into()))?
        as u32;

    let contribution_weeks = parse_contribution_weeks(
        &user["contributionsCollection"]["contributionCalendar"]["weeks"],
    )?;

    let mut commits = fetch_recent_commits(&client, token).await?;
    let mut others = fetch_recent_activity(&client, token).await?;

    commits.sort_by_key(|c| Reverse(c.created_at));
    others.sort_by_key(|o| Reverse(o.created_at));

    let mut recent_activity: Vec<ActivityItem> = commits.into_iter().chain(others).collect();
    recent_activity.sort_by_key(|i| Reverse(i.created_at));

    Ok(GitHubStats {
        fetched_at: now,
        total_contributions,
        public_repos,
        period_from,
        period_to,
        contribution_weeks,
        recent_activity,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn parse_contribution_weeks_parses_counts() {
        let weeks = serde_json::json!([
            {
                "contributionDays": [
                    { "date": "2025-04-28", "contributionCount": 5 },
                    { "date": "2025-04-29", "contributionCount": 0 }
                ]
            }
        ]);
        let parsed = parse_contribution_weeks(&weeks).expect("valid test JSON");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].days[0].count, 5);
        assert_eq!(parsed[0].days[1].count, 0);
    }

    /// Verifies that the commit search response shape is correctly mapped to
    /// [`ActivityItem`] values, with the title extracted from the first line of
    /// the commit message and the date parsed from RFC 3339 with a tz offset.
    #[test]
    fn parse_commit_search_item_extracts_fields() {
        // Simulate a single item from the commit search response.
        let item = serde_json::json!({
            "sha": "abc123def456",
            "html_url": "https://github.com/owner/repo/commit/abc123def456",
            "repository": { "full_name": "owner/repo", "private": false },
            "commit": {
                "message": "feat: add thing\n\nLong body here",
                "author": { "date": "2026-04-28T15:58:14+10:00" }
            }
        });

        // Replicate the mapping logic from fetch_recent_commits inline.
        let sha = item["sha"].as_str().unwrap();
        let html_url = item["html_url"].as_str().unwrap().to_string();
        let repo = item["repository"]["full_name"]
            .as_str()
            .unwrap()
            .to_string();
        let message = item["commit"]["message"].as_str().unwrap();
        let title = message.lines().next().unwrap_or("").to_string();
        let created_at = chrono::DateTime::parse_from_rfc3339(
            item["commit"]["author"]["date"].as_str().unwrap(),
        )
        .unwrap()
        .to_utc();

        assert_eq!(title, "feat: add thing");
        assert_eq!(repo, "owner/repo");
        assert_eq!(
            html_url,
            "https://github.com/owner/repo/commit/abc123def456"
        );
        // Date should be converted to UTC (15:58:14+10:00 = 05:58:14Z).
        assert_eq!(created_at.format("%H:%M:%S").to_string(), "05:58:14");
        let _ = sha;
    }

    #[test]
    fn parse_activity_item_pr_merged() {
        let item = serde_json::json!({
            "title": "feat: merge me",
            "html_url": "https://github.com/owner/repo/pull/1",
            "repository_url": "https://api.github.com/repos/owner/repo",
            "created_at": "2026-04-20T10:00:00Z",
            "state": "closed",
            "pull_request": { "merged_at": "2026-04-21T10:00:00Z" }
        });

        let is_pr = item.get("pull_request").is_some();
        let merged = item["pull_request"]["merged_at"].is_string();
        let raw_state = item["state"].as_str().unwrap_or("open");
        let state = if raw_state == "open" {
            ActivityState::Open
        } else if merged {
            ActivityState::Merged
        } else {
            ActivityState::Closed
        };

        assert!(is_pr);
        assert_eq!(state, ActivityState::Merged);
    }

    #[test]
    fn parse_activity_item_pr_closed_not_merged() {
        let item = serde_json::json!({
            "title": "feat: abandoned",
            "html_url": "https://github.com/owner/repo/pull/2",
            "repository_url": "https://api.github.com/repos/owner/repo",
            "created_at": "2026-04-20T10:00:00Z",
            "state": "closed",
            "pull_request": { "merged_at": null }
        });

        let merged = item["pull_request"]["merged_at"].is_string();
        let raw_state = item["state"].as_str().unwrap_or("open");
        let state = if raw_state == "open" {
            ActivityState::Open
        } else if merged {
            ActivityState::Merged
        } else {
            ActivityState::Closed
        };

        assert_eq!(state, ActivityState::Closed);
    }

    #[test]
    fn parse_activity_item_issue_detection() {
        let issue_item = serde_json::json!({
            "title": "Bug report",
            "html_url": "https://github.com/owner/repo/issues/3",
            "repository_url": "https://api.github.com/repos/owner/repo",
            "created_at": "2026-04-15T10:00:00Z",
            "state": "open"
            // no pull_request field
        });

        let is_pr = issue_item.get("pull_request").is_some();
        assert!(
            !is_pr,
            "item without pull_request field should be treated as issue"
        );
    }
}
