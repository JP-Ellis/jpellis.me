use leptos::prelude::*;
use stylance::import_style;

use crate::components::Footer;
use crate::components::Masthead;
use crate::components::YearInCode;

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

/// Almanac home page.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Masthead />
        <main>
            // ── Hero ──────────────────────────────────────────────
            <section class=style::hero>
                <div class="container">
                    <p class="eyebrow">"Hello, again"</p>
                    <h1>
                        "I codify " <em>"contracts"</em> " between systems for a living, in "
                        <span class=style::accent>"Rust"</span> " and "
                        <span class=style::accent_python>"Python"</span> "; "
                        " and once spent a decade chasing where the "
                        <em class=style::antimatter>"antimatter"</em> " went."
                    </h1>
                    <p class=style::lead>
                        "Senior software engineer at " <strong>"PactFlow (SmartBear)"</strong>
                        ". Open source contributor to " <code>"pact-python"</code> ", "
                        <code>"tikz-feynman"</code> ", " <code>"rust-skiplist"</code> ", and more. "
                    </p>
                    <p class=style::lead>
                        "Previously: data and cloud engineering at KPMG, "
                        "and a PhD in theoretical particle physics."
                    </p>
                </div>
            </section>

            // ── Year in Code ──────────────────────────────────────
            <YearInCode />

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
