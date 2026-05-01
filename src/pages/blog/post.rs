use leptos::prelude::*;
use stylance::import_style;

use crate::components::Footer;
use crate::components::Masthead;

import_style!(style, "blog.module.scss");

#[component]
pub fn BlogPostPage() -> impl IntoView {
    view! {
        <Masthead />
        <main></main>
        <Footer />
    }
}
