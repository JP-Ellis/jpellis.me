use leptos::prelude::*;
use stylance::import_style;

use crate::components::Band;
use crate::components::Footer;
use crate::components::Masthead;

mod email;
mod pgp;
import_style!(style, "contact.module.scss");

const PGP_KEY: pgp::PgpKey = pgp::PgpKey("AA152D8F537EE25D3AC2FADCF162288C8BA20FCE");
const EMAIL: email::EmailAddress = email::EmailAddress("website@jpellis.me");

/// Contact page.
#[component]
pub fn ContactPage() -> impl IntoView {
    let copied = RwSignal::new(false);

    let copy_email = move |_| {
        #[cfg(feature = "hydrate")]
        {
            if let Some(window) = leptos::web_sys::window() {
                let _ = window.navigator().clipboard().write_text(EMAIL.as_ref());
            }
            copied.set(true);
            leptos::leptos_dom::helpers::set_timeout(
                move || copied.set(false),
                std::time::Duration::from_secs(2),
            );
        }
    };

    view! {
        <Masthead />
        <main>
            // ── Hero ───────────────────────────────────────────────
            <section class=style::hero>
                <div class="container">
                    <p class="eyebrow">"Contact"</p>
                    <h1>
                        "The best way to reach me is " <em class=style::hero_accent>"email"</em>
                    </h1>
                    <p class=style::hero_lead>
                        "I read everything that lands in my inbox; I don't always reply quickly, \
                         but I do reply. Be specific about what you'd like — it shortens the loop \
                         for both of us."
                    </p>
                </div>
            </section>

            // ── Email band ─────────────────────────────────────────
            <Band>
                <div class="container">
                    <div class=style::band_inner>
                        <span class="eyebrow">"Write to"</span>
                        <p class=style::email_display>{EMAIL.display_view()}</p>
                        <div class=style::band_actions>
                            <a href=format!("mailto:{EMAIL}") class="btn">
                                "↗ open in mail client"
                            </a>
                            <noscript>
                                <style>".copy-btn { display: none; }"</style>
                            </noscript>
                            <button class="btn copy-btn" on:click=copy_email>
                                {move || if copied.get() { "✓ copied" } else { "copy address" }}
                            </button>
                            <a
                                href=PGP_KEY.url()
                                class=style::pgp
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                // spellchecker:ignore-next-line
                                {move || format!("pgp · {PGP_KEY}")}
                            </a>
                        </div>
                    </div>
                </div>
            </Band>

            // ── Elsewhere ──────────────────────────────────────────
            <section class=style::elsewhere_section>
                <div class="container">
                    <span class="eyebrow">"Or, elsewhere"</span>
                    <div class=style::channel_grid>
                        <div class=style::channel_card>
                            <span class="eyebrow--muted">"Email"</span>
                            <a href="mailto:website@jpellis.me" class=style::channel_handle>
                                "website@jpellis.me"
                            </a>
                            <p class=style::channel_desc>"preferred"</p>
                        </div>
                        <div class=style::channel_card>
                            <span class="eyebrow--muted">"GitHub"</span>
                            <a
                                href="https://github.com/JP-Ellis"
                                class=style::channel_handle
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                "github.com/JP-Ellis"
                            </a>
                            <p class=style::channel_desc>"where most of the work lives"</p>
                        </div>
                        <div class=style::channel_card>
                            <span class="eyebrow--muted">"LinkedIn"</span>
                            <a
                                href="https://linkedin.com/in/joshuapellis"
                                class=style::channel_handle
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                "linkedin.com/in/joshuapellis"
                            </a>
                            <p class=style::channel_desc>"for recruiting conversations"</p>
                        </div>
                        <div class=style::channel_card>
                            <span class="eyebrow--muted">"ORCID"</span>
                            <a
                                href="https://orcid.org/0000-0003-2556-1536"
                                class=style::channel_handle
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                "0000-0003-2556-1536"
                            </a>
                            <p class=style::channel_desc>"for the academic side"</p>
                        </div>
                    </div>
                </div>
            </section>

            // ── A note ─────────────────────────────────────────────
            <section class=style::note_section>
                <div class="container">
                    <div class="eyebrow-grid">
                        <span class="eyebrow">"A note"</span>
                        <div>
                            <p>
                                "If you're filing a bug report or feature request in one of my \
                                 open-source projects — TikZ-Feynman, the Pact Rust \
                                 implementation, or anything else on GitHub — please open an issue \
                                 in the relevant repository rather than emailing me. It's easier \
                                 to track, discuss, and keep visible there."
                            </p>
                            <p class=style::note_muted>
                                "I'm not currently looking for new roles, but I'm always happy to \
                                 chat about Pact and contract testing, Rust ↔ Python interop, or \
                                 particle-physics numerics."
                            </p>
                        </div>
                    </div>
                </div>
            </section>
        </main>
        <Footer />
    }
}
