use chrono::DateTime;
use chrono::Utc;
use leptos::prelude::*;
use leptos_meta::Meta;
use leptos_meta::Script;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use stylance::import_style;

use crate::components::Footer;
use crate::components::Masthead;
use crate::integration::ProjectsStats;
use crate::integration::get_projects_stats;
use crate::integration::github::projects::model::CommitInfo;
use crate::integration::github::projects::model::ReleaseInfo;
use crate::integration::github::projects::model::RepoStats;
use crate::pages::not_found::NotFoundPage;
use crate::pages::projects::ActivityConfig;
use crate::pages::projects::find_project_page;
use crate::pages::projects::format_stars;

import_style!(style, "detail.module.scss");

// ── Prism.js (WASM only) ──────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
mod prism {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen(
        inline_js = "export function highlight_all() { if (typeof Prism !== 'undefined') { Prism.highlightAll(); } }"
    )]
    extern "C" {
        pub fn highlight_all();
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Formats an ISO 8601 timestamp as a human-readable relative string.
///
/// Examples: `"3d ago"`, `"2w ago"`, `"4mo ago"`, `"1y ago"`.
/// Falls back to the raw string if parsing fails.
fn format_relative_date(iso: &str) -> String {
    let Ok(dt) = iso.parse::<DateTime<Utc>>() else {
        return iso.to_string();
    };
    let secs = (Utc::now() - dt).num_seconds().max(0);
    if secs < 3600 {
        let mins = secs / 60;
        return if mins <= 1 {
            "just now".to_string()
        } else {
            format!("{mins}m ago")
        };
    }
    let hours = secs / 3600;
    if hours < 24 {
        return format!("{hours}h ago");
    }
    let days = secs / 86400;
    if days < 7 {
        return format!("{days}d ago");
    }
    let weeks = days / 7;
    if weeks < 5 {
        return format!("{weeks}w ago");
    }
    let months = days / 30;
    if months < 12 {
        return format!("{months}mo ago");
    }
    format!("{}y ago", days / 365)
}

/// Formats an ISO 8601 date string as `"14 Jan 2025"`.
fn format_short_date(iso: &str) -> String {
    let date_part = iso.split('T').next().unwrap_or(iso);
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.len() < 3 {
        return iso.to_string();
    }
    let month = match parts[1] {
        "01" => "Jan",
        "02" => "Feb",
        "03" => "Mar",
        "04" => "Apr",
        "05" => "May",
        "06" => "Jun",
        "07" => "Jul",
        "08" => "Aug",
        "09" => "Sep",
        "10" => "Oct",
        "11" => "Nov",
        "12" => "Dec",
        _ => parts[1],
    };
    let day = parts[2].trim_start_matches('0');
    format!(
        "{} {} {}",
        if day.is_empty() { "1" } else { day },
        month,
        parts[0]
    )
}

// ── ProjectDetailPage ─────────────────────────────────────────────────────────

#[component]
pub fn ProjectDetailPage() -> impl IntoView {
    let params = use_params_map();
    let page = move || params.with(|p| p.get("slug").as_deref().and_then(find_project_page));

    let projects_res = LocalResource::new(get_projects_stats);

    #[cfg(target_arch = "wasm32")]
    Effect::new(|_| {
        prism::highlight_all();
    });

    view! {
        {move || match page() {
            None => {
                view! {
                    <NotFoundPage
                        heading="Project not found."
                        back_href="/projects"
                        back_label="← All projects"
                    />
                }
                    .into_any()
            }
            Some(p) => {
                view! {
                    <Title text=p.title />
                    <Meta name="description" content=p.tagline />
                    <Script
                        src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/prism.min.js"
                        defer=""
                    />
                    <Script
                        src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-latex.min.js"
                        defer=""
                    />
                    <Script
                        src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-rust.min.js"
                        defer=""
                    />
                    <Script
                        src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-python.min.js"
                        defer=""
                    />
                    <Masthead />
                    <main>

                        // ── Hero ──────────────────────────────────────────
                        <section class=style::hero>
                            <div class="container">
                                <p class="eyebrow">"Projects"</p>
                                <h1>{p.title}</h1>
                                <p class=style::tagline>{p.tagline}</p>
                                <a
                                    href=format!("https://github.com/{}", p.github)
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class=style::github_link
                                >
                                    "View on GitHub ↗"
                                </a>
                            </div>
                        </section>

                        // ── Stats + Activity panel ─────────────────────────
                        <Suspense fallback=move || {
                            view! { <StatsPanel repo=None activity=p.activity /> }
                        }>
                            {move || {
                                let stats: Option<ProjectsStats> = projects_res
                                    .get()
                                    .and_then(|r| r.ok());
                                let repo = stats
                                    .as_ref()
                                    .and_then(|s| {
                                        s.repos.iter().find(|r| r.slug == p.github).cloned()
                                    });
                                view! { <StatsPanel repo=repo activity=p.activity /> }
                            }}
                        </Suspense>

                        // ── Prose content ──────────────────────────────────
                        <section class=style::prose_section>
                            <div class="container container--prose">
                                <div class="prose" inner_html=p.body_html />
                            </div>
                        </section>

                    </main>
                    <Footer />
                }
                    .into_any()
            }
        }}
    }
}

// ── StatsPanel ────────────────────────────────────────────────────────────────

#[component]
fn StatsPanel(repo: Option<RepoStats>, activity: ActivityConfig) -> impl IntoView {
    let show_commits =
        activity.recent_commits && repo.as_ref().is_some_and(|r| !r.recent_commits.is_empty());

    view! {
        <section class=style::stats_section>
            <div class="container">
                <div class=style::stats_grid>

                    // Left column: stat chips (all screen widths)
                    <div class=style::chips>
                        <StatChip
                            icon="⭐"
                            label="stars"
                            value=repo
                                .as_ref()
                                .map(|r| format_stars(r.stars))
                                .map(|v| view! { {v} }.into_any())
                                .unwrap_or_else(|| {
                                    view! { <span class=style::chip_skeleton aria-hidden="true" /> }
                                        .into_any()
                                })
                            link=None
                        />
                        <StatChip
                            icon="🍴"
                            label="forks"
                            value=repo
                                .as_ref()
                                .map(|r| r.forks.to_string())
                                .map(|v| view! { {v} }.into_any())
                                .unwrap_or_else(|| {
                                    view! { <span class=style::chip_skeleton aria-hidden="true" /> }
                                        .into_any()
                                })
                            link=None
                        />
                        <StatChip
                            icon="👁"
                            label="watchers"
                            value=repo
                                .as_ref()
                                .map(|r| r.watchers.to_string())
                                .map(|v| view! { {v} }.into_any())
                                .unwrap_or_else(|| {
                                    view! { <span class=style::chip_skeleton aria-hidden="true" /> }
                                        .into_any()
                                })
                            link=None
                        />
                        <StatChip
                            icon="❗"
                            label="open issues"
                            value=repo
                                .as_ref()
                                .map(|r| r.open_issues.to_string())
                                .map(|v| view! { {v} }.into_any())
                                .unwrap_or_else(|| {
                                    view! { <span class=style::chip_skeleton aria-hidden="true" /> }
                                        .into_any()
                                })
                            link=None
                        />
                        {activity
                            .release
                            .then(|| {
                                let release: Option<ReleaseInfo> = repo
                                    .as_ref()
                                    .and_then(|r| r.latest_release.clone());
                                release
                                    .map(|rel| {
                                        let label = format!(
                                            "{} · {}",
                                            rel.tag,
                                            format_short_date(&rel.date),
                                        );
                                        let url = rel.url.clone();
                                        view! {
                                            <StatChip
                                                icon="🚀"
                                                label="latest release"
                                                value=view! { {label} }.into_any()
                                                link=Some(url)
                                            />
                                        }
                                    })
                            })}
                        {activity
                            .open_prs
                            .then(|| {
                                repo.as_ref()
                                    .map(|r| {
                                        let count = r.open_prs;
                                        let label = if count == 1 {
                                            "1 open PR".to_string()
                                        } else {
                                            format!("{count} open PRs")
                                        };
                                        view! {
                                            <StatChip
                                                icon="🔀"
                                                label="pull requests"
                                                value=view! { {label} }.into_any()
                                                link=None
                                            />
                                        }
                                    })
                            })}
                    </div>

                    // Right column: recent commits (wide screens only, via CSS)
                    {show_commits
                        .then(|| {
                            let commits = repo
                                .as_ref()
                                .map(|r| r.recent_commits.clone())
                                .unwrap_or_default();
                            view! { <CommitsList commits=commits /> }
                        })}

                </div>
            </div>
        </section>
    }
}

// ── StatChip ──────────────────────────────────────────────────────────────────

#[component]
fn StatChip(
    icon: &'static str,
    label: &'static str,
    value: AnyView,
    link: Option<String>,
) -> impl IntoView {
    let value_node: AnyView = match link {
        Some(url) => view! {
            <a href=url target="_blank" rel="noopener noreferrer" class=style::chip_value_link>
                {value}
            </a>
        }
        .into_any(),
        None => value,
    };

    view! {
        <dl class=style::chip>
            <dt class=style::chip_label>{label}</dt>
            <dd class=style::chip_icon_value>
                <span aria-hidden="true">{icon}</span>
                {value_node}
            </dd>
        </dl>
    }
}

// ── CommitsList ───────────────────────────────────────────────────────────────

#[component]
fn CommitsList(commits: Vec<CommitInfo>) -> impl IntoView {
    view! {
        <div class=style::commits>
            <p class=style::commits_heading>"Recent commits"</p>
            <ul class=style::commit_list>
                {commits
                    .into_iter()
                    .map(|c| {
                        let relative = format_relative_date(&c.date);
                        view! {
                            <li class=style::commit_item>
                                <a
                                    href=c.url
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class=style::commit_sha
                                    title=c.message.clone()
                                >
                                    {c.sha}
                                </a>
                                <span class=style::commit_message>{c.message}</span>
                                <span class=style::commit_meta>{c.author} " · " {relative}</span>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}
