use leptos::prelude::*;

use crate::components::Footer;
use crate::components::Masthead;
use crate::pages::not_found::NotFoundPage;
use crate::pages::projects::find_project_page;

#[component]
pub fn ProjectDetailPage() -> impl IntoView {
    use leptos_router::hooks::use_params_map;
    let params = use_params_map();
    let page = move || params.with(|p| p.get("slug").as_deref().and_then(find_project_page));

    view! {
        {move || match page() {
            None => {
                view! {
                    <NotFoundPage
                        heading="Project not found."
                        back_href="/projects"
                        back_label="← All projects"
                    />
                }
                    .into_any()
            }
            Some(p) => {
                view! {
                    <Masthead />
                    <main>
                        <section>
                            <div class="container">
                                <h1>{p.title}</h1>
                            </div>
                        </section>
                    </main>
                    <Footer />
                }
                    .into_any()
            }
        }}
    }
}
