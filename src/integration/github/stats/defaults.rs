#![expect(
    clippy::expect_used,
    reason = "static LazyLock initializer and bounded arithmetic: corrupt embedded data should panic"
)]
use std::sync::LazyLock;

use chrono::DateTime;
use chrono::Duration;
use chrono::NaiveDate;
use chrono::Utc;
use serde::Deserialize;

use crate::integration::github::stats::model::ActivityItem;
use crate::integration::github::stats::model::ActivityKind;
use crate::integration::github::stats::model::ActivityState;
use crate::integration::github::stats::model::ContributionDay;
use crate::integration::github::stats::model::ContributionWeek;
use crate::integration::github::stats::model::GitHubStats;

// ─── Fallback JSON ────────────────────────────────────────────────────────────

/// Deserialization shape for `fallback.json`.
#[derive(Deserialize)]
struct FallbackData {
    /// Total commit contributions in the last year.
    commit_contributions: u32,
    /// Total pull request contributions in the last year.
    pr_contributions: u32,
    /// Total issue contributions in the last year.
    issue_contributions: u32,
    /// Number of public repositories.
    public_repos: u32,
    /// Recent activity items to display in the activity feed.
    recent_activity: Vec<FallbackActivity>,
}

/// A single recent-activity entry as stored in `fallback.json`.
#[derive(Deserialize)]
struct FallbackActivity {
    /// Type of activity (commit, PR, issue, etc.).
    kind: ActivityKind,
    /// Repository slug (e.g. `"owner/repo"`).
    repo: String,
    /// Human-readable title of the activity item.
    title: String,
    /// URL linking to the activity item on GitHub.
    url: String,
    /// State of the item, if applicable (e.g. open/closed/merged).
    state: Option<ActivityState>,
    /// Real timestamp from the GitHub API, written by `scripts/update-fallback-data.rs`.
    created_at: DateTime<Utc>,
}

/// Parsed once at startup from the embedded `fallback.json`.
static FALLBACK: LazyLock<FallbackData> = LazyLock::new(|| {
    serde_json::from_str(include_str!("fallback.json"))
        .expect("fallback.json is valid JSON matching FallbackData")
});

// ─── Public API ───────────────────────────────────────────────────────────────

/// Returns placeholder [`GitHubStats`] for use when the GitHub API is unavailable.
///
/// Scalar counts come from [`FALLBACK`] (embedded `fallback.json`).  The
/// contribution grid is generated deterministically via a linear congruential
/// generator.  Dates are relative to the current time, so the placeholder
/// remains timely regardless of when the binary was compiled.
///
/// # Returns
///
/// A [`GitHubStats`] struct with placeholder data that renders correctly in the UI.
///
/// # Example
///
/// ```rust
/// let stats = fallback_stats();
/// assert_eq!(stats.public_repos, 14);
/// assert_eq!(stats.contribution_weeks.len(), 53);
/// ```
#[must_use = "the fallback stats are discarded if not used"]
#[inline]
pub fn fallback_stats() -> GitHubStats {
    let now = Utc::now();
    let today = now.date_naive();
    #[expect(
        clippy::arithmetic_side_effects,
        reason = "subtracting a fixed small duration from a real date cannot overflow NaiveDate"
    )]
    let period_from = today - Duration::days(364);
    let period_to = today;

    let contribution_weeks = fallback_grid(period_from);
    let total_contributions: u32 = contribution_weeks
        .iter()
        .flat_map(|w| w.days.iter())
        .map(|d| d.count)
        .sum();

    let fb = &*FALLBACK;

    let recent_activity = fb
        .recent_activity
        .iter()
        .map(|a| ActivityItem {
            kind: a.kind.clone(),
            repo: a.repo.clone(),
            title: a.title.clone(),
            url: a.url.clone(),
            state: a.state.clone(),
            created_at: a.created_at,
        })
        .collect();

    GitHubStats {
        fetched_at: now,
        total_contributions,
        commit_contributions: fb.commit_contributions,
        pr_contributions: fb.pr_contributions,
        issue_contributions: fb.issue_contributions,
        public_repos: fb.public_repos,
        period_from,
        period_to,
        contribution_weeks,
        recent_activity,
    }
}

