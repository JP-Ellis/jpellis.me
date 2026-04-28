//! GitHub API fetcher.
//!
//! Calls the GitHub GraphQL API for contribution data and the public events REST
//! API for recent commit activity, then assembles a [`GitHubStats`] struct.
//!
//! On native (Axum / SSR) targets the two API requests are issued concurrently
//! via [`tokio::join!`].  On WASM32 (CF Workers) they are issued sequentially
//! because the CF Workers runtime is single-threaded and `tokio::join!` is not
//! available.

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
/// * `query` - The GraphQL query string.
/// * `client` - A shared HTTP client.
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

/// Fetches the authenticated user's public events from the GitHub REST API.
///
/// # Arguments
///
/// * `client` - A shared HTTP client.
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
async fn get_public_events(
    client: &reqwest::Client,
    token: &str,
) -> Result<serde_json::Value, FetchError> {
    let resp = client
        .get("https://api.github.com/users/JP-Ellis/events/public")
        .header("Authorization", format!("Bearer {token}"))
        .header("User-Agent", "jpellis-me/1.0")
        .send()
        .await
        .map_err(|e| FetchError::Http(e.to_string()))?;
    if !resp.status().is_success() {
        return Err(FetchError::Http(format!("Events status {}", resp.status())));
    }
    resp.json::<serde_json::Value>()
        .await
        .map_err(|e| FetchError::Parse(e.to_string()))
}

// ---------------------------------------------------------------------------
// GraphQL query builder
// ---------------------------------------------------------------------------

/// Builds the GraphQL query for the contribution calendar, repositories,
/// pull requests, and issues over the given date range.
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
    pullRequests(last: 20, orderBy: {{ field: UPDATED_AT, direction: DESC }}) {{
      nodes {{
        title createdAt state url
        repository {{ nameWithOwner isPrivate }}
      }}
    }}
    issues(last: 20, orderBy: {{ field: UPDATED_AT, direction: DESC }}) {{
      nodes {{
        title createdAt state url
        repository {{ nameWithOwner isPrivate }}
      }}
    }}
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

