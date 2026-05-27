#![recursion_limit = "256"]
// Native (Axum) SSR entry point.  Not compiled for WASM targets — CF Workers
// uses `workers.rs` (`#[event(fetch)]`) as its entry point instead.
#[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
#[tokio::main]
async fn main() {
    use axum::Router;
    use jpellis_me::App;
    use jpellis_me::integration::ProjectsStatsProvider;
    use jpellis_me::integration::StatsProvider;
    use jpellis_me::shell;
    use leptos::prelude::provide_context;
    use leptos_axum::LeptosRoutes;
    use leptos_axum::generate_route_list;
    use leptos_config::get_configuration;

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
    let stats_provider = StatsProvider::file(token.clone());
    let projects_provider = ProjectsStatsProvider::file(token);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || {
                provide_context(stats_provider.clone());
                provide_context(projects_provider.clone());
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(all(feature = "ssr", not(target_arch = "wasm32"))))]
fn main() {}
