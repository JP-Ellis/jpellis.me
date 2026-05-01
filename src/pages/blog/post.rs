use leptos::prelude::*;
use leptos_meta::Link;
use leptos_meta::Meta;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use stylance::import_style;

use crate::blog::find_post;
use crate::blog::format_date;
use crate::components::Footer;
use crate::components::Masthead;

import_style!(style, "blog.module.scss");

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    let post = move || params.with(|p| p.get("slug").as_deref().and_then(find_post));

    view! {
        {move || match post() {
            None => {
                view! {
                    <Masthead />
                    <main>
                        <section>
                            <div class="container">
                                <p>"Post not found."</p>
                                <a href="/blog">"← All posts"</a>
                            </div>
                        </section>
                    </main>
                    <Footer />
                }
                    .into_any()
            }
            Some(p) => {
                let desc = p.description.unwrap_or("");

                view! {
                    <Title text=p.title />
                    <Meta name="description" content=desc />
                    {p.source.map(|src| view! { <Link rel="canonical" href=src /> })}
                    <Masthead />
                    <main>
                        <article class=style::article>
                            <div class="container">
                                {p
                                    .source
                                    .map(|src| {
                                        let domain = src
                                            .splitn(3, '/')
                                            .nth(2)
                                            .map(|rest| rest.split('/').next().unwrap_or(rest))
                                            .unwrap_or(src);
                                        view! {
                                            <p class=style::crosspost_banner>
                                                "Originally published on "
                                                <a href=src target="_blank" rel="noopener noreferrer">
                                                    {domain}
                                                </a>
                                            </p>
                                        }
                                    })} <header class=style::post_header>
                                    <h1 class=style::post_heading>{p.title}</h1>
                                    <div class=style::post_meta>
                                        <time datetime=p.date>{format_date(p.date)}</time>
                                        {(!p.tags.is_empty())
                                            .then(|| {
                                                view! {
                                                    <span>"·"</span>
                                                    {p
                                                        .tags
                                                        .iter()
                                                        .map(|&t| {
                                                            view! { <span class=style::tag>{t}</span> }
                                                        })
                                                        .collect_view()}
                                                }
                                            })}
                                    </div>
                                </header> <div class=style::prose inner_html=p.body_html />
                            </div>
                        </article>
                    </main>
                    <Footer />
                }
                    .into_any()
            }
        }}
    }
}
