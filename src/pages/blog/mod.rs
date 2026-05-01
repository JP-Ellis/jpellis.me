pub mod post;

use leptos::prelude::*;
use stylance::import_style;

use crate::blog::POSTS;
use crate::blog::format_date;
use crate::components::Footer;
use crate::components::Masthead;

import_style!(style, "blog.module.scss");

#[component]
pub fn BlogListPage() -> impl IntoView {
    let (active_tag, set_active_tag) = signal::<Option<String>>(None);

    let filtered = move || {
        let tag = active_tag.get();
        POSTS
            .iter()
            .filter(|p| {
                tag.as_ref()
                    .map_or(true, |t| p.tags.iter().any(|&pt| pt == t.as_str()))
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

    view! {
        <Masthead />
        <main>
            <section class=style::blog_section>
                <div class="container">
                    <div class="eyebrow-grid">
                        <div>
                            <span class="eyebrow">"Blog"</span>
                            {(!all_tags.is_empty())
                                .then(|| {
                                    view! {
                                        <div class=style::post_tags>
                                            {all_tags
                                                .iter()
                                                .map(|&tag| {
                                                    view! {
                                                        <button
                                                            class=style::tag
                                                            data-active=move || active_tag.get().as_deref() == Some(tag)
                                                            on:click=move |_| {
                                                                set_active_tag
                                                                    .update(|t| {
                                                                        if t.as_deref() == Some(tag) {
                                                                            *t = None;
                                                                        } else {
                                                                            *t = Some(tag.to_string());
                                                                        }
                                                                    });
                                                            }
                                                        >
                                                            {tag}
                                                        </button>
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    }
                                })}
                        </div>
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
                                        let source_domain = post
                                            .source
                                            .and_then(|s| {
                                                s.splitn(3, '/')
                                                    .nth(2)
                                                    .map(|rest| rest.split('/').next().unwrap_or(rest))
                                                    .map(|d| (d.to_string(), s))
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
                                                {source_domain
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
                                                                    .map(|&t| view! { <span class=style::tag>{t}</span> })
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
