use leptos::prelude::*;
use leptos_meta::Title;
use stylance::import_style;

use crate::components::Footer;
use crate::components::Masthead;

import_style!(style, "not_found.module.scss");

/// Generic not-found page, used both as the route wildcard catch-all and for
/// missing blog posts. Sets HTTP 404 status in SSR context.
#[component]
pub fn NotFoundPage(
    /// Heading shown in the hero, e.g. `"Page not found."` or `"Post not found."`
    #[prop(default = "Page not found.")]
    heading: &'static str,
    /// URL for the back link, e.g. `"/"` or `"/blog"`.
    #[prop(default = "/")]
    back_href: &'static str,
    /// Visible text of the back link, e.g. `"← Home"` or `"← All posts"`.
    #[prop(default = "← Home")]
    back_label: &'static str,
) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        use axum::http::StatusCode;
        use leptos_axum::ResponseOptions;
        if let Some(resp) = use_context::<ResponseOptions>() {
            resp.set_status(StatusCode::NOT_FOUND);
        }
    }

    view! {
        <Title text="404 — Not found" />
        <Masthead />
        <main>
            <section class=style::hero>
                <div class="container">
                    <p class="eyebrow">"404"</p>
                    <h1>{heading}</h1>
                    <a href=back_href class=style::back_link>
                        {back_label}
                    </a>
                </div>
            </section>
        </main>
        <Footer />
    }
}
