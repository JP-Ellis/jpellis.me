#![expect(
    clippy::pub_use,
    reason = "re-exports flatten the module hierarchy for callers — the sub-modules are implementation details"
)]

/// Band component test page.
mod band;
/// CSS foundation (tokens, colours, spacing) test page.
mod css_foundation;
/// Footer component test page.
mod footer;
/// Masthead / navigation test page.
mod masthead;

pub use band::BandPage;
pub use css_foundation::CssFoundationPage;
pub use footer::FooterPage;
use leptos::prelude::*;
use leptos_router::MatchNestedRoutes;
use leptos_router::any_nested_route::IntoAnyNestedRoute as _;
use leptos_router::components::Outlet;
use leptos_router::components::ParentRoute;
use leptos_router::components::Route;
use leptos_router::path;
pub use masthead::MastheadPage;

/// All debug routes, rooted at `/__test`.
#[component(transparent)]
pub fn TestRoutes() -> impl MatchNestedRoutes + Clone + Send + 'static {
    view! {
        <ParentRoute path=path!("__test") view=TestLayout>
            <Route path=path!("") view=TestIndex />
            <Route path=path!("masthead") view=MastheadPage />
            <Route path=path!("footer") view=FooterPage />
            <Route path=path!("band") view=BandPage />
            <Route path=path!("css-foundation") view=CssFoundationPage />
        </ParentRoute>
    }
    .into_inner()
    .into_any_nested_route()
}

#[component]
pub fn TestLayout() -> impl IntoView {
    view! {
        <div style="border-top: 2px solid var(--color-accent); padding: 8px 16px; font-family: var(--font-mono); font-size: 11px; color: var(--color-muted); letter-spacing: 0.1em; background-color: var(--color-paper-deep);">
            "⚠ debug build — /__test"
        </div>
        <nav style="padding: 8px 16px; display: flex; gap: 16px; border-bottom: 1px solid var(--color-rule);">
            <a class="eyebrow" href="/__test/masthead">
                "masthead"
            </a>
            <a class="eyebrow" href="/__test/footer">
                "footer"
            </a>
            <a class="eyebrow" href="/__test/band">
                "band"
            </a>
            <a class="eyebrow" href="/__test/css-foundation">
                "css-foundation"
            </a>
        </nav>
        <Outlet />
    }
}

#[component]
pub fn TestIndex() -> impl IntoView {
    view! {
        <div class="container" style="padding-block: 32px;">
            <h1>"/__test"</h1>
            <ul style="margin-block-start: 16px; padding-inline-start: 24px;">
                <li>
                    <a href="/__test/masthead">"masthead"</a>
                    " — sticky header, nav active state, sticky scroll"
                </li>
                <li>
                    <a href="/__test/footer">"footer"</a>
                    " — three-column footer with social links"
                </li>
                <li>
                    <a href="/__test/band">"band"</a>
                    " — inverted section, band tokens, global child selectors"
                </li>
                <li>
                    <a href="/__test/css-foundation">"css-foundation"</a>
                    " — all CSS foundation elements"
                </li>
            </ul>
        </div>
    }
}
