use chrono::Duration;
use chrono::NaiveDate;
use chrono::Utc;

use crate::integration::github::stats::model::ActivityItem;
use crate::integration::github::stats::model::ActivityKind;
use crate::integration::github::stats::model::ActivityState;
use crate::integration::github::stats::model::ContributionDay;
use crate::integration::github::stats::model::ContributionWeek;
use crate::integration::github::stats::model::GitHubStats;

/// Returns placeholder [`GitHubStats`] for use when the GitHub API is unavailable.
///
/// The contribution grid is generated deterministically via a linear congruential
/// generator, so it always produces the same plausible-looking pattern. Dates are
/// relative to the current time so the placeholder remains timely.
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
pub fn fallback_stats() -> GitHubStats {
    let now = Utc::now();
    let today = now.date_naive();
    let period_from = today - Duration::days(364);
    let period_to = today;

    let contribution_weeks = fallback_grid(period_from);
    let total_contributions: u32 = contribution_weeks
        .iter()
        .flat_map(|w| w.days.iter())
        .map(|d| d.count)
        .sum();

    GitHubStats {
        fetched_at: now,
        total_contributions,
        commit_contributions: 547,
        pr_contributions: 23,
        issue_contributions: 18,
        public_repos: 14,
        period_from,
        period_to,
        contribution_weeks,
        recent_activity: vec![
            ActivityItem {
                kind: ActivityKind::Commit,
                repo: "pact-foundation/pact-python".to_string(),
                title: "feat(ffi): bind verifier results".to_string(),
                url: "https://github.com/pact-foundation/pact-python".to_string(),
                state: None,
                created_at: now - Duration::days(4),
            },
            ActivityItem {
                kind: ActivityKind::Commit,
                repo: "JP-Ellis/jpellis.me".to_string(),
                title: "fix: fetch recent prs and issues".to_string(),
                url: "https://github.com/JP-Ellis/jpellis.me".to_string(),
                state: None,
                created_at: now - Duration::days(5),
            },
            ActivityItem {
                kind: ActivityKind::PullRequest,
                repo: "pact-foundation/pact-python".to_string(),
                title: "feat(ffi): verifier FFI bindings".to_string(),
                url: "https://github.com/pact-foundation/pact-python/pull/1".to_string(),
                state: Some(ActivityState::Merged),
                created_at: now - Duration::days(8),
            },
            ActivityItem {
                kind: ActivityKind::Issue,
                repo: "pact-foundation/pact-python".to_string(),
                title: "Support for Pact v4 plugins".to_string(),
                url: "https://github.com/pact-foundation/pact-python/issues/1".to_string(),
                state: Some(ActivityState::Open),
                created_at: now - Duration::days(12),
            },
        ],
    }
}

fn fallback_grid(start: NaiveDate) -> Vec<ContributionWeek> {
    fn lcg(s: &mut u64) -> f64 {
        *s = s.wrapping_mul(9301).wrapping_add(49297) % 233280;
        *s as f64 / 233280.0
    }

    let mut s: u64 = 11;
    let mut weeks = Vec::with_capacity(53);

    for w in 0..53_usize {
        let mut days = Vec::with_capacity(7);
        for d in 0..7_usize {
            let v = lcg(&mut s).powf(1.6);
            let burst = if w > 32 && w < 44 && lcg(&mut s) > 0.4 {
                0.3_f64
            } else {
                0.0_f64
            };
            let raw = ((v + burst).min(1.0) * 10.0) as u32;
            let date = start + Duration::days((w * 7 + d) as i64);
            days.push(ContributionDay { date, count: raw });
        }
        weeks.push(ContributionWeek { days });
    }
    weeks
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_fallback_stats_determinism() {
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

    #[test]
    fn test_fallback_grid_shape() {
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
