use std::sync::LazyLock;

use chrono::Utc;

use crate::integration::github::work::model::RepoStats;
use crate::integration::github::work::model::WorkStats;

// ─── Fallback JSON ────────────────────────────────────────────────────────────

/// Parsed once at startup from the embedded `fallback.json`.
static FALLBACK_REPOS: LazyLock<Vec<RepoStats>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("fallback.json"))
        .expect("fallback.json is valid JSON matching Vec<RepoStats>")
});

// ─── Public API ───────────────────────────────────────────────────────────────

/// Returns placeholder [`WorkStats`] for use when the GitHub API is unavailable.
///
/// Repo data comes from [`FALLBACK_REPOS`] (embedded `fallback.json`), which
/// holds approximate star/fork counts recorded at the time the file was last
/// updated.  Values will grow stale over time but remain plausible; refresh by
/// editing `fallback.json` every few months once actual stats are available.
///
/// # Returns
///
/// A [`WorkStats`] with one [`RepoStats`] entry for every slug in
/// `src/config/work.toml`, using hardcoded star/fork counts.
pub fn fallback_work_stats() -> WorkStats {
    WorkStats {
        fetched_at: Utc::now(),
        repos: FALLBACK_REPOS.clone(),
    }
}
