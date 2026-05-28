pub mod post;

use leptos::prelude::*;
use leptos_router::NavigateOptions;
use leptos_router::hooks::query_signal_with_options;
use stylance::import_style;

use crate::blog::POSTS;
use crate::blog::format_date;
use crate::blog::source_domain;
use crate::components::Band;
use crate::components::Footer;
use crate::components::Masthead;

import_style!(style, "blog.module.scss");

fn filter_url(tag: Option<&str>, year: Option<&str>) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(t) = tag {
        parts.push(format!("tag={t}"));
    }
    if let Some(y) = year {
        parts.push(format!("year={y}"));
    }
    if parts.is_empty() {
        "/blog".to_string()
    } else {
        format!("/blog?{}", parts.join("&"))
    }
}

#[component]
pub fn BlogListPage() -> impl IntoView {
    let filter_nav = NavigateOptions {
        replace: true,
        scroll: false,
        ..Default::default()
    };
    let (active_tag, set_active_tag) =
        query_signal_with_options::<String>("tag", filter_nav.clone());
    let (active_year, set_active_year) = query_signal_with_options::<String>("year", filter_nav);

    let filtered = move || {
        let tag = active_tag.get();
        let year = active_year.get();
        POSTS
            .iter()
            .filter(|p| {
                let tag_ok = tag.as_ref().is_none_or(|t| p.tags.contains(&t.as_str()));
                let year_ok = year.as_ref().is_none_or(|y| p.date.starts_with(y.as_str()));
                tag_ok && year_ok
            })
            .collect::<Vec<_>>()
    };

    let all_tags = {
        let mut tags: Vec<&'static str> =
            POSTS.iter().flat_map(|p| p.tags.iter().copied()).collect();
        tags.sort_unstable();
        tags.dedup();
        tags
    };

    let all_years = {
        let mut years: Vec<&'static str> = POSTS.iter().filter_map(|p| p.date.get(..4)).collect();
        years.sort_unstable();
        years.dedup();
        years.reverse();
        years
    };

    view! {
        <Masthead />
        <main>
            <section class=style::hero>
                <div class="container">
                    <p class="eyebrow">"Blog"</p>
                    <h1>"Writing"</h1>
                </div>
            </section>

            <Band>
                <div class="container">
                    <div class=style::filter_band>
                        <div class=format!("eyebrow-grid {}", style::filter_row)>
                            <span class="eyebrow eyebrow--muted">"Tag"</span>
                            <div class=style::filter_tags>
                                {all_tags
                                    .iter()
                                    .map(|&tag| {
                                        let tag_href = move || {
                                            let ct = active_tag.get();
                                            let cy = active_year.get();
                                            let new_tag = if ct.as_deref() == Some(tag) {
                                                None
                                            } else {
                                                Some(tag)
                                            };
                                            filter_url(new_tag, cy.as_deref())
                                        };
                                        let is_active = move || {
                                            active_tag.get().as_deref() == Some(tag)
                                        };
                                        view! {
                                            <a
                                                href=tag_href
                                                class=move || {
                                                    if is_active() {
                                                        "tag tag--pill tag--accent"
                                                    } else {
                                                        "tag tag--pill"
                                                    }
                                                }
                                                role="button"
                                                aria-pressed=move || is_active().to_string()
                                                on:click=move |ev| {
                                                    ev.prevent_default();
                                                    let new_val = if active_tag.get().as_deref() == Some(tag) {
                                                        None
                                                    } else {
                                                        Some(tag.to_string())
                                                    };
                                                    set_active_tag.set(new_val);
                                                }
                                            >
                                                {tag}
                                            </a>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        </div>
                        <hr class="rule-list" />
                        <div class=format!("eyebrow-grid {}", style::filter_row)>
                            <span class="eyebrow eyebrow--muted">"Year"</span>
                            <div class=style::filter_tags>
                                {all_years
                                    .iter()
                                    .map(|&year| {
                                        let year_href = move || {
                                            let ct = active_tag.get();
                                            let cy = active_year.get();
                                            let new_year = if cy.as_deref() == Some(year) {
                                                None
                                            } else {
                                                Some(year)
                                            };
                                            filter_url(ct.as_deref(), new_year)
                                        };
                                        let is_active = move || {
                                            active_year.get().as_deref() == Some(year)
                                        };
                                        view! {
                                            <a
                                                href=year_href
                                                class=move || {
                                                    if is_active() {
                                                        "tag tag--pill tag--accent"
                                                    } else {
                                                        "tag tag--pill"
                                                    }
                                                }
                                                role="button"
                                                aria-pressed=move || is_active().to_string()
                                                on:click=move |ev| {
                                                    ev.prevent_default();
                                                    let new_val = if active_year.get().as_deref() == Some(year)
                                                    {
                                                        None
                                                    } else {
                                                        Some(year.to_string())
                                                    };
                                                    set_active_year.set(new_val);
                                                }
                                            >
                                                {year}
                                            </a>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        </div>
                    </div>
                </div>
            </Band>

            <section class=style::posts_section>
                <div class="container">
                    <div class="eyebrow-grid">
                        <span class="eyebrow">"Posts"</span>
                        <div>
                            {move || {
                                filtered()
                                    .into_iter()
                                    .enumerate()
                                    .map(|(i, post)| {
                                        let border = if i == 0 {
                                            "rule-section"
                                        } else {
                                            "rule-list"
                                        };
                                        let href = format!("/blog/{}", post.slug);
                                        let source_info = post
                                            .source
                                            .and_then(|s| {
                                                source_domain(s).map(|d| (d.to_string(), s))
                                            });
                                        view! {
                                            <div
                                                class=format!("{} {border}", style::post_row)
                                                data-testid="post-row"
                                            >
                                                <a href=href class=style::post_title>
                                                    {post.title}
                                                </a>
                                                <span class=style::post_date>{format_date(post.date)}</span>
                                                <div
                                                    class=style::post_excerpt
                                                    inner_html=post.excerpt_html
                                                />
                                                {source_info
                                                    .map(|(domain, src)| {
                                                        view! {
                                                            <a
                                                                href=src
                                                                class=style::crosspost_source
                                                                target="_blank"
                                                                rel="noopener noreferrer"
                                                            >
                                                                "↗ originally on "
                                                                {domain}
                                                            </a>
                                                        }
                                                    })}
                                                {(!post.tags.is_empty())
                                                    .then(|| {
                                                        view! {
                                                            <div class=style::post_tags>
                                                                {post
                                                                    .tags
                                                                    .iter()
                                                                    .map(|&t| {
                                                                        view! { <span class="tag tag--hash">{t}</span> }
                                                                    })
                                                                    .collect_view()}
                                                            </div>
                                                        }
                                                    })}
                                            </div>
                                        }
                                    })
                                    .collect_view()
                            }}
                        </div>
                    </div>
                </div>
            </section>
        </main>
        <Footer />
    }
}
