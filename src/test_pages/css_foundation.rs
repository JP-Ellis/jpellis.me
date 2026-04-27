use leptos::prelude::*;

use crate::components::Band;
use crate::components::Footer;
use crate::components::Masthead;

#[component]
fn SpacingRow(#[prop(into)] token: String) -> impl IntoView {
    let bar_style = format!(
        "width: var({token}); height: 4px; background: var(--color-ink); \
         display: inline-block; flex-shrink: 0;"
    );
    view! {
        <div style="display: flex; align-items: center; gap: var(--space-4); margin-block: var(--space-2);">
            <span class="eyebrow--muted" style="width: 80px; flex-shrink: 0;">
                {token}
            </span>
            <div data-spacing-bar="true" style=bar_style />
        </div>
    }
}

#[component]
fn Swatch(#[prop(into)] token: String) -> impl IntoView {
    let testid = format!("swatch-{}", token.trim_start_matches("--"));
    let style = format!(
        "display: inline-block; width: 48px; height: 48px; \
         background-color: var({token}); border: 1px solid var(--color-rule);"
    );
    view! { <span data-testid=testid style=style /> }
}

#[component]
pub fn CssFoundationPage() -> impl IntoView {
    view! {
        <div class="container" style="padding-block: var(--space-7);">
            <p class="eyebrow--muted">
                "To test dark mode: DevTools → Rendering → Emulate CSS media → "
                <code>"prefers-color-scheme: dark"</code>
            </p>

            <div
                data-testid="section-typography"
                class="eyebrow-grid"
                style="margin-block: var(--space-7);"
            >
                <span class="eyebrow">"Typography"</span>
                <div>
                    <h1>"Display heading h1"</h1>
                    <h2>"Section heading h2"</h2>
                    <h3>"Sub-section h3"</h3>
                    <h4>"Group heading h4"</h4>
                    <h5>"Label heading h5"</h5>
                    <h6>"Minor heading h6"</h6>
                    <p>"Body paragraph — Newsreader, the workhorse of running text."</p>
                    <p>
                        <a href="#">"Hyperlink example"</a>
                        " — inline anchor."
                    </p>
                    <p>"Inline " <code>"code element"</code> " in running text."</p>
                    <pre>
                        <code>"fn main() {\n    println!(\"hello\");\n}"</code>
                    </pre>
                    <p>
                        <strong>"Strong text"</strong>
                        " and "
                        <em>"emphasised text."</em>
                    </p>
                    <div class="container--prose" style="margin-block-start: var(--space-4);">
                        <p>
                            "Inside .container--prose — max-width 880px, for reading-optimised pages."
                        </p>
                    </div>
                </div>
            </div>

            <div
                data-testid="section-colors"
                class="eyebrow-grid"
                style="margin-block: var(--space-7);"
            >
                <span class="eyebrow">"Colors"</span>
                <div>
                    <div style="display: flex; flex-wrap: wrap; gap: var(--space-3); margin-block-end: var(--space-3);">
                        <Swatch token="--color-paper" />
                        <Swatch token="--color-ink" />
                        <Swatch token="--color-accent" />
                        <Swatch token="--color-paper-deep" />
                        <Swatch token="--color-muted" />
                        <Swatch token="--color-faint" />
                        <Swatch token="--color-rule" />
                        <Swatch token="--color-accent-soft" />
                    </div>
                    <div style="display: flex; flex-wrap: wrap; gap: var(--space-3);">
                        <Swatch token="--color-band-bg" />
                        <Swatch token="--color-band-text" />
                        <Swatch token="--color-band-muted" />
                        <Swatch token="--color-band-faint" />
                    </div>
                </div>
            </div>

            <div
                data-testid="section-spacing"
                class="eyebrow-grid"
                style="margin-block: var(--space-7);"
            >
                <span class="eyebrow">"Spacing"</span>
                <div>
                    <SpacingRow token="--space-1" />
                    <SpacingRow token="--space-2" />
                    <SpacingRow token="--space-3" />
                    <SpacingRow token="--space-4" />
                    <SpacingRow token="--space-5" />
                    <SpacingRow token="--space-6" />
                    <SpacingRow token="--space-7" />
                    <SpacingRow token="--space-8" />
                </div>
            </div>

            <div
                data-testid="section-eyebrow"
                class="eyebrow-grid"
                style="margin-block: var(--space-7);"
            >
                <span class="eyebrow">"Eyebrow"</span>
                <div style="display: flex; gap: var(--space-6); align-items: baseline;">
                    <span class="eyebrow">"№ 01 / Accent"</span>
                    <span class="eyebrow eyebrow--muted">"2026-04-26 / Muted"</span>
                </div>
            </div>

            <div
                data-testid="section-hairlines"
                class="eyebrow-grid"
                style="margin-block: var(--space-7);"
            >
                <span class="eyebrow">"Hairlines"</span>
                <div>
                    <hr class="rule-section" />
                    <ul style="list-style: none; padding: 0; margin-block: var(--space-4);">
                        <li style="padding-block: var(--space-3);">"First list item"</li>
                        <hr class="rule-list" />
                        <li style="padding-block: var(--space-3);">"Second list item"</li>
                        <hr class="rule-list" />
                        <li style="padding-block: var(--space-3);">"Third list item"</li>
                    </ul>
                </div>
            </div>

            <div
                data-testid="section-tags"
                class="eyebrow-grid"
                style="margin-block: var(--space-7);"
            >
                <span class="eyebrow">"Tags"</span>
                <div>
                    <div style="display: flex; flex-wrap: wrap; gap: var(--space-3);">
                        <span class="tag">"plain"</span>
                        <span class="tag tag--pill">"pill"</span>
                        <span class="tag tag--hash">"#hash"</span>
                        <span class="tag tag--hash tag--accent">"#accent"</span>
                    </div>
                    <p class="eyebrow--muted" style="margin-block-start: var(--space-3);">
                        "Hover each tag to see background shift."
                    </p>
                </div>
            </div>

            <div
                data-testid="section-eyebrow-grid"
                class="eyebrow-grid"
                style="margin-block: var(--space-7);"
            >
                <span class="eyebrow">"Live eyebrow-grid"</span>
                <div>
                    <p>"First paragraph of content in the right column."</p>
                    <p>"Second paragraph — the grid aligns the label to the first baseline."</p>
                </div>
            </div>
        </div>

        <Band test_id="section-band">
            <div class="container" style="padding-block: var(--space-7);">
                <span class="eyebrow">"Band"</span>
                <h2>"Always the inverse of the page colour scheme."</h2>
                <div style="display: flex; flex-wrap: wrap; gap: var(--space-3); margin-block: var(--space-5);">
                    <span class="tag tag--pill">"band-tag"</span>
                </div>
                <hr class="rule-list" />
                <p style="margin-block-start: var(--space-5);">
                    "Light mode → dark band. Dark mode → light band. Uses --color-band-* tokens derived from --color-ink / --color-paper."
                </p>
            </div>
        </Band>

        <div class="container" style="padding-block: var(--space-7);">
            <div
                data-testid="section-masthead"
                class="eyebrow-grid"
                style="margin-block: var(--space-7);"
            >
                <span class="eyebrow">"Masthead"</span>
                <div style="position: relative; overflow: hidden;">
                    <Masthead />
                </div>
            </div>

            <div
                data-testid="section-footer"
                class="eyebrow-grid"
                style="margin-block: var(--space-7);"
            >
                <span class="eyebrow">"Footer"</span>
                <Footer />
            </div>

            <div
                data-testid="section-focus"
                class="eyebrow-grid"
                style="margin-block: var(--space-7);"
            >
                <span class="eyebrow">"Focus"</span>
                <div style="display: flex; gap: var(--space-5); align-items: center; flex-wrap: wrap;">
                    <button style="padding: 8px 16px; cursor: pointer;">
                        "Tab to this button to verify focus ring"
                    </button>
                    <a href="#" style="padding: 4px;">
                        "Tab to this link to verify focus ring"
                    </a>
                </div>
            </div>

            <div
                data-testid="section-sr-only"
                class="eyebrow-grid"
                style="margin-block: var(--space-7);"
            >
                <span class="eyebrow">"sr-only"</span>
                <div>
                    <span class="sr-only">"This text is visually hidden"</span>
                    <p>"The span above this paragraph is visually hidden (inspect to verify)."</p>
                </div>
            </div>
        </div>
    }
}
