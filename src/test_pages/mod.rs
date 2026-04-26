mod css_foundation;

pub use css_foundation::CssFoundationPage;
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn TestLayout() -> impl IntoView {
    view! {
        <div style="border-top: 2px solid var(--color-accent); padding: 8px 16px; font-family: var(--font-mono); font-size: 11px; color: var(--color-muted); letter-spacing: 0.1em; background-color: var(--color-paper-deep);">
            "⚠ debug build — /__test"
        </div>
        <nav style="padding: 8px 16px; display: flex; gap: 16px; border-bottom: 1px solid var(--color-rule);">
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
                    <a href="/__test/css-foundation">"css-foundation"</a>
                    " — all CSS foundation elements"
                </li>
            </ul>
        </div>
    }
}
