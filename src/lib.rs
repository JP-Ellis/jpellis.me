use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_meta::MetaTags;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_meta::provide_meta_context;
use leptos_router::components::Route;
use leptos_router::components::Router;
use leptos_router::components::Routes;
use leptos_router::path;

mod components;
pub(crate) mod config;
pub mod integration;
mod pages;

#[cfg(all(target_arch = "wasm32", feature = "ssr"))]
mod workers;

#[cfg(debug_assertions)]
mod test_pages;

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
                <link
                    href="https://fonts.googleapis.com/css2?family=Fraunces:ital,opsz,wght@0,9..144,100..700;1,9..144,100..600&family=Newsreader:ital,opsz,wght@0,6..72,300..700;1,6..72,300..600&family=Fira+Code:wght@300;400;500;600&display=swap"
                    rel="stylesheet"
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[cfg(not(debug_assertions))]
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/jpellis-me.css" />
        <Title text="Joshua Ellis" />
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=path!("") view=pages::HomePage />
                <Route path=path!("contact") view=pages::ContactPage />
                <Route path=path!("resume") view=pages::ResumePage />
                <Route path=path!("work") view=pages::WorkPage />
            </Routes>
        </Router>
    }
}

#[cfg(debug_assertions)]
#[component]
pub fn App() -> impl IntoView {
    use leptos_router::components::ParentRoute;

    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/jpellis-me.css" />
        <Title text="Joshua Ellis" />
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=path!("") view=pages::HomePage />
                <Route path=path!("contact") view=pages::ContactPage />
                <Route path=path!("resume") view=pages::ResumePage />
                <Route path=path!("work") view=pages::WorkPage />
                <ParentRoute path=path!("__test") view=test_pages::TestLayout>
                    <Route path=path!("") view=test_pages::TestIndex />
                    <Route path=path!("masthead") view=test_pages::MastheadPage />
                    <Route path=path!("footer") view=test_pages::FooterPage />
                    <Route path=path!("band") view=test_pages::BandPage />
                    <Route path=path!("css-foundation") view=test_pages::CssFoundationPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use leptos::mount::hydrate_body;
    console_error_panic_hook::set_once();
    hydrate_body(App);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
