use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "style.module.scss");

/// Full-bleed contrast section — dark in light mode, light in dark mode.
///
/// Always use `--color-band-*` tokens for text and borders inside a band.
/// The optional `test_id` sets a `data-testid` attribute for Playwright tests.
#[component]
pub fn Band(children: Children, #[prop(optional)] test_id: Option<&'static str>) -> impl IntoView {
    view! {
        <section class=style::band data-testid=test_id>
            {children()}
        </section>
    }
}
