#![expect(
    clippy::shadow_reuse,
    reason = "Leptos #[component] macro internally re-binds function parameters"
)]

use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "band.module.scss");

/// Full-bleed contrast section — dark in light mode, light in dark mode.
///
/// The colour tokens (`--color-ink`, `--color-muted`, `--color-faint`, etc.)
/// are locally inverted inside the band, so child components need no changes.
/// The optional `test_id` sets a `data-testid` attribute for Playwright tests.
#[expect(
    clippy::module_name_repetitions,
    reason = "the full name is clearer in cross-module imports"
)]
#[component]
pub fn Band(
    /// Content to render inside the contrast band.
    children: Children,
    /// Optional `data-testid` attribute value for Playwright tests.
    #[prop(optional)]
    test_id: Option<&'static str>,
) -> impl IntoView {
    view! {
        <section class=style::band data-testid=test_id>
            {children()}
        </section>
    }
}
