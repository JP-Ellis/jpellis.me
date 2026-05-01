use leptos::prelude::*;
use leptos_meta::Link;
use leptos_meta::Meta;
use leptos_meta::Script;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use stylance::import_style;

use crate::blog::find_post;
use crate::blog::format_date;
use crate::blog::source_domain;
use crate::components::Footer;
use crate::components::Masthead;

import_style!(style, "blog.module.scss");

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

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    let post = move || params.with(|p| p.get("slug").as_deref().and_then(find_post));

    #[cfg(target_arch = "wasm32")]
    Effect::new(|_| {
        prism::highlight_all();
    });

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
                    <Stylesheet href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-tomorrow.min.css" />
                    <Script
                        src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/prism.min.js"
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
                    <Script
                        src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-bash.min.js"
                        defer=""
                    />
                    <Script
                        src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-toml.min.js"
                        defer=""
                    />
                    <Title text=p.title />
                    <Meta name="description" content=desc />
                    {p.source.map(|src| view! { <Link rel="canonical" href=src /> })}
                    <Masthead />
                    <main>
                        <article class=style::article>
                            <div class="container container--prose">
                                <a href="/blog" class=style::back_link>
                                    "← All posts"
                                </a>
                                {p
                                    .source
                                    .map(|src| {
                                        let domain = source_domain(src).unwrap_or(src);
                                        view! {
                                            <p class=style::crosspost_banner>
                                                "Originally published on "
                                                <a href=src target="_blank" rel="noopener noreferrer">
                                                    {domain}
                                                </a>
                                            </p>
                                        }
                                    })}
                                <header class=style::post_header>
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
                                                            view! { <span class="tag tag--hash">{t}</span> }
                                                        })
                                                        .collect_view()}
                                                }
                                            })}
                                    </div>
                                </header>
                                <div class="prose" inner_html=p.body_html />
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
