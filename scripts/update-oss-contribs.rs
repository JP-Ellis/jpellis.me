#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml_edit = { version = "0.25", features = ["serde"] }
chrono = { version = "0.4", default-features = false, features = ["clock", "std"] }
---
//! Updates the `[[oss_contribs]]` section of `src/config/work.toml` from live
//! GitHub contribution data.
//!
//! Run from the repository root:
//!
//!   ./scripts/update-oss-contribs.rs
//!
//! Requires an authenticated `gh` CLI.
//!
//! The script:
//!   1. Fetches your contributions for the past year via the GitHub GraphQL API.
//!   2. Filters out own repos, work-org repos, and repos with no PRs.
//!   3. Preserves any existing custom display names.
//!   4. Sorts by GitHub star count descending (so the most notable appear first).
//!   5. Rewrites the `[[oss_contribs]]` section in `src/config/work.toml`.

use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

// MARK: Configuration

/// GitHub login whose contributions are analysed.
const GITHUB_USER: &str = "JP-Ellis";

/// Orgs/owners excluded entirely from the misc list.
///
/// Note: individual repos already in `tracked_slugs` are also excluded
/// regardless of org, so e.g. `pact-foundation/pact-python` is deduped
/// automatically without needing its org here.
const SKIP_ORGS: &[&str] = &["JP-Ellis", "hep-rs", "pactflow"];

/// Minimum number of PRs to qualify for the misc list.
const MIN_PRS: u64 = 1;

/// Path to the config file, relative to the repository root.
const CONFIG_PATH: &str = "src/config/work.toml";

// MARK: Types

#[derive(Debug, serde::Deserialize)]
struct WorkConfig {
    tracked_slugs: Vec<String>,
    #[serde(default)]
    oss_contribs: Vec<OssContrib>,
}

#[derive(Debug, serde::Deserialize)]
struct OssContrib {
    slug: String,
    name: String,
}

#[derive(Debug, Default)]
struct RepoActivity {
    commits: u64,
    prs: u64,
    is_private: bool,
}

// MARK: Main
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(CONFIG_PATH)
        .canonicalize()?;

    if !config_path.exists() {
        return Err(format!("Config file not found at {CONFIG_PATH}").into());
    }

    let config_str = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Could not read {CONFIG_PATH}: {e}"))?;
    let config: WorkConfig = toml_edit::de::from_str(&config_str)
        .map_err(|e| format!("Could not parse {CONFIG_PATH}: {e}"))?;

    // Build a name-override map from the current entries.
    let existing_names: HashMap<&str, &str> = config
        .oss_contribs
        .iter()
        .map(|c| (c.slug.as_str(), c.name.as_str()))
        .collect();

    let from = year_ago_rfc3339();
    let to = now_rfc3339();
    eprintln!("Fetching contributions {from} → {to} for @{GITHUB_USER}...");

    let activity = fetch_contributions(&from, &to)?;

    // Build a set of slugs that are already in tracked_slugs (main PROJECTS).
    let tracked: std::collections::HashSet<&str> =
        config.tracked_slugs.iter().map(|s| s.as_str()).collect();

    // Filter.
    let mut filtered: Vec<String> = activity
        .iter()
        .filter(|(slug, data)| {
            let org = slug.split('/').next().unwrap_or("");
            !SKIP_ORGS.contains(&org)
                && !tracked.contains(slug.as_str())
                && !data.is_private
                && data.prs >= MIN_PRS
        })
        .map(|(slug, _)| slug.clone())
        .collect();
    filtered.sort(); // stable base order before star sort

    // Fetch star counts and sort by popularity descending.
    eprintln!("Fetching star counts for {} repos...", filtered.len());
    let stars = fetch_stars(&filtered).unwrap_or_default();
    filtered.sort_by(|a, b| {
        stars
            .get(b)
            .copied()
            .unwrap_or(0)
            .cmp(&stars.get(a).copied().unwrap_or(0))
            .then(a.cmp(b))
    });

    // Build (slug, display_name, stars) triples, preserving custom names.
    let entries: Vec<(String, String, u64)> = filtered
        .iter()
        .map(|slug| {
            let default_name = slug.split('/').nth(1).unwrap_or(slug.as_str());
            let name = existing_names
                .get(slug.as_str())
                .copied()
                .unwrap_or(default_name)
                .to_string();
            let star_count = stars.get(slug).copied().unwrap_or(0);
            (slug.clone(), name, star_count)
        })
        .collect();

    let patched = patch_oss_contribs(&config_str, &entries)?;
    std::fs::write(&config_path, &patched)
        .map_err(|e| format!("Could not write {CONFIG_PATH}: {e}"))?;

    println!(
        "Updated {CONFIG_PATH} — {} OSS contrib entries:",
        entries.len()
    );
    for slug in &filtered {
        let is_new = !existing_names.contains_key(slug.as_str());
        let star_count = stars.get(slug).copied().unwrap_or(0);
        let star_str = if star_count >= 1_000 {
            format!("{:.1}k★", star_count as f64 / 1_000.0)
        } else {
            format!("{star_count}★")
        };
        let tag = if is_new { "  [new]" } else { "" };
        println!("  {slug} ({star_str}){tag}");
    }

    Ok(())
}

