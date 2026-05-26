#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
chrono = { version = "0.4", default-features = false, features = ["clock", "serde", "std"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml_edit = { version = "0.25", features = ["serde"] }
---
//! Refreshes the fallback JSON data baked into the binary.
//!
//! Writes two files from live GitHub API data:
//!   - `src/integration/github/stats/fallback.json`
//!   - `src/integration/github/work/fallback.json`
//!
//! Run from the repository root:
//!
//!   ./scripts/update-fallback-data.rs
//!
//! Requires an authenticated `gh` CLI (`gh auth status` should succeed).
//!
//! The script fetches:
//!   - Contribution counts (commits, PRs, issues, public repos) via GraphQL.
//!   - Recent public activity (commits + PRs + issues) via the Search API.
//!   - Star and fork counts for every repo in `tracked_slugs` via GraphQL.

use std::path::Path;
use std::process::Command;

// MARK: Configuration

const GITHUB_USER: &str = "JP-Ellis";
const WORK_CONFIG_PATH: &str = "src/config/work.toml";
const STATS_FALLBACK_PATH: &str = "src/integration/github/stats/fallback.json";
const WORK_FALLBACK_PATH: &str = "src/integration/github/work/fallback.json";

/// Number of recent activity items stored in the stats fallback.
///
/// Matches the live site behaviour: 6 commits + up to 4 PRs/issues merged and
/// sorted newest-first.
const COMMIT_FETCH: usize = 12; // fetch 2× so deduplication leaves enough
const PR_FETCH: usize = 8;
const ISSUE_FETCH: usize = 8;
const ACTIVITY_KEEP: usize = 10;

// MARK: Types

#[derive(Debug, serde::Deserialize)]
struct WorkConfig {
    tracked_slugs: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
struct StatsFallback {
    commit_contributions: u32,
    pr_contributions: u32,
    issue_contributions: u32,
    public_repos: u32,
    recent_activity: Vec<ActivityItem>,
}

#[derive(Debug, serde::Serialize)]
struct ActivityItem {
    kind: String,
    repo: String,
    title: String,
    url: String,
    state: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, serde::Serialize)]
struct RepoStats {
    slug: String,
    stars: u64,
    forks: u64,
}

// MARK: Main

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()?;

    let config_str = std::fs::read_to_string(root.join(WORK_CONFIG_PATH))
        .map_err(|e| format!("Could not read {WORK_CONFIG_PATH}: {e}"))?;
    let config: WorkConfig = toml_edit::de::from_str(&config_str)
        .map_err(|e| format!("Could not parse {WORK_CONFIG_PATH}: {e}"))?;

    // ── Stats fallback ────────────────────────────────────────────────────────
    eprintln!("Fetching GitHub stats for @{GITHUB_USER}...");
    let stats = fetch_stats()?;
    let stats_json = serde_json::to_string_pretty(&stats)?;
    std::fs::write(root.join(STATS_FALLBACK_PATH), format!("{stats_json}\n"))?;
    println!(
        "Updated {STATS_FALLBACK_PATH} — {} activity items, {}/{}/{} commit/PR/issue contributions",
        stats.recent_activity.len(),
        stats.commit_contributions,
        stats.pr_contributions,
        stats.issue_contributions,
    );

    // ── Work fallback ─────────────────────────────────────────────────────────
    eprintln!(
        "\nFetching star/fork counts for {} repos...",
        config.tracked_slugs.len()
    );
    let repos = fetch_repos(&config.tracked_slugs)?;
    let work_json = serde_json::to_string_pretty(&repos)?;
    std::fs::write(root.join(WORK_FALLBACK_PATH), format!("{work_json}\n"))?;
    println!("Updated {WORK_FALLBACK_PATH} — {} repos", repos.len());

    Ok(())
}

// MARK: Stats fetching

fn fetch_stats() -> Result<StatsFallback, Box<dyn std::error::Error>> {
    // 1. Contribution counts via GraphQL.
    let now = chrono::Utc::now();
    let from = format!(
        "{}T00:00:00Z",
        (now - chrono::Duration::days(365)).format("%Y-%m-%d")
    );
    let to = format!("{}T23:59:59Z", now.format("%Y-%m-%d"));

    let query = format!(
        r#"{{
  user(login: "{GITHUB_USER}") {{
    contributionsCollection(from: "{from}", to: "{to}") {{
      totalCommitContributions
      totalPullRequestContributions
      totalIssueContributions
    }}
    repositories(privacy: PUBLIC) {{ totalCount }}
  }}
}}"#
    );

    let gql = gh_graphql(&query)?;
    let user = &gql["data"]["user"];
    let contribs = &user["contributionsCollection"];

    let commit_contributions =
        contribs["totalCommitContributions"].as_u64().unwrap_or(0) as u32;
    let pr_contributions =
        contribs["totalPullRequestContributions"].as_u64().unwrap_or(0) as u32;
    let issue_contributions =
        contribs["totalIssueContributions"].as_u64().unwrap_or(0) as u32;
    let public_repos = user["repositories"]["totalCount"].as_u64().unwrap_or(0) as u32;

    eprintln!(
        "  contributions: {commit_contributions} commits / {pr_contributions} PRs / {issue_contributions} issues, {public_repos} public repos"
    );

    // 2. Recent commits.
    eprintln!("  Fetching recent commits...");
    let commit_body = gh_rest(
        &format!(
            "/search/commits?q=author:{GITHUB_USER}+is:public&sort=author-date&order=desc&per_page={COMMIT_FETCH}"
        ),
        &[("Accept", "application/vnd.github.cloak-preview")],
    )?;
    let commits = parse_commits(&commit_body);
    eprintln!("  → {} commits", commits.len());

    // 3. Recent PRs.
    eprintln!("  Fetching recent PRs...");
    let pr_body = gh_rest(
        &format!(
            "/search/issues?q=author:{GITHUB_USER}+is:pull-request+is:public&sort=created&order=desc&per_page={PR_FETCH}"
        ),
        &[],
    )?;
    let prs = parse_activity(&pr_body);
    eprintln!("  → {} PRs", prs.len());

    // 4. Recent issues.
    eprintln!("  Fetching recent issues...");
    let issue_body = gh_rest(
        &format!(
            "/search/issues?q=author:{GITHUB_USER}+is:issue+is:public&sort=created&order=desc&per_page={ISSUE_FETCH}"
        ),
        &[],
    )?;
    let issues = parse_activity(&issue_body);
    eprintln!("  → {} issues", issues.len());

    // 5. Merge, sort newest-first, truncate.
    let mut all: Vec<ActivityItem> = commits.into_iter().chain(prs).chain(issues).collect();
    all.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    all.truncate(ACTIVITY_KEEP);

    eprintln!("\n  Recent activity ({} items):", all.len());
    for item in &all {
        let state = item
            .state
            .as_deref()
            .map(|s| format!(" [{s}]"))
            .unwrap_or_default();
        eprintln!("    [{}] {}{}: {}", item.kind, item.repo, state, item.title);
    }

    Ok(StatsFallback {
        commit_contributions,
        pr_contributions,
        issue_contributions,
        public_repos,
        recent_activity: all,
    })
}

