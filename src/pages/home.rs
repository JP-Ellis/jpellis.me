use chrono::DateTime;
use chrono::Utc;
use leptos::prelude::*;
use stylance::import_style;

use crate::components::Band;
use crate::components::Footer;
use crate::components::Masthead;
use crate::github::defaults::fallback_stats;
use crate::github::model::ActivityKind;
use crate::github::model::ActivityState;
use crate::github::model::GitHubStats;
use crate::github::server_fn::get_github_stats;

import_style!(style, "home.module.scss");

const SELECTED_WORK: [(&str, &str, &str); 3] = [
    (
        "pact-python",
        "OSS · library",
        "Python ↔ Rust contract testing",
    ),
    (
        "tikz-feynman",
        "OSS · LaTeX",
        "Feynman diagrams in TikZ; 400+ citations",
    ),
    (
        "boltzmann-solver",
        "OSS · numerics",
        "Coupled rate equations, custom quadrature",
    ),
];

const CROSS_POSTS: [(&str, &str, &str); 2] = [
    (
        "A small love letter to consumer-driven contracts",
        "pactflow.io",
        "Mar 2026",
    ),
    (
        "Why pact-python is moving to a Rust core",
        "pactflow.io",
        "Nov 2025",
    ),
];

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
fn cell_level_from_count(count: u32, max: u32) -> u8 {
    if max == 0 || count == 0 {
        return 0;
    }
    let ratio = count as f64 / max as f64;
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
fn time_ago(dt: &DateTime<Utc>) -> String {
    let secs = (Utc::now() - *dt).num_seconds().max(0);
    if secs < 3600 {
        format!("{}m", (secs / 60).max(1))
    } else if secs < 86400 {
        format!("{}h", secs / 3600)
    } else if secs < 604800 {
        format!("{}d", secs / 86400)
    } else {
        format!("{}w", secs / 604800)
    }
}

/// Renders the "Year in Code" band with live contribution grid and recent activity.
///
/// # Arguments
///
/// * `stats` - Owned [`GitHubStats`] to render.
/// * `grid` - Pre-computed grid levels from [`build_grid_levels`].
fn year_in_code_view(stats: GitHubStats, grid: Vec<Vec<u8>>) -> impl IntoView {
    let commit_count = stats.total_contributions.to_string();
    let repo_count = stats.public_repos.to_string();
    let date_range = format!(
        "{} — {}",
        stats.period_from.format("%b %Y"),
        stats.period_to.format("%b %Y")
    );

    view! {
        <Band test_id="year-in-code">
            <div class=format!("container {}", style::band_inner)>
                <div class=style::band_header>
                    <div>
                        <p class="eyebrow">"The year in code"</p>
                        <p class=style::stats_headline>
                            <em>{commit_count}</em>
                            " commits across "
                            <em>{repo_count}</em>
                            " repositories."
                        </p>
                    </div>
                    <span class=style::date_range>{date_range}</span>
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

                <div class=style::band_content>
                    <div>
                        <p class=format!("eyebrow {}", style::latest_label)>"Latest"</p>
                        {if stats.recent_activity.is_empty() {
                            view! { <p>"No recent activity."</p> }.into_any()
                        } else {
                            stats
                                .recent_activity
                                .iter()
                                .map(|item| {
                                    let kind_label = match item.kind {
                                        ActivityKind::Commit => "commit",
                                        ActivityKind::PullRequest => "PR",
                                        ActivityKind::Issue => "issue",
                                    };
                                    let state_label = item
                                        .state
                                        .as_ref()
                                        .map(|s| match s {
                                            ActivityState::Open => " · open",
                                            ActivityState::Closed => " · closed",
                                            ActivityState::Merged => " · merged",
                                        });
                                    let ago = time_ago(&item.created_at);
                                    // TODO: wrap row in <a href={item.url.clone()}> once link styling is ready
                                    view! {
                                        <div class=style::commit_row data-testid="commit-row">
                                            <span class=style::commit_repo>
                                                {item.repo.clone()}
                                                <span class=style::commit_kind>
                                                    {format!(" [{kind_label}{}]", state_label.unwrap_or(""))}
                                                </span>
                                            </span>
                                            <span class=style::commit_msg>{item.title.clone()}</span>
                                            <span class=style::commit_age>{ago}</span>
                                        </div>
                                    }
                                })
                                .collect_view()
                                .into_any()
                        }}
                    </div>
                    <div class=style::band_aside>
                        <p class=style::band_quote>
                            "\"If the code is the body of work, this is the index.\""
                        </p>
                        <p>
                            "Most of what I make is open. The grid above is the truthful "
                            "version of a résumé — public, dated, and dense in the parts "
                            "where I was paying attention."
                        </p>
                    </div>
                </div>
            </div>
        </Band>
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn cell_level_from_count_edge_cases() {
        assert_eq!(cell_level_from_count(0, 0), 0);
        assert_eq!(cell_level_from_count(0, 10), 0);
        assert_eq!(cell_level_from_count(10, 10), 4);
        assert_eq!(cell_level_from_count(1, 10), 1); // ratio 0.10 → level 1
        assert_eq!(cell_level_from_count(3, 10), 2); // ratio 0.30 → level 2
        assert_eq!(cell_level_from_count(6, 10), 3); // ratio 0.60 → level 3
        assert_eq!(cell_level_from_count(8, 10), 4); // ratio 0.80 → level 4
    }

    #[test]
    fn build_grid_levels_normalises_correctly() {
        use chrono::NaiveDate;
        use chrono::Utc;

        use crate::github::model::ContributionDay;
        use crate::github::model::ContributionWeek;
        use crate::github::model::GitHubStats;

        let stats = GitHubStats {
            fetched_at: Utc::now(),
            total_contributions: 10,
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

    #[test]
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
}

/// Almanac home page.
#[component]
pub fn HomePage() -> impl IntoView {
    let stats_res = Resource::new(|| (), |_| get_github_stats());

    view! {
        <Masthead />
        <main>
            // ── Hero ──────────────────────────────────────────────
            <section class=style::hero>
                <div class="container container--prose">
                    <p class="eyebrow">"Hello, again"</p>
                    <h1>
                        "I write " <em>"contracts"</em> " between systems for a living, in "
                        <span class=style::accent>"Rust"</span>
                        " and Python — and once spent a decade chasing where the "
                        <em>"antimatter"</em> " went."
                    </h1>
                    <p class=style::lead>
                        "Senior software engineer at " <strong>"SmartBear / PactFlow"</strong>
                        ", working on the open-source " <code>"pact-python"</code>
                        " rewrite over a Rust FFI core. Previously: data "
                        "engineering at KPMG, and a PhD in theoretical particle physics."
                    </p>
                </div>
            </section>

            // ── Year in Code ──────────────────────────────────────
            <Suspense fallback=move || {
                let fb = fallback_stats();
                let grid = build_grid_levels(&fb);
                year_in_code_view(fb, grid)
            }>
                {move || {
                    stats_res
                        .get()
                        .map(|result| {
                            let stats = result.unwrap_or_else(|_| fallback_stats());
                            let grid = build_grid_levels(&stats);
                            year_in_code_view(stats, grid)
                        })
                }}
            </Suspense>

            // ── Selected Work ─────────────────────────────────────
            <section class=style::work_section>
                <div class="container">
                    <div class="eyebrow-grid">
                        <span class="eyebrow">"Selected work"</span>
                        <div>
                            {SELECTED_WORK
                                .iter()
                                .enumerate()
                                .map(|(i, &(name, kind, summary))| {
                                    let border = if i == 0 { "rule-section" } else { "rule-list" };
                                    view! {
                                        <div
                                            class=format!("{} {}", style::work_row, border)
                                            data-testid="work-row"
                                        >
                                            <span class=style::work_name>{name}</span>
                                            <span class=style::work_kind>{kind}</span>
                                            <span class=style::work_summary>{summary}</span>
                                            <a href="#" class=style::work_link>
                                                "↗ case study"
                                            </a>
                                        </div>
                                    }
                                })
                                .collect_view()} <a href="/work" class=style::work_all_link>
                                "↗ All work"
                            </a>
                        </div>
                    </div>
                </div>
            </section>

            // ── Elsewhere ─────────────────────────────────────────
            <section class=style::elsewhere_section>
                <div class="container">
                    <div class="eyebrow-grid">
                        <span class="eyebrow">"Elsewhere"</span>
                        <div>
                            <p class=style::elsewhere_intro>
                                "I don't keep a blog here. The few pieces I do write live on the "
                                <a href="https://pactflow.io/blog">"Pact"</a>
                                " blog — they're cross-posted below."
                            </p>
                            {CROSS_POSTS
                                .iter()
                                .map(|&(title, source, date)| {
                                    view! {
                                        <div
                                            class=format!("{} rule-list", style::cross_post_row)
                                            data-testid="cross-post-row"
                                        >
                                            <div>
                                                <span class=style::cross_post_title>{title}</span>
                                                <a href="#" class=style::cross_post_source>
                                                    "↗ "
                                                    {source}
                                                </a>
                                            </div>
                                            <span class=style::cross_post_date>{date}</span>
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>
                </div>
            </section>
        </main>
        <Footer />
    }
}
