//! Cloudflare Workers entry point.
//!
//! Provides `fetch` and `scheduled` event handlers for the CF Workers runtime.
//! This module compiles only for WASM32 SSR targets.
//!
//! - **`fetch`**: Handles incoming HTTP requests by routing them through an
//!   Axum router with Leptos SSR.  A [`StatsProvider`] is injected into Leptos
//!   context so the `get_github_stats` server function can access KV.
//!
//! - **`scheduled`**: Runs on a cron trigger, fetches live GitHub statistics,
//!   and writes the result to the `GITHUB_STATS` KV namespace.

#![cfg(all(target_arch = "wasm32", feature = "ssr"))]

use axum::Router;
use leptos::prelude::provide_context;
use leptos_axum::LeptosRoutes;
use leptos_axum::generate_route_list;
use tower::util::ServiceExt;
use worker::Context;
use worker::Env;
use worker::HttpRequest;
use worker::Result;
use worker::ScheduleContext;
use worker::ScheduledEvent;
use worker::console_error;
use worker::console_log;
use worker::event;

use crate::App;
use crate::integration::StatsProvider;
use crate::integration::github::stats::fetch::fetch_from_github;
use crate::shell;

/// Handles incoming HTTP fetch events by routing requests through a Leptos +
/// Axum application.
///
/// Injects a [`StatsProvider`] with the `GITHUB_STATS` KV store, the CF
/// [`Context`], and the `GITHUB_TOKEN` secret into Leptos context so server
/// functions can retrieve GitHub stats.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request forwarded from the Workers runtime.
/// * `env` - The CF Workers environment, used to access KV namespaces and
///   secrets.
/// * `ctx` - The CF Workers context, used for `wait_until` background tasks.
///
/// # Errors
///
/// Returns a [`worker::Error`] if a required binding (`GITHUB_STATS` KV or
/// `GITHUB_TOKEN` secret) is missing, or if Axum fails to produce a response.
#[event(fetch)]
pub async fn fetch_handler(
    req: HttpRequest,
    env: Env,
    ctx: Context,
) -> Result<axum::response::Response> {
    let kv = env.kv("GITHUB_STATS")?;
    let token = env.secret("GITHUB_TOKEN")?.to_string();
    let provider = StatsProvider::kv(kv, ctx, token);

    let leptos_options = leptos::config::LeptosOptions::builder()
        .output_name("jpellis-me")
        .build();
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(provider.clone()),
            {
                let opts = leptos_options.clone();
                move || shell(opts.clone())
            },
        )
        .with_state(leptos_options);

    Ok(app.oneshot(req).await?)
}

/// Handles scheduled cron events by refreshing the GitHub statistics cache.
///
/// Fetches live GitHub statistics via the GitHub API and writes the serialised
/// result to the `GITHUB_STATS` KV namespace under the key `"stats"`.  All
/// error conditions are logged to the Workers console rather than propagated,
/// so a single refresh failure does not affect live request handling.
///
/// # Arguments
///
/// * `_event` - The scheduled event metadata (unused).
/// * `env` - The CF Workers environment, used to access KV namespaces and
///   secrets.
/// * `_ctx` - The CF Workers schedule context (unused).
#[event(scheduled)]
pub async fn scheduled_handler(_event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    let kv = match env.kv("GITHUB_STATS") {
        Ok(k) => k,
        Err(e) => {
            console_error!("KV binding error: {e}");
            return;
        }
    };
    let token = match env.secret("GITHUB_TOKEN") {
        Ok(s) => s.to_string(),
        Err(e) => {
            console_error!("GITHUB_TOKEN secret error: {e}");
            return;
        }
    };

    match fetch_from_github(&token).await {
        Ok(fresh) => match serde_json::to_string(&fresh) {
            Ok(json) => match kv.put("stats", json) {
                Ok(builder) => {
                    if let Err(e) = builder.execute().await {
                        console_error!("KV write error: {e}");
                    } else {
                        console_log!("GitHub stats refreshed successfully");
                    }
                }
                Err(e) => console_error!("KV put setup error: {e}"),
            },
            Err(e) => console_error!("Serialisation error: {e}"),
        },
        Err(e) => console_error!("GitHub fetch error: {e}"),
    }
}
