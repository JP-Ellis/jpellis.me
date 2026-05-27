use leptos::prelude::*;
use leptos_router::hooks::use_location;
use stylance::import_style;

mod clock;
use clock::Clock;

import_style!(style, "masthead.module.scss");

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
                                        style::nav_link.to_string()
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
