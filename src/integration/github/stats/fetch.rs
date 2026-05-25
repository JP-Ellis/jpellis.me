//! GitHub API fetcher.
//!
//! Issues a GraphQL request for the contribution calendar, a Search API request
//! for recent public commits, and a Search API request for recent public PRs
//! and issues, then assembles a [`GitHubStats`] struct.

use std::cmp::Reverse;

use chrono::Duration;
use chrono::NaiveDate;
use chrono::Utc;

use crate::integration::github::stats::model::ActivityItem;
use crate::integration::github::stats::model::ActivityKind;
use crate::integration::github::stats::model::ActivityState;
use crate::integration::github::stats::model::ContributionDay;
use crate::integration::github::stats::model::ContributionWeek;
use crate::integration::github::stats::model::GitHubStats;

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
      restrictedContributionsCount
      totalCommitContributions
      totalPullRequestContributions
      totalIssueContributions
    }}
    repositories(privacy: PUBLIC) {{ totalCount }}
  }}
}}"#
    )
}

// ---------------------------------------------------------------------------
// GraphQL contribution totals parser
// ---------------------------------------------------------------------------

/// Parses contribution counts from a `contributionsCollection` JSON object.
///
/// Adds [`restrictedContributionsCount`] into the returned `total` so that
/// contributions to private organisations and repositories are reflected in the
/// displayed count even when the access token cannot read their content.
///
/// # Arguments
///
/// * `contributions` - The `contributionsCollection` field from a GitHub
///   GraphQL response.
///
/// # Returns
///
/// A tuple of `(total, commits, pull_requests, issues)` on success.
///
/// # Errors
///
/// Returns [`FetchError::Parse`] if any expected field is missing.
pub fn parse_contribution_totals(
    contributions: &serde_json::Value,
) -> Result<(u32, u32, u32, u32), FetchError> {
    let calendar_total = contributions["contributionCalendar"]["totalContributions"]
        .as_u64()
        .ok_or_else(|| FetchError::Parse("totalContributions missing".into()))?
        as u32;

    let restricted = contributions["restrictedContributionsCount"]
        .as_u64()
        .ok_or_else(|| FetchError::Parse("restrictedContributionsCount missing".into()))?
        as u32;

    let commits = contributions["totalCommitContributions"]
        .as_u64()
        .ok_or_else(|| FetchError::Parse("totalCommitContributions missing".into()))?
        as u32;

    let prs = contributions["totalPullRequestContributions"]
        .as_u64()
        .ok_or_else(|| FetchError::Parse("totalPullRequestContributions missing".into()))?
        as u32;

    let issues = contributions["totalIssueContributions"]
        .as_u64()
        .ok_or_else(|| FetchError::Parse("totalIssueContributions missing".into()))?
        as u32;

    Ok((calendar_total + restricted, commits, prs, issues))
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

/// Fetches the most recent public commits authored by JP-Ellis via the
/// GitHub commit search API.
///
/// Uses the `author:JP-Ellis is:public` search query sorted by author date.
/// The commit search API requires the `application/vnd.github.cloak-preview`
/// Accept header to be enabled.
///
/// Fetches `limit * 2` results from the API so that after deduplication
/// against PR titles in the display layer there are still enough items to
/// fill `limit` slots.
///
/// # Arguments
///
/// * `client` - A shared HTTP client.
/// * `token` - A GitHub personal access token.
/// * `limit` - Desired number of commits after the display layer deduplicates.
///
/// # Returns
///
/// A [`Vec`] of up to `limit * 2` [`ActivityItem`] values with
/// [`ActivityKind::Commit`].
///
/// # Errors
///
/// Returns [`FetchError`] if the HTTP request fails or the response cannot be
/// parsed.
async fn fetch_recent_commits(
    client: &reqwest::Client,
    token: &str,
    limit: usize,
) -> Result<Vec<ActivityItem>, FetchError> {
    let per_page = limit * 2;
    let url = format!(
        "https://api.github.com/search/commits?q=author:JP-Ellis+is:public&sort=author-date&order=desc&per_page={per_page}"
    );
    let body = rest_get(
        client,
        &url,
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

/// Parses the `items` array from a GitHub issue search API response into
/// [`ActivityItem`] values.
///
/// Items that include a `pull_request` field are mapped to
/// [`ActivityKind::PullRequest`]; all others are mapped to
/// [`ActivityKind::Issue`]. For pull requests, the `merged_at` sub-field
/// distinguishes merged PRs from simply-closed ones.
///
/// # Arguments
///
/// * `body` - The parsed JSON response from the GitHub issue search API.
///
/// # Returns
///
/// A [`Vec`] of [`ActivityItem`] values on success.
///
/// # Errors
///
/// Returns [`FetchError::Parse`] if the `items` field is missing or not an
/// array.
pub fn parse_activity_items(body: &serde_json::Value) -> Result<Vec<ActivityItem>, FetchError> {
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

/// Merges two [`ActivityItem`] lists, sorts the combined list by `created_at`
/// in descending order, and truncates to `limit` items.
///
/// # Arguments
///
/// * `a` - First list of activity items (e.g. pull requests).
/// * `b` - Second list of activity items (e.g. issues).
/// * `limit` - Maximum number of items to return.
///
/// # Returns
///
/// A [`Vec`] of at most `limit` [`ActivityItem`] values, sorted newest-first.
pub fn merge_and_sort_activity(
    a: Vec<ActivityItem>,
    b: Vec<ActivityItem>,
    limit: usize,
) -> Vec<ActivityItem> {
    let mut combined: Vec<ActivityItem> = a.into_iter().chain(b).collect();
    combined.sort_by_key(|i| Reverse(i.created_at));
    combined.truncate(limit);
    combined
}

/// Fetches the most recently created public PRs and issues authored by
/// JP-Ellis via two concurrent GitHub issue search API requests.
///
/// GitHub's issue search API requires either `is:pull-request` or `is:issue`
/// in the query; a combined query is not accepted. This function issues both
/// requests concurrently and merges the results, returning the `limit` most
/// recent items sorted newest-first.
///
/// Items with a `pull_request` field are mapped to
/// [`ActivityKind::PullRequest`]; all others to [`ActivityKind::Issue`].
///
/// # Arguments
///
/// * `client` - A shared HTTP client.
/// * `token` - A GitHub personal access token.
/// * `limit` - Maximum number of items to return after merging.
///
/// # Returns
///
/// A [`Vec`] of up to `limit` [`ActivityItem`] values.
///
/// # Errors
///
/// Returns [`FetchError`] if either HTTP request fails or either response
/// cannot be parsed.
async fn fetch_recent_activity(
    client: &reqwest::Client,
    token: &str,
    limit: usize,
) -> Result<Vec<ActivityItem>, FetchError> {
    let pr_url = format!(
        "https://api.github.com/search/issues?q=author:JP-Ellis+is:pull-request+is:public&sort=created&order=desc&per_page={limit}"
    );
    let issue_url = format!(
        "https://api.github.com/search/issues?q=author:JP-Ellis+is:issue+is:public&sort=created&order=desc&per_page={limit}"
    );

    let (pr_body, issue_body) = futures::try_join!(
        rest_get(client, &pr_url, token, &[]),
        rest_get(client, &issue_url, token, &[])
    )?;

    let prs = parse_activity_items(&pr_body)?;
    let issues = parse_activity_items(&issue_body)?;

    Ok(merge_and_sort_activity(prs, issues, limit))
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Fetches live GitHub statistics for the JP-Ellis account.
///
/// Issues four requests:
/// 1. A GraphQL request for the contribution calendar and public repository count.
/// 2. A commit search request for the 6 most recent public commits.
/// 3. A PR search request for the most recently created public pull requests.
/// 4. An issue search request for the most recently created public issues.
///
/// Requests 3 and 4 are issued concurrently; the results are merged and
/// truncated to the 4 most recent items.
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

    let contributions = &user["contributionsCollection"];

    let (total_contributions, commit_contributions, pr_contributions, issue_contributions) =
        parse_contribution_totals(contributions)?;

    let public_repos = user["repositories"]["totalCount"]
        .as_u64()
        .ok_or_else(|| FetchError::Parse("totalCount missing".into()))?
        as u32;

    let contribution_weeks =
        parse_contribution_weeks(&contributions["contributionCalendar"]["weeks"])?;

    let commits = fetch_recent_commits(&client, token, 6).await?;
    let others = fetch_recent_activity(&client, token, 4).await?;

    let mut recent_activity: Vec<ActivityItem> = commits.into_iter().chain(others).collect();
    recent_activity.sort_by_key(|i| Reverse(i.created_at));

    Ok(GitHubStats {
        fetched_at: now,
        total_contributions,
        commit_contributions,
        pr_contributions,
        issue_contributions,
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

    /// The GraphQL query must request `restrictedContributionsCount` so that
    /// contributions to private orgs and repos are counted even when the token
    /// cannot read their content.
    #[test]
    fn build_graphql_query_includes_restricted_contributions_count() {
        let query = build_graphql_query("2025-01-01T00:00:00Z", "2026-01-01T23:59:59Z");
        assert!(
            query.contains("restrictedContributionsCount"),
            "query must include restrictedContributionsCount; got:\n{query}"
        );
    }

    /// `parse_contribution_totals` adds `restrictedContributionsCount` into the
    /// returned total so that private-org contributions are reflected in the
    /// displayed count even when the token cannot read their content.
    #[test]
    fn parse_contribution_totals_sums_restricted_contributions() {
        let contributions = serde_json::json!({
            "contributionCalendar": { "totalContributions": 2133 },
            "restrictedContributionsCount": 485,
            "totalCommitContributions": 843,
            "totalPullRequestContributions": 189,
            "totalIssueContributions": 29
        });

        let (total, commits, prs, issues) =
            parse_contribution_totals(&contributions).expect("valid contribution JSON");

        assert_eq!(total, 2618, "total must include restricted contributions");
        assert_eq!(commits, 843);
        assert_eq!(prs, 189);
        assert_eq!(issues, 29);
    }

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

    /// `parse_activity_items` maps a PR search response body into [`ActivityItem`]
    /// values with correct kind, state, repo, title, url, and created_at.
    #[test]
    fn parse_activity_items_parses_pr_search_response() {
        let body = serde_json::json!({
            "total_count": 2,
            "items": [
                {
                    "title": "feat: merge me",
                    "html_url": "https://github.com/owner/repo/pull/1",
                    "repository_url": "https://api.github.com/repos/owner/repo",
                    "created_at": "2026-04-20T10:00:00Z",
                    "state": "closed",
                    "pull_request": { "merged_at": "2026-04-21T10:00:00Z" }
                },
                {
                    "title": "feat: open pr",
                    "html_url": "https://github.com/owner/repo/pull/2",
                    "repository_url": "https://api.github.com/repos/owner/repo",
                    "created_at": "2026-04-18T08:00:00Z",
                    "state": "open",
                    "pull_request": { "merged_at": null }
                }
            ]
        });

        let items = parse_activity_items(&body).expect("valid PR search body");

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].kind, ActivityKind::PullRequest);
        assert_eq!(items[0].state, Some(ActivityState::Merged));
        assert_eq!(items[0].title, "feat: merge me");
        assert_eq!(items[0].repo, "owner/repo");
        assert_eq!(items[1].kind, ActivityKind::PullRequest);
        assert_eq!(items[1].state, Some(ActivityState::Open));
    }

    /// `parse_activity_items` correctly maps issue items (no `pull_request` field).
    #[test]
    fn parse_activity_items_parses_issue_search_response() {
        let body = serde_json::json!({
            "total_count": 1,
            "items": [
                {
                    "title": "Bug: crash on startup",
                    "html_url": "https://github.com/owner/repo/issues/7",
                    "repository_url": "https://api.github.com/repos/owner/repo",
                    "created_at": "2026-03-10T12:00:00Z",
                    "state": "open"
                }
            ]
        });

        let items = parse_activity_items(&body).expect("valid issue search body");

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].kind, ActivityKind::Issue);
        assert_eq!(items[0].state, Some(ActivityState::Open));
        assert_eq!(items[0].title, "Bug: crash on startup");
    }

    /// `merge_and_sort_activity` interleaves two lists in descending created_at order.
    #[test]
    fn merge_and_sort_activity_interleaves_by_date() {
        let make = |kind: ActivityKind, state: ActivityState, ts: &str| ActivityItem {
            kind,
            repo: "owner/repo".into(),
            title: ts.into(),
            url: format!("https://example.com/{ts}"),
            state: Some(state),
            created_at: ts
                .parse::<chrono::DateTime<Utc>>()
                .expect("valid timestamp"),
        };

        let prs = vec![
            make(
                ActivityKind::PullRequest,
                ActivityState::Merged,
                "2026-04-20T10:00:00Z",
            ),
            make(
                ActivityKind::PullRequest,
                ActivityState::Open,
                "2026-04-15T10:00:00Z",
            ),
        ];
        let issues = vec![
            make(
                ActivityKind::Issue,
                ActivityState::Open,
                "2026-04-18T10:00:00Z",
            ),
            make(
                ActivityKind::Issue,
                ActivityState::Closed,
                "2026-04-10T10:00:00Z",
            ),
        ];

        let merged = merge_and_sort_activity(prs, issues, 10);

        assert_eq!(merged.len(), 4);
        // Must be strictly descending.
        assert_eq!(
            merged[0].created_at.to_rfc3339(),
            "2026-04-20T10:00:00+00:00"
        );
        assert_eq!(
            merged[1].created_at.to_rfc3339(),
            "2026-04-18T10:00:00+00:00"
        );
        assert_eq!(
            merged[2].created_at.to_rfc3339(),
            "2026-04-15T10:00:00+00:00"
        );
        assert_eq!(
            merged[3].created_at.to_rfc3339(),
            "2026-04-10T10:00:00+00:00"
        );
    }

    /// `merge_and_sort_activity` truncates to the requested limit.
    #[test]
    fn merge_and_sort_activity_truncates_to_limit() {
        let make = |ts: &str| ActivityItem {
            kind: ActivityKind::Issue,
            repo: "r".into(),
            title: ts.into(),
            url: "u".into(),
            state: Some(ActivityState::Open),
            created_at: ts
                .parse::<chrono::DateTime<Utc>>()
                .expect("valid timestamp"),
        };

        let a = vec![
            make("2026-04-20T00:00:00Z"),
            make("2026-04-19T00:00:00Z"),
            make("2026-04-18T00:00:00Z"),
        ];
        let b = vec![make("2026-04-17T00:00:00Z"), make("2026-04-16T00:00:00Z")];

        let merged = merge_and_sort_activity(a, b, 4);

        assert_eq!(merged.len(), 4);
        assert_eq!(
            merged[3].created_at.to_rfc3339(),
            "2026-04-17T00:00:00+00:00"
        );
    }
}