// MARK: GitHub API
fn fetch_contributions(
    from: &str,
    to: &str,
) -> Result<HashMap<String, RepoActivity>, Box<dyn std::error::Error>> {
    let query = format!(
        r#"{{
  user(login: "{GITHUB_USER}") {{
    contributionsCollection(from: "{from}", to: "{to}") {{
      commitContributionsByRepository(maxRepositories: 100) {{
        repository {{ nameWithOwner isPrivate }}
        contributions {{ totalCount }}
      }}
      pullRequestContributionsByRepository(maxRepositories: 100) {{
        repository {{ nameWithOwner isPrivate }}
        contributions {{ totalCount }}
      }}
    }}
  }}
}}"#
    );

    let output = Command::new("gh")
        .args(["api", "graphql", "-f", &format!("query={query}")])
        .output()
        .map_err(|e| {
            format!("Failed to run `gh`: {e}. Is the gh CLI installed and authenticated?")
        })?;

    if !output.status.success() {
        return Err(format!(
            "`gh api graphql` failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let collection = &json["data"]["user"]["contributionsCollection"];

    let mut activity: HashMap<String, RepoActivity> = HashMap::new();

    for item in collection["commitContributionsByRepository"]
        .as_array()
        .unwrap_or(&vec![])
    {
        let slug = item["repository"]["nameWithOwner"]
            .as_str()
            .unwrap_or("")
            .to_string();
        if slug.is_empty() {
            continue;
        }
        let private = item["repository"]["isPrivate"].as_bool().unwrap_or(true);
        let count = item["contributions"]["totalCount"].as_u64().unwrap_or(0);
        let entry = activity.entry(slug).or_default();
        entry.commits += count;
        entry.is_private = private;
    }

    for item in collection["pullRequestContributionsByRepository"]
        .as_array()
        .unwrap_or(&vec![])
    {
        let slug = item["repository"]["nameWithOwner"]
            .as_str()
            .unwrap_or("")
            .to_string();
        if slug.is_empty() {
            continue;
        }
        let private = item["repository"]["isPrivate"].as_bool().unwrap_or(true);
        let count = item["contributions"]["totalCount"].as_u64().unwrap_or(0);
        let entry = activity.entry(slug).or_default();
        entry.prs += count;
        entry.is_private = private;
    }

    Ok(activity)
}

/// Fetches the star count for each slug in a single batched GraphQL query.
///
/// Uses aliased `repository` fields (`r0`, `r1`, …) so only one round-trip is
/// needed regardless of list length.  Repos that fail to resolve are silently
/// omitted (the caller falls back to 0 stars).
fn fetch_stars(slugs: &[String]) -> Result<HashMap<String, u64>, Box<dyn std::error::Error>> {
    if slugs.is_empty() {
        return Ok(HashMap::new());
    }

    let fields: Vec<String> = slugs
        .iter()
        .enumerate()
        .filter_map(|(i, slug)| {
            let mut parts = slug.splitn(2, '/');
            let owner = parts.next()?;
            let name = parts.next()?;
            Some(format!(
                "  r{i}: repository(owner: \"{owner}\", name: \"{name}\") {{ stargazerCount }}"
            ))
        })
        .collect();
    let query = format!("{{\n{}\n}}", fields.join("\n"));

    let output = Command::new("gh")
        .args(["api", "graphql", "-f", &format!("query={query}")])
        .output()
        .map_err(|e| format!("Failed to fetch stars: {e}"))?;

    if !output.status.success() {
        return Err(format!(
            "Star fetch failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let data = &json["data"];

    let mut result = HashMap::new();
    for (i, slug) in slugs.iter().enumerate() {
        if let Some(n) = data[format!("r{i}")]["stargazerCount"].as_u64() {
            result.insert(slug.clone(), n);
        }
    }

    Ok(result)
}

// MARK: File patching
/// Replaces the `[[oss_contribs]]` array-of-tables in the TOML document with
/// `entries`, preserving all other keys verbatim.
fn patch_oss_contribs(
    content: &str,
    entries: &[(String, String, u64)],
) -> Result<String, Box<dyn std::error::Error>> {
    let mut doc = content
        .parse::<toml_edit::DocumentMut>()
        .map_err(|e| format!("Could not parse {CONFIG_PATH} as TOML: {e}"))?;

    doc.remove("oss_contribs");

    let mut aot = toml_edit::ArrayOfTables::new();
    for (i, (slug, name, stars)) in entries.iter().enumerate() {
        let mut table = toml_edit::Table::new();
        table["slug"] = toml_edit::value(slug.as_str());
        table["name"] = toml_edit::value(name.as_str());
        table["stars"] = toml_edit::value(*stars as i64);
        let prefix = if i == 0 {
            "\n# Misc OSS contributions - updated by: cargo scripts/update-oss-contribs.rs\n"
        } else {
            "\n"
        };
        table.decor_mut().set_prefix(prefix);
        aot.push(table);
    }
    doc["oss_contribs"] = toml_edit::Item::ArrayOfTables(aot);

    Ok(doc.to_string())
}

// ─── Date helpers ────────────────────────────────────────────────────────────

fn year_ago_rfc3339() -> String {
    let dt = chrono::Utc::now() - chrono::Duration::days(365);
    dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn now_rfc3339() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}
