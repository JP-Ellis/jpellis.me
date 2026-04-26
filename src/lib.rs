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
                    href="https://fonts.googleapis.com/css2?family=Fraunces:ital,opsz,wght@0,9..144,300..700;1,9..144,300..600&family=Newsreader:ital,opsz,wght@0,6..72,300..600;1,6..72,300..500&family=Fira+Code:wght@300;400;500;600&display=swap"
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

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/jpellis-me.css" />
        <Title text="Joshua Ellis" />
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! { <h1>"Hello, world!"</h1> }
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
