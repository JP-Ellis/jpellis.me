use leptos::prelude::*;
use stylance::import_style;

use crate::components::Band;
use crate::components::Footer;
use crate::components::Masthead;

import_style!(style, "home.module.scss");

// TODO: replace with GitHub API server function
const COMMIT_COUNT: &str = "1,247";
const REPO_COUNT: &str = "14";
const DATE_RANGE: &str = "May 2025 — Apr 2026";

const LATEST_COMMITS: [(&str, &str, &str); 5] = [
    ("pact-python", "feat(ffi): bind verifier results", "4h"),
    ("pact-python", "fix: v4 matchers on dict roots", "1d"),
    ("boltzmann-solver", "perf(quad): cache leaf weights", "1w"),
    ("dotfiles", "feat(helix): soft-wrap default", "1w"),
    ("tikz-feynman", "docs: 2025 dependency note", "3w"),
];

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

fn commit_grid() -> Vec<Vec<f64>> {
    fn lcg(s: &mut u64) -> f64 {
        *s = s.wrapping_mul(9301).wrapping_add(49297) % 233280;
        *s as f64 / 233280.0
    }

    let mut s: u64 = 11;
    let mut grid = Vec::with_capacity(53);

    for w in 0..53_usize {
        let mut col = Vec::with_capacity(7);
        for _ in 0..7 {
            let v = lcg(&mut s).powf(1.6);
            let r2 = lcg(&mut s);
            let burst = if w > 32 && w < 44 && r2 > 0.4 {
                0.3_f64
            } else {
                0.0_f64
            };
            col.push((v + burst).min(1.0));
        }
        grid.push(col);
    }
    grid
}

fn cell_level(v: f64) -> u8 {
    if v < 0.05 {
        0
    } else if v < 0.25 {
        1
    } else if v < 0.5 {
        2
    } else if v < 0.75 {
        3
    } else {
        4
    }
}

/// Almanac home page.
#[component]
pub fn HomePage() -> impl IntoView {
    let grid = commit_grid();

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
            <Band test_id="year-in-code">
                <div class=format!("container {}", style::band_inner)>
                    <div class=style::band_header>
                        <div>
                            <p class="eyebrow">"The year in code"</p>
                            <p class=style::stats_headline>
                                <em>{COMMIT_COUNT}</em>
                                " commits across "
                                <em>{REPO_COUNT}</em>
                                " repositories."
                            </p>
                        </div>
                        <span class=style::date_range>{DATE_RANGE}</span>
                    </div>

                    <div class=style::commit_grid>
                        {grid
                            .iter()
                            .map(|col| {
                                view! {
                                    <div class=style::commit_col data-testid="commit-col">
                                        {col
                                            .iter()
                                            .map(|&v| {
                                                view! {
                                                    <span
                                                        class=style::commit_cell
                                                        data-commit-level=cell_level(v).to_string()
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
                            {LATEST_COMMITS
                                .iter()
                                .map(|&(repo, msg, ago)| {
                                    view! {
                                        <div class=style::commit_row data-testid="commit-row">
                                            <span class=style::commit_repo>{repo}</span>
                                            <span class=style::commit_msg>{msg}</span>
                                            <span class=style::commit_age>{ago}</span>
                                        </div>
                                    }
                                })
                                .collect_view()}
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
