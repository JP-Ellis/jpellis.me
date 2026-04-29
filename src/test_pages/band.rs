use leptos::prelude::*;

use crate::components::Band;

#[component]
pub fn BandPage() -> impl IntoView {
    view! {
        <div class="container" style="padding-block: var(--space-6);">
            <p class="eyebrow--muted">"Band — isolated"</p>
            <p style="margin-block-start: var(--space-3); color: var(--color-muted); font-family: var(--font-mono); font-size: 12px;">
                "Inverted background. The band locally swaps --color-paper ↔ --color-ink "
                "so all derived tokens invert automatically. No band-specific tokens needed."
            </p>
        </div>

        <Band test_id="band-demo-content">
            <div class="container" style="padding-block: var(--space-6);">
                <p class="eyebrow">"Eyebrow inside band (--color-accent, auto-inverted context)"</p>
                <p style="margin-block: var(--space-4) 0;">
                    "Body text uses " <code>"--color-ink"</code> " which auto-inverts to paper. "
                    <a href="#">"Links inherit the inverted ink colour."</a>
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