// ─── Grid generation ─────────────────────────────────────────────────────────

/// Generates a deterministic 53-week contribution grid for fallback display.
fn fallback_grid(start: NaiveDate) -> Vec<ContributionWeek> {
    #[expect(
        clippy::integer_division_remainder_used,
        reason = "modulo is part of the LCG algorithm; intentional"
    )]
    #[expect(
        clippy::float_arithmetic,
        reason = "float division to normalise LCG output to [0.0, 1.0)"
    )]
    fn lcg(s: &mut u64) -> f64 {
        *s = s.wrapping_mul(9301).wrapping_add(49297) % 233_280;
        #[expect(
            clippy::as_conversions,
            clippy::cast_precision_loss,
            reason = "seed is bounded to [0, 233_280) so precision loss is negligible"
        )]
        let result = *s as f64 / 233_280.0_f64;
        result
    }

    let mut s: u64 = 11;
    let mut weeks = Vec::with_capacity(53);

    for w in 0..53_usize {
        let mut days = Vec::with_capacity(7);
        for d in 0..7_usize {
            let v = lcg(&mut s).powf(1.6_f64);
            let burst = if w > 32_usize && w < 44_usize && lcg(&mut s) > 0.4_f64 {
                0.3_f64
            } else {
                0.0_f64
            };
            #[expect(
                clippy::as_conversions,
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                clippy::float_arithmetic,
                reason = "value is clamped to [0.0, 1.0] then scaled to [0.0, 10.0]; truncation to u32 is intentional"
            )]
            let raw = ((v + burst).min(1.0_f64) * 10.0_f64) as u32;
            #[expect(
                clippy::arithmetic_side_effects,
                reason = "w and d are bounded by loop limits (53 and 7), so w * 7 + d cannot overflow usize"
            )]
            let offset = i64::try_from(w * 7_usize + d)
                .expect("w*7+d is bounded by loop limits (max 52*7+6 = 370) and fits in i64");
            #[expect(
                clippy::arithmetic_side_effects,
                reason = "offset is bounded by loop (max 370 days); adding small duration to a real date cannot overflow NaiveDate"
            )]
            let date = start + Duration::days(offset);
            days.push(ContributionDay { date, count: raw });
        }
        weeks.push(ContributionWeek { days });
    }
    weeks
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[expect(
        clippy::indexing_slicing,
        reason = "test assertions on known non-empty fallback data"
    )]
    fn fallback_stats_determinism() {
        let stats1 = fallback_stats();
        let stats2 = fallback_stats();

        assert_eq!(
            stats1.total_contributions, stats2.total_contributions,
            "total_contributions should be deterministic"
        );
        assert_eq!(
            stats1.public_repos, stats2.public_repos,
            "public_repos should be deterministic"
        );
        assert_eq!(
            stats1.recent_activity[0].title, stats2.recent_activity[0].title,
            "first activity title should be deterministic"
        );
        assert_eq!(
            stats1.contribution_weeks[0].days[0].count, stats2.contribution_weeks[0].days[0].count,
            "grid[0][0] count should be deterministic"
        );
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn fallback_grid_shape() {
        let stats = fallback_stats();

        assert_eq!(
            stats.contribution_weeks.len(),
            53,
            "contribution_weeks should have exactly 53 entries"
        );

        for (week_idx, week) in stats.contribution_weeks.iter().enumerate() {
            assert_eq!(
                week.days.len(),
                7,
                "week {} should have exactly 7 days",
                week_idx
            );

            for (day_idx, day) in week.days.iter().enumerate() {
                assert!(
                    day.count <= 10,
                    "week {} day {} count should be 0-10, got {}",
                    week_idx,
                    day_idx,
                    day.count
                );
            }
        }
    }
}