fn parse_commits(body: &serde_json::Value) -> Vec<ActivityItem> {
    body["items"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|item| {
            let html_url = item["html_url"].as_str()?.to_string();
            let repo = item["repository"]["full_name"].as_str()?.to_string();
            let message = item["commit"]["message"].as_str()?;
            let title = message.lines().next().unwrap_or("").to_string();
            if title.is_empty() {
                return None;
            }
            let created_at = item["commit"]["author"]["date"]
                .as_str()
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.to_utc())?;
            Some(ActivityItem {
                kind: "commit".to_string(),
                repo,
                title,
                url: html_url,
                state: None,
                created_at,
            })
        })
        .collect()
}

fn parse_activity(body: &serde_json::Value) -> Vec<ActivityItem> {
    body["items"]
        .as_array()
        .unwrap_or(&vec![])
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
                .and_then(|s| s.parse::<chrono::DateTime<chrono::Utc>>().ok())?;

            let is_pr = item.get("pull_request").is_some();
            let (kind, state) = if is_pr {
                let merged = item["pull_request"]["merged_at"].is_string();
                let raw = item["state"].as_str().unwrap_or("open");
                let state = if raw == "open" {
                    "open"
                } else if merged {
                    "merged"
                } else {
                    "closed"
                };
                ("pull_request", state)
            } else {
                let state = if item["state"].as_str() == Some("open") {
                    "open"
                } else {
                    "closed"
                };
                ("issue", state)
            };

            Some(ActivityItem {
                kind: kind.to_string(),
                repo,
                title,
                url,
                state: Some(state.to_string()),
                created_at,
            })
        })
        .collect()
}

// MARK: Work fetching

/// Fetches star and fork counts for all slugs in a single batched GraphQL query.
fn fetch_repos(slugs: &[String]) -> Result<Vec<RepoStats>, Box<dyn std::error::Error>> {
    let fields: Vec<String> = slugs
        .iter()
        .enumerate()
        .filter_map(|(i, slug)| {
            let mut parts = slug.splitn(2, '/');
            let owner = parts.next()?;
            let name = parts.next()?;
            Some(format!(
                "  r{i}: repository(owner: \"{owner}\", name: \"{name}\") {{ stargazerCount forks {{ totalCount }} }}"
            ))
        })
        .collect();
    let query = format!("{{\n{}\n}}", fields.join("\n"));
    let data = gh_graphql(&query)?["data"].clone();

    let mut repos = Vec::new();
    for (i, slug) in slugs.iter().enumerate() {
        let node = &data[format!("r{i}")];
        if node.is_null() {
            eprintln!("  warning: {slug} not found");
            continue;
        }
        let stars = node["stargazerCount"].as_u64().unwrap_or(0);
        let forks = node["forks"]["totalCount"].as_u64().unwrap_or(0);
        eprintln!("  {slug}: {stars}★  {forks}⑂");
        repos.push(RepoStats {
            slug: slug.clone(),
            stars,
            forks,
        });
    }

    Ok(repos)
}

// MARK: gh CLI helpers

fn gh_graphql(query: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let output = Command::new("gh")
        .args(["api", "graphql", "-f", &format!("query={query}")])
        .output()
        .map_err(|e| format!("Failed to run `gh`: {e}. Is the gh CLI installed and authenticated?"))?;

    if !output.status.success() {
        return Err(format!(
            "`gh api graphql` failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(serde_json::from_slice(&output.stdout)?)
}

fn gh_rest(
    path: &str,
    extra_headers: &[(&str, &str)],
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut args: Vec<String> = vec!["api".to_string(), path.to_string()];
    for (name, value) in extra_headers {
        args.extend(["-H".to_string(), format!("{name}: {value}")]);
    }

    let output = Command::new("gh")
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to run `gh api {path}`: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "`gh api {path}` failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(serde_json::from_slice(&output.stdout)?)
}