/// Parses pull request or issue nodes from the GraphQL response into
/// [`ActivityItem`] values, filtering out items from private repositories.
///
/// # Arguments
///
/// * `nodes` - The `nodes` array from the GraphQL pull requests or issues field.
/// * `kind` - The [`ActivityKind`] to assign to each parsed item.
///
/// # Returns
///
/// A [`Vec`] of [`ActivityItem`] structs (private repos are silently excluded).
fn parse_activity_items(nodes: &serde_json::Value, kind: ActivityKind) -> Vec<ActivityItem> {
    let Some(arr) = nodes.as_array() else {
        return vec![];
    };
    arr.iter()
        .filter(|node| {
            node["repository"]["isPrivate"]
                .as_bool()
                .map(|p| !p)
                .unwrap_or(true)
        })
        .filter_map(|node| {
            let title = node["title"].as_str()?.to_string();
            let url = node["url"].as_str()?.to_string();
            let repo = node["repository"]["nameWithOwner"].as_str()?.to_string();
            let created_at = node["createdAt"]
                .as_str()
                .and_then(|s| s.parse::<chrono::DateTime<Utc>>().ok())?;
            let state = match node["state"].as_str() {
                Some("OPEN") => Some(ActivityState::Open),
                Some("CLOSED") => Some(ActivityState::Closed),
                Some("MERGED") => Some(ActivityState::Merged),
                _ => None,
            };
            Some(ActivityItem {
                kind: kind.clone(),
                repo,
                title,
                url,
                state,
                created_at,
            })
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Commit event parsing from REST
// ---------------------------------------------------------------------------

/// Parses `PushEvent` entries from the GitHub public events REST response into
/// [`ActivityItem`] values.
///
/// Only `PushEvent` entries are included; all other event types are ignored.
/// At most 10 commit items are returned (the most recent).
///
/// # Arguments
///
/// * `events` - The JSON array returned by the public events endpoint.
///
/// # Returns
///
/// A [`Vec`] of up to 10 [`ActivityItem`] structs with
/// [`ActivityKind::Commit`].
fn parse_commit_events(events: &serde_json::Value) -> Vec<ActivityItem> {
    let Some(arr) = events.as_array() else {
        return vec![];
    };
    arr.iter()
        .filter(|e| e["type"].as_str() == Some("PushEvent"))
        .filter_map(|e| {
            let repo = e["repo"]["name"].as_str()?.to_string();
            let commits = e["payload"]["commits"].as_array()?;
            let head = commits.last()?;
            let title = head["message"].as_str()?.lines().next()?.to_string();
            let sha = head["sha"].as_str()?;
            let url = format!("https://github.com/{repo}/commit/{sha}");
            let created_at = e["created_at"]
                .as_str()
                .and_then(|s| s.parse::<chrono::DateTime<Utc>>().ok())?;
            Some(ActivityItem {
                kind: ActivityKind::Commit,
                repo,
                title,
                url,
                state: None,
                created_at,
            })
        })
        .take(10)
        .collect()
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Fetches live GitHub statistics for the JP-Ellis account.
///
/// Issues a GraphQL request (contribution calendar, repositories, PRs, issues)
/// and a REST request (public push events) concurrently, then merges the
/// results into a [`GitHubStats`] struct.
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
/// Returns [`FetchError::Http`] if either API call fails, or
/// [`FetchError::Parse`] if the response cannot be interpreted.
pub async fn fetch_from_github(token: &str) -> Result<GitHubStats, FetchError> {
    let now = Utc::now();
    let period_to = now.date_naive();
    let period_from = period_to - Duration::days(365);
    let from = format!("{}T00:00:00Z", period_from);
    let to = format!("{}T23:59:59Z", period_to);

    let client = reqwest::Client::new();
    let query = build_graphql_query(&from, &to);

    // On native targets issue requests concurrently; on WASM32 (CF Workers)
    // the runtime is single-threaded so sequential calls are used instead.
    #[cfg(not(target_arch = "wasm32"))]
    let (gql_result, events_result) = tokio::join!(
        graphql(&client, &query, token),
        get_public_events(&client, token)
    );
    #[cfg(target_arch = "wasm32")]
    let (gql_result, events_result) = {
        let gql = graphql(&client, &query, token).await;
        let events = get_public_events(&client, token).await;
        (gql, events)
    };

    let gql = gql_result?;
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

    let mut activity: Vec<ActivityItem> = vec![];

    match events_result {
        Ok(events) => activity.extend(parse_commit_events(&events)),
        Err(e) => {
            #[cfg(not(target_arch = "wasm32"))]
            leptos::logging::warn!("Public events fetch failed (commit activity omitted): {e}");
            #[cfg(target_arch = "wasm32")]
            worker::console_error!("Public events fetch failed (commit activity omitted): {e}");
        }
    }
    activity.extend(parse_activity_items(
        &user["pullRequests"]["nodes"],
        ActivityKind::PullRequest,
    ));
    activity.extend(parse_activity_items(
        &user["issues"]["nodes"],
        ActivityKind::Issue,
    ));

    activity.sort_by_key(|item| std::cmp::Reverse(item.created_at));
    activity.truncate(8);

    Ok(GitHubStats {
        fetched_at: now,
        total_contributions,
        public_repos,
        period_from,
        period_to,
        contribution_weeks,
        recent_activity: activity,
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
    fn parse_activity_items_filters_private_repos() {
        let nodes = serde_json::json!([
            {
                "title": "public PR",
                "url": "https://github.com/pub/repo/pull/1",
                "createdAt": "2026-04-28T10:00:00Z",
                "state": "OPEN",
                "repository": { "nameWithOwner": "pub/repo", "isPrivate": false }
            },
            {
                "title": "private PR",
                "url": "https://github.com/priv/repo/pull/2",
                "createdAt": "2026-04-28T09:00:00Z",
                "state": "MERGED",
                "repository": { "nameWithOwner": "priv/repo", "isPrivate": true }
            }
        ]);
        let items = parse_activity_items(&nodes, ActivityKind::PullRequest);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "public PR");
    }

    #[test]
    fn parse_commit_events_extracts_push_events_only() {
        let events = serde_json::json!([
            {
                "type": "WatchEvent",
                "repo": { "name": "other/repo" },
                "created_at": "2026-04-28T10:00:00Z",
                "payload": {}
            },
            {
                "type": "PushEvent",
                "repo": { "name": "JP-Ellis/pact-python" },
                "created_at": "2026-04-28T09:00:00Z",
                "payload": {
                    "commits": [
                        { "sha": "abc123", "message": "feat: add thing\n\nBody text" }
                    ]
                }
            }
        ]);
        let commits = parse_commit_events(&events);
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].title, "feat: add thing");
        assert_eq!(commits[0].repo, "JP-Ellis/pact-python");
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
}
