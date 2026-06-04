//! GitHub contribution timeline component.

use std::collections::HashSet;

use chrono::DateTime;
use chrono::Utc;
use leptos::prelude::*;
use stylance::import_style;

use crate::components::Band;
use crate::integration::ActivityItem;
use crate::integration::ActivityKind;
use crate::integration::ActivityState;
use crate::integration::GitHubStats;
use crate::integration::get_github_stats;

import_style!(style, "year_in_code.module.scss");

/// Returns the contribution level (0–4) for a given count relative to the maximum.
///
/// # Arguments
///
/// * `count` - The contribution count for the day.
/// * `max` - The maximum contribution count across all days.
///
/// # Returns
///
/// A level from 0 (no contributions) to 4 (highest activity).
#[expect(
    clippy::float_arithmetic,
    reason = "float division for level computation is intentional"
)]
fn cell_level_from_count(count: u32, max: u32) -> u8 {
    if max == 0 || count == 0 {
        return 0;
    }
    let ratio = f64::from(count) / f64::from(max);
    if ratio < 0.25 {
        1
    } else if ratio < 0.50 {
        2
    } else if ratio < 0.75 {
        3
    } else {
        4
    }
}

/// Converts contribution week data from [`GitHubStats`] into a 2-D grid of levels.
///
/// Each level is in the range 0–4, normalised relative to the maximum daily count
/// across the entire period.
///
/// # Arguments
///
/// * `stats` - The GitHub stats containing contribution weeks.
///
/// # Returns
///
/// A `Vec<Vec<u8>>` where the outer vec is weeks and the inner vec is days (levels).
fn build_grid_levels(stats: &GitHubStats) -> Vec<Vec<u8>> {
    let max = stats
        .contribution_weeks
        .iter()
        .flat_map(|w| w.days.iter())
        .map(|d| d.count)
        .max()
        .unwrap_or(1);
    stats
        .contribution_weeks
        .iter()
        .map(|week| {
            week.days
                .iter()
                .map(|day| cell_level_from_count(day.count, max))
                .collect()
        })
        .collect()
}

/// Returns a human-readable relative time string for a [`DateTime<Utc>`].
///
/// # Arguments
///
/// * `dt` - The datetime to format as a relative string.
///
/// # Returns
///
/// A string like `"1h"`, `"3d"`, or `"2w"`.
#[expect(
    clippy::arithmetic_side_effects,
    clippy::integer_division,
    clippy::integer_division_remainder_used,
    reason = "divisions are by non-zero constants; chrono subtraction is saturating; truncating division for display formatting is intentional"
)]
fn time_ago(dt: &DateTime<Utc>) -> String {
    let secs = (Utc::now() - *dt).num_seconds().max(0);
    if secs < 3_600 {
        format!("{}m", (secs / 60).max(1))
    } else if secs < 86_400 {
        format!("{}h", secs / 3_600)
    } else if secs < 604_800 {
        format!("{}d", secs / 86_400)
    } else {
        format!("{}w", secs / 604_800)
    }
}

/// Removes commits whose title matches any PR's title in the same list.
///
/// When a single-commit branch is merged as a PR, the commit message and PR
/// title are identical. This function keeps the PR and discards the redundant
/// commit so the UI doesn't show the same work twice.
///
/// # Arguments
///
/// * `activity` - The full list of recent activity items.
///
/// # Returns
///
/// A new `Vec` with duplicate commits removed; all non-commit items are kept.
fn dedup_commits_against_prs(activity: Vec<ActivityItem>) -> Vec<ActivityItem> {
    let pr_titles: HashSet<String> = activity
        .iter()
        .filter(|i| matches!(i.kind, ActivityKind::PullRequest))
        .map(|i| i.title.clone())
        .collect();

    activity
        .into_iter()
        .filter(|i| !(matches!(i.kind, ActivityKind::Commit) && pr_titles.contains(&i.title)))
        .collect()
}

