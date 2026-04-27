use leptos::prelude::*;

use crate::components::Band;

#[component]
pub fn BandPage() -> impl IntoView {
    view! {
        <div class="container" style="padding-block: var(--space-6);">
            <p class="eyebrow--muted">"Band — isolated"</p>
            <p style="margin-block-start: var(--space-3); color: var(--color-muted); font-family: var(--font-mono); font-size: 12px;">
                "Inverted background. " <code>".eyebrow"</code> ", " <code>".rule-section"</code>
                ", and " <code>".rule-list"</code>
                " inside a band use band tokens via :global() selectors."
            </p>
        </div>

        <Band test_id="band-demo-content">
            <div class="container" style="padding-block: var(--space-6);">
                <p class="eyebrow">"Eyebrow inside band (band-muted)"</p>
                <p style="margin-block: var(--space-4) 0;">
                    "Body text in " <code>"--color-band-text"</code> ". "
                    <a href="#">"Link uses band-text, accent on hover."</a>
                </p>
            </div>
        </Band>

        <div style="height: var(--space-5);" />

        <Band test_id="band-demo-hairlines">
            <div class="container" style="padding-block: var(--space-6);">
                <p class="eyebrow">"Hairlines inside band"</p>
                <p style="margin-block-start: var(--space-4);">"Above rule-section"</p>
                <hr class="rule-section" />
                <p>"Below rule-section / above rule-list"</p>
                <hr class="rule-list" />
                <p>"Below rule-list"</p>
            </div>
        </Band>
    }
}
