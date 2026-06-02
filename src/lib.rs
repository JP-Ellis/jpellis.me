//! Personal website built with Leptos.
#![recursion_limit = "256"]
// The #[component] macro generates exhaustive structs for props types; we have
// no control over the generated code so must suppress the lint crate-wide.
#![expect(
    clippy::exhaustive_structs,
    clippy::must_use_candidate,
    clippy::missing_inline_in_public_items,
    reason = "Leptos #[component] macro generates exhaustive prop structs and public functions that cannot easily carry these attributes"
)]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_meta::MetaTags;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_meta::provide_meta_context;
use leptos_router::components::Route;
use leptos_router::components::Router;
use leptos_router::components::Routes;
use leptos_router::path;

/// Blog posts loaded at build time.
pub mod blog;
/// Reusable UI components.
mod components;
/// Static site configuration (projects, etc.).
pub(crate) mod config;
/// External service integrations (GitHub stats, etc.).
pub mod integration;
/// Page-level Leptos components.
mod pages;

/// Cloudflare Workers entry point (WASM + SSR).
#[cfg(all(target_arch = "wasm32", feature = "ssr"))]
mod workers;

/// Test-only pages for visual regression and component review.
#[cfg(debug_assertions)]
mod test_pages;

#[cfg(feature = "ssr")]
/// Returns the full HTML shell for server-side rendering.
#[must_use]
#[inline]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                <link
                    href="https://fonts.googleapis.com/css2?family=Fraunces:ital,opsz,wght@0,9..144,100..700;1,9..144,100..600&family=Newsreader:ital,opsz,wght@0,6..72,300..700;1,6..72,300..600&family=Fira+Code:wght@300;400;500;600&display=swap"
                    rel="stylesheet"
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[cfg(not(debug_assertions))]
/// Root Leptos application component.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/jpellis-me.css" />
        <Title text="Joshua Ellis" />
        <Router>
            <Routes fallback=|| view! { <pages::NotFoundPage /> }>
                <Route path=path!("") view=pages::HomePage />
                <Route path=path!("contact") view=pages::ContactPage />
                <Route path=path!("resume") view=pages::ResumePage />
                <Route path=path!("projects") view=pages::ProjectsPage />
                <Route path=path!("projects/:slug") view=pages::ProjectDetailPage />
                <Route path=path!("blog") view=pages::BlogListPage />
                <Route path=path!("blog/:slug") view=pages::BlogPostPage />
                <Route path=path!("*any") view=|| view! { <pages::NotFoundPage /> } />
            </Routes>
        </Router>
    }
}

#[cfg(debug_assertions)]
/// Root Leptos application component (debug build with test routes).
#[component]
pub fn App() -> impl IntoView {
    use leptos_router::components::ParentRoute;

    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/jpellis-me.css" />
        <Title text="Joshua Ellis" />
        <Router>
            <Routes fallback=|| view! { <pages::NotFoundPage /> }>
                <Route path=path!("") view=pages::HomePage />
                <Route path=path!("contact") view=pages::ContactPage />
                <Route path=path!("resume") view=pages::ResumePage />
                <Route path=path!("projects") view=pages::ProjectsPage />
                <Route path=path!("projects/:slug") view=pages::ProjectDetailPage />
                <Route path=path!("blog") view=pages::BlogListPage />
                <Route path=path!("blog/:slug") view=pages::BlogPostPage />
                <ParentRoute path=path!("__test") view=test_pages::TestLayout>
                    <Route path=path!("") view=test_pages::TestIndex />
                    <Route path=path!("masthead") view=test_pages::MastheadPage />
                    <Route path=path!("footer") view=test_pages::FooterPage />
                    <Route path=path!("band") view=test_pages::BandPage />
                    <Route path=path!("css-foundation") view=test_pages::CssFoundationPage />
                </ParentRoute>
                <Route path=path!("*any") view=|| view! { <pages::NotFoundPage /> } />
            </Routes>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
/// Client-side hydration entry point.
pub fn hydrate() {
    use leptos::mount::hydrate_body;
    console_error_panic_hook::set_once();
    hydrate_body(App);
}

#[cfg(test)]
mod tests {
    #![expect(
        clippy::default_numeric_fallback,
        reason = "trivial test; integer types are unambiguous"
    )]
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