/// Renders a skeleton placeholder for the year-in-code band while data loads.
fn year_in_code_skeleton() -> impl IntoView {
    view! {
        <Band test_id="year-in-code">
            <div class=format!("container {}", style::band_inner)>
                <div class=style::band_header>
                    <div>
                        <p class="eyebrow">"The year in code"</p>
                        <p class=style::stats_headline>
                            <span class=style::skeleton_line style="width:18ch" />
                        </p>
                    </div>
                </div>
                <div class=style::band_intro>
                    <span
                        class=style::skeleton_line
                        style="width:55ch;display:block;margin-bottom:var(--space-2)"
                    />
                    <span class=style::skeleton_line style="width:38ch;display:block" />
                </div>
                <div class=style::commit_grid>
                    {std::iter::repeat_with(|| {
                            view! {
                                <div class=style::commit_col data-testid="commit-col">
                                    {std::iter::repeat_with(|| {
                                            view! {
                                                <span class=style::commit_cell data-commit-level="0" />
                                            }
                                        })
                                        .take(7)
                                        .collect_view()}
                                </div>
                            }
                        })
                        .take(53)
                        .collect_view()}
                </div>
                <div class=style::band_latest>
                    <div>
                        <p class=format!("eyebrow {}", style::latest_label)>"Latest commits"</p>
                        {std::iter::repeat_with(|| {
                                view! {
                                    <div class=style::commit_row data-testid="commit-row">
                                        <span class=style::skeleton_line style="width:80%" />
                                        <span class=style::skeleton_line style="width:70%" />
                                        <span class=style::skeleton_line style="width:28px" />
                                    </div>
                                }
                            })
                            .take(6)
                            .collect_view()}
                    </div>
                    <div>
                        <p class=format!(
                            "eyebrow {}",
                            style::latest_label,
                        )>"Latest issues & PRs"</p>
                        {std::iter::repeat_with(|| {
                                view! {
                                    <div class=style::activity_row data-testid="activity-row">
                                        <div class=style::activity_repo_wrap>
                                            <span class=style::skeleton_line style="width:90%" />
                                            <span class=style::skeleton_line style="width:55%" />
                                        </div>
                                        <span class=style::skeleton_line style="width:75%" />
                                        <span class=style::skeleton_line style="width:28px" />
                                    </div>
                                }
                            })
                            .take(4)
                            .collect_view()}
                    </div>
                </div>
            </div>
        </Band>
    }
}

