use leptos::prelude::*;
use stylance::import_style;

use crate::blog::format_date;
use crate::blog::source_domain;
use crate::components::Footer;
use crate::components::Masthead;
use crate::components::YearInCode;
use crate::pages::projects::PROJECTS;
use crate::pages::projects::ProjectLink;

import_style!(style, "home.module.scss");

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
            <section class=style::projects_section>
                <div class="container">
                    <div class="eyebrow-grid">
                        <span class="eyebrow">"Selected work"</span>
                        <div>
                            {PROJECTS
                                .iter()
                                .take(3)
                                .enumerate()
                                .map(|(i, entry)| {
                                    let border = if i == 0 { "rule-section" } else { "rule-list" };
                                    let link_view = match entry.link {
                                        Some(ProjectLink::GitHub(slug)) => {
                                            Some(
                                                view! {
                                                    <a
                                                        href=format!("https://github.com/{slug}")
                                                        class=style::projects_link
                                                        target="_blank"
                                                        rel="noopener noreferrer"
                                                    >
                                                        "↗ github"
                                                    </a>
                                                }
                                                    .into_any(),
                                            )
                                        }
                                        Some(ProjectLink::External(url)) => {
                                            Some(
                                                view! {
                                                    <a
                                                        href=url
                                                        class=style::projects_link
                                                        target="_blank"
                                                        rel="noopener noreferrer"
                                                    >
                                                        "↗ site"
                                                    </a>
                                                }
                                                    .into_any(),
                                            )
                                        }
                                        None => None,
                                    };
                                    view! {
                                        <div
                                            class=format!("{} {}", style::projects_row, border)
                                            data-testid="projects-row"
                                        >
                                            <span class=style::projects_name>{entry.name}</span>
                                            <span class=style::projects_kind>{entry.kind}</span>
                                            <span class=style::projects_summary>{entry.summary}</span>
                                            {link_view}
                                        </div>
                                    }
                                })
                                .collect_view()} <a href="/projects" class=style::projects_all_link>
                                "↗ All projects"
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
                            {crate::blog::POSTS
                                .iter()
                                .take(3)
                                .enumerate()
                                .map(|(i, post)| {
                                    let border = if i == 0 { "rule-section" } else { "rule-list" };
                                    let href = format!("/blog/{}", post.slug);
                                    let source_domain = post
                                        .source
                                        .and_then(|s| {
                                            source_domain(s).map(|d| (d.to_string(), s))
                                        });
                                    view! {
                                        <div
                                            class=format!("{} {border}", style::cross_post_row)
                                            data-testid="cross-post-row"
                                        >
                                            <div>
                                                <a href=href class=style::cross_post_title>
                                                    {post.title}
                                                </a>
                                                {source_domain
                                                    .map(|(domain, src)| {
                                                        view! {
                                                            <a
                                                                href=src
                                                                class=style::cross_post_source
                                                                target="_blank"
                                                                rel="noopener noreferrer"
                                                            >
                                                                "↗ "
                                                                {domain}
                                                            </a>
                                                        }
                                                    })}
                                            </div>
                                            <span class=style::cross_post_date>
                                                {format_date(post.date)}
                                            </span>
                                        </div>
                                    }
                                })
                                .collect_view()} <a href="/blog" class=style::projects_all_link>
                                "↗ All posts"
                            </a>
                        </div>
                    </div>
                </div>
            </section>
        </main>
        <Footer />
    }
}
