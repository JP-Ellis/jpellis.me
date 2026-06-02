#![expect(
    clippy::module_name_repetitions,
    reason = "component names mirror their module for discoverability"
)]

use leptos::prelude::*;
use leptos_router::hooks::use_location;
use stylance::import_style;

/// Roman-numeral clock, progressively enhanced from server-rendered date.
mod clock;
use clock::Clock;

import_style!(style, "masthead.module.scss");

/// Navigation links: `(label, path)` pairs rendered in the site header.
const NAV: &[(&str, &str)] = &[
    ("Index", "/"),
    ("Projects", "/projects"),
    ("Résumé", "/resume"),
    ("Blog", "/blog"),
    ("Contact", "/contact"),
];

/// Site-wide sticky header with logo, navigation, and volume label.
#[component]
pub fn Masthead() -> impl IntoView {
    let location = use_location();

    view! {
        <header class=style::masthead>
            <a href="/" class=style::logo>
                "Joshua "
                <em>"Ellis"</em>
            </a>
            <nav class=style::nav aria-label="Site">
                {NAV
                    .iter()
                    .copied()
                    .map(|(label, path)| {
                        view! {
                            <a
                                href=path
                                aria-current=move || {
                                    (location.pathname.get() == path).then_some("page")
                                }
                                class=move || {
                                    if location.pathname.get() == path {
                                        format!("{} {}", style::nav_link, style::nav_link_active)
                                    } else {
                                        style::nav_link.to_owned()
                                    }
                                }
                            >
                                {label}
                            </a>
                        }
                    })
                    .collect_view()}
            </nav>
            <span class=style::volume>
                <Clock />
            </span>
        </header>
    }
}
