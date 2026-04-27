use leptos::prelude::*;

use crate::components::Masthead;

#[component]
pub fn MastheadPage() -> impl IntoView {
    view! {
        <Masthead />
        <div class="container" style="padding-block: var(--space-7);">
            <p class="eyebrow--muted">"Masthead — isolated"</p>
            <p style="margin-block-start: var(--space-4); color: var(--color-muted); font-family: var(--font-mono); font-size: 12px;">
                "No nav link is active at this URL (" <code>"/__test/masthead"</code> "). Visit "
                <a href="/">"/"</a> " to see the Index link active."
            </p>
            <p style="margin-block-start: var(--space-3); color: var(--color-muted); font-family: var(--font-mono); font-size: 12px;">
                "Scroll past the fold to verify sticky positioning."
            </p>
            <div style="height: 200vh;" />
        </div>
    }
}
