use leptos::prelude::*;
use stylance::import_style;

import_style!(style, "footer.module.scss");

/// Site-wide footer with copyright, social links, and licence.
#[expect(
    clippy::module_name_repetitions,
    reason = "the full name is clearer in cross-module imports"
)]
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class=style::footer>
            <span class=style::item>"© 2026 Joshua P. Ellis"</span>
            <span class=style::item>
                <a href="https://github.com/JP-Ellis" rel="noopener noreferrer">
                    "github"
                </a>
                " · "
                <a href="https://linkedin.com/in/joshuapellis" rel="noopener noreferrer">
                    "linkedin"
                </a>
                " · "
                <a href="mailto:josh@jpellis.me" rel="noopener noreferrer">
                    "email"
                </a>
            </span>
            <span class=style::item>"cc by 4.0"</span>
        </footer>
    }
}