/// Renders the full year-in-code band with live data.
#[expect(clippy::too_many_lines, reason = "Leptos component template")]
fn year_in_code_inner(stats: GitHubStats, grid: Vec<Vec<u8>>) -> impl IntoView {
    let commit_count = stats.commit_contributions.to_string();
    let pr_count = stats.pr_contributions.to_string();
    let issue_count = stats.issue_contributions.to_string();
    let repo_count = stats.public_repos.to_string();
    let date_range = format!(
        "{} — {}",
        stats.period_from.format("%b %Y"),
        stats.period_to.format("%b %Y")
    );

    let (mut commits, issues_prs): (Vec<ActivityItem>, Vec<ActivityItem>) =
        dedup_commits_against_prs(stats.recent_activity)
            .into_iter()
            .partition(|i| matches!(i.kind, ActivityKind::Commit));
    commits.truncate(6);

    view! {
        <Band test_id="year-in-code">
            <div class=format!("container {}", style::band_inner)>
                <div class=style::band_header>
                    <div>
                        <p class="eyebrow">"The year in code"</p>
                        <p class=style::stats_headline>
                            <em>{commit_count}</em>
                            " commits, "
                            <em>{pr_count}</em>
                            " PRs, and "
                            <em>{issue_count}</em>
                            " issues across "
                            <em>{repo_count}</em>
                            " repositories."
                        </p>
                    </div>
                    <span class=style::date_range>{date_range}</span>
                </div>

                <div class=style::band_intro>
                    "Most of what I make is open. The grid below is the truthful "
                    "version of a résumé — public, dated, and dense in the parts "
                    "where I was paying attention."
                </div>

                <div class=style::commit_grid>
                    {grid
                        .iter()
                        .map(|col| {
                            view! {
                                <div class=style::commit_col data-testid="commit-col">
                                    {col
                                        .iter()
                                        .map(|&level| {
                                            view! {
                                                <span
                                                    class=style::commit_cell
                                                    data-commit-level=level.to_string()
                                                />
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            }
                        })
                        .collect_view()}
                </div>

                <div class=style::band_latest>
                    <div>
                        <p class=format!("eyebrow {}", style::latest_label)>"Latest commits"</p>
                        {if commits.is_empty() {
                            view! { <p class=style::no_activity>"No recent commits."</p> }
                                .into_any()
                        } else {
                            commits
                                .iter()
                                .map(|item| {
                                    let ago = time_ago(&item.created_at);
                                    view! {
                                        <div class=style::commit_row data-testid="commit-row">
                                            <span class=style::commit_repo>{item.repo.clone()}</span>
                                            <span class=style::commit_msg>{item.title.clone()}</span>
                                            <span class=style::commit_age>{ago}</span>
                                        </div>
                                    }
                                })
                                .collect_view()
                                .into_any()
                        }}
                    </div>
                    <div>
                        <p class=format!(
                            "eyebrow {}",
                            style::latest_label,
                        )>"Latest issues & PRs"</p>
                        {if issues_prs.is_empty() {
                            view! { <p class=style::no_activity>"No recent issues or PRs."</p> }
                                .into_any()
                        } else {
                            issues_prs
                                .iter()
                                .map(|item| {
                                    let state_label = item
                                        .state
                                        .as_ref()
                                        .map(|s| match s {
                                            ActivityState::Open => "open",
                                            ActivityState::Closed => "closed",
                                            ActivityState::Merged => "merged",
                                        });
                                    let kind_label = match item.kind {
                                        ActivityKind::PullRequest => "PR",
                                        ActivityKind::Issue => "issue",
                                        ActivityKind::Commit => "commit",
                                    };
                                    let badge = if let Some(state) = state_label {
                                        format!("[{kind_label} · {state}]")
                                    } else {
                                        format!("[{kind_label}]")
                                    };
                                    let ago = time_ago(&item.created_at);
                                    view! {
                                        <div class=style::activity_row data-testid="activity-row">
                                            <div class=style::activity_repo_wrap>
                                                <span class=style::commit_repo>{item.repo.clone()}</span>
                                                <span class=style::activity_kind>{badge}</span>
                                            </div>
                                            <span class=style::commit_msg>{item.title.clone()}</span>
                                            <span class=style::commit_age>{ago}</span>
                                        </div>
                                    }
                                })
                                .collect_view()
                                .into_any()
                        }}
                    </div>
                </div>
            </div>
        </Band>
    }
}

/// Renders the "Year in Code" band with live GitHub contribution data.
///
/// Fetches stats via [`get_github_stats`] and shows [`fallback_stats`] while
/// loading or on error.
#[component]
pub fn YearInCode() -> impl IntoView {
    let stats_res = LocalResource::new(get_github_stats);

    view! {
        <Suspense fallback=move || year_in_code_skeleton()>
            {move || {
                stats_res
                    .get()
                    .map(|result| match result {
                        Ok(stats) => {
                            let grid = build_grid_levels(&stats);
                            year_in_code_inner(stats, grid).into_any()
                        }
                        Err(_) => year_in_code_skeleton().into_any(),
                    })
            }}
        </Suspense>
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use pretty_assertions::assert_eq;

    use super::*;

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn cell_level_from_count_edge_cases() {
        assert_eq!(cell_level_from_count(0, 0), 0);
        assert_eq!(cell_level_from_count(0, 10), 0);
        assert_eq!(cell_level_from_count(10, 10), 4);
        assert_eq!(cell_level_from_count(1, 10), 1); // ratio 0.10 → level 1
        assert_eq!(cell_level_from_count(3, 10), 2); // ratio 0.30 → level 2
        assert_eq!(cell_level_from_count(6, 10), 3); // ratio 0.60 → level 3
        assert_eq!(cell_level_from_count(8, 10), 4); // ratio 0.80 → level 4
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[expect(
        clippy::indexing_slicing,
        reason = "test assertions on known-size grid; expect on infallible date construction"
    )]
    fn build_grid_levels_normalises_correctly() {
        use chrono::NaiveDate;
        use chrono::Utc;

        use crate::integration::github::stats::model::ContributionDay;
        use crate::integration::github::stats::model::ContributionWeek;
        use crate::integration::github::stats::model::GitHubStats;

        let stats = GitHubStats {
            fetched_at: Utc::now(),
            total_contributions: 10,
            commit_contributions: 8,
            pr_contributions: 1,
            issue_contributions: 1,
            public_repos: 1,
            period_from: NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
            period_to: NaiveDate::from_ymd_opt(2025, 1, 7).expect("valid date"),
            contribution_weeks: vec![ContributionWeek {
                days: vec![
                    ContributionDay {
                        date: NaiveDate::from_ymd_opt(2025, 1, 1).expect("valid date"),
                        count: 0,
                    },
                    ContributionDay {
                        date: NaiveDate::from_ymd_opt(2025, 1, 2).expect("valid date"),
                        count: 10,
                    },
                ],
            }],
            recent_activity: vec![],
        };
        let grid = build_grid_levels(&stats);
        assert_eq!(grid.len(), 1);
        assert_eq!(grid[0][0], 0); // count 0 → level 0
        assert_eq!(grid[0][1], 4); // count 10/10 = max → level 4
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[cfg_attr(
        target_arch = "wasm32",
        expect(
            clippy::arithmetic_side_effects,
            reason = "DateTime - Duration in test fixtures won't overflow"
        )
    )]
    fn time_ago_boundaries() {
        let now = Utc::now();
        // 30 seconds ago → "1m" (floor to 1 minute minimum)
        assert_eq!(time_ago(&(now - Duration::seconds(30))), "1m");
        // 90 seconds ago → "1m"
        assert_eq!(time_ago(&(now - Duration::seconds(90))), "1m");
        // 30 minutes ago → "30m"
        assert_eq!(time_ago(&(now - Duration::minutes(30))), "30m");
        // 2 hours ago → "2h"
        assert_eq!(time_ago(&(now - Duration::hours(2))), "2h");
        // 3 days ago → "3d"
        assert_eq!(time_ago(&(now - Duration::days(3))), "3d");
        // 2 weeks ago → "2w"
        assert_eq!(time_ago(&(now - Duration::weeks(2))), "2w");
    }

    fn make_item(kind: ActivityKind, title: &str) -> ActivityItem {
        ActivityItem {
            kind,
            repo: "owner/repo".to_owned(),
            title: title.to_owned(),
            url: "https://example.com".to_owned(),
            state: None,
            created_at: Utc::now(),
        }
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn dedup_removes_commit_when_pr_has_same_title() {
        let activity = vec![
            make_item(ActivityKind::Commit, "feat: add thing"),
            make_item(ActivityKind::PullRequest, "feat: add thing"),
            make_item(ActivityKind::Commit, "fix: unrelated"),
        ];
        let result = dedup_commits_against_prs(activity);
        assert_eq!(result.len(), 2);
        assert!(
            result.iter().any(
                |i| matches!(i.kind, ActivityKind::PullRequest) && i.title == "feat: add thing"
            )
        );
        assert!(
            result
                .iter()
                .any(|i| matches!(i.kind, ActivityKind::Commit) && i.title == "fix: unrelated")
        );
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn dedup_keeps_commit_when_no_matching_pr() {
        let activity = vec![
            make_item(ActivityKind::Commit, "feat: standalone commit"),
            make_item(ActivityKind::PullRequest, "feat: different title"),
        ];
        let result = dedup_commits_against_prs(activity);
        assert_eq!(result.len(), 2);
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn dedup_keeps_issue_even_when_title_matches_commit() {
        let activity = vec![
            make_item(ActivityKind::Commit, "fix: bug"),
            make_item(ActivityKind::Issue, "fix: bug"),
        ];
        let result = dedup_commits_against_prs(activity);
        // Only PRs trigger deduplication; issues do not.
        assert_eq!(result.len(), 2);
    }
}
