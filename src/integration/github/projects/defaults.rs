#![expect(
    clippy::expect_used,
    reason = "static LazyLock initializers: corrupt embedded data should panic"
)]

use std::sync::LazyLock;

use chrono::Utc;

use crate::integration::github::projects::model::ProjectsStats;
use crate::integration::github::projects::model::RepoStats;

// ─── Fallback JSON ────────────────────────────────────────────────────────────

/// Parsed once at startup from the embedded `fallback.json`.
static FALLBACK_REPOS: LazyLock<Vec<RepoStats>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("fallback.json"))
        .expect("fallback.json is valid JSON matching Vec<RepoStats>")
});

// ─── Public API ───────────────────────────────────────────────────────────────

/// Returns placeholder [`ProjectsStats`] for use when the GitHub API is unavailable.
///
/// Repo data comes from [`FALLBACK_REPOS`] (embedded `fallback.json`), which
/// holds approximate star/fork counts recorded at the time the file was last
/// updated.  Values will grow stale over time but remain plausible; refresh by
/// editing `fallback.json` every few months once actual stats are available.
///
/// # Returns
///
/// A [`ProjectsStats`] with one [`RepoStats`] entry for every slug in
/// `src/config/projects.toml`, using hardcoded star/fork counts.
#[inline]
pub fn fallback_projects_stats() -> ProjectsStats {
    ProjectsStats {
        fetched_at: Utc::now(),
        repos: FALLBACK_REPOS.clone(),
    }
}
