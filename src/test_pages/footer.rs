use leptos::prelude::*;

use crate::components::Footer;

#[component]
pub fn FooterPage() -> impl IntoView {
    view! {
        <div class="container" style="padding-block: var(--space-7);">
            <p class="eyebrow--muted">"Footer — isolated"</p>
        </div>
        <Footer />
    }
}
