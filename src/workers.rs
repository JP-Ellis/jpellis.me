//! Cloudflare Workers entry point.
//!
//! Provides `fetch` and `scheduled` event handlers for the CF Workers runtime.
//! This module compiles only for WASM32 SSR targets.
//!
//! - **`fetch`**: Routes HTTP requests through Leptos SSR. Injects both
//!   [`StatsProvider`] and [`WorkStatsProvider`] into Leptos context.
//!
//! - **`scheduled`**: Dispatches on `event.cron()`:
//!   - Daily (`0 13 * * *`) → refreshes `github/stats`.
//!   - Weekly (`0 13 * * 1`) → refreshes `github/work`.

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
use crate::integration::WorkStatsProvider;
use crate::integration::github::stats::fetch::fetch_from_github;
use crate::integration::github::work::fetch::fetch_work_stats;
use crate::shell;

/// Handles incoming HTTP fetch events.
///
/// Injects [`StatsProvider`] and [`WorkStatsProvider`] into Leptos context so
/// server functions can access GitHub stats and work stats respectively.
///
/// # Errors
///
/// Returns a [`worker::Error`] if a required binding (`GITHUB_STATS` or
/// `WORK_STATS` KV, or `GITHUB_TOKEN` secret) is missing, or if Axum fails
/// to produce a response.
#[event(fetch)]
pub async fn fetch_handler(
    req: HttpRequest,
    env: Env,
    ctx: Context,
) -> Result<axum::response::Response> {
    let stats_kv = env.kv("GITHUB_STATS")?;
    let work_kv = env.kv("WORK_STATS")?;
    let token = env.secret("GITHUB_TOKEN")?.to_string();
    let ctx = std::sync::Arc::new(ctx);

    let stats_provider = StatsProvider::kv(stats_kv, ctx.clone(), token.clone());
    let work_provider = WorkStatsProvider::kv(work_kv, ctx, token);

    let leptos_options = leptos::config::LeptosOptions::builder()
        .output_name("jpellis-me")
        .build();
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || {
                provide_context(stats_provider.clone());
                provide_context(work_provider.clone());
            },
            {
                let opts = leptos_options.clone();
                move || shell(opts.clone())
            },
        )
        .with_state(leptos_options);

    Ok(app.oneshot(req).await?)
}

/// Handles scheduled cron events.
///
/// - `"0 13 * * *"` (daily) — refreshes GitHub contribution stats.
/// - `"0 13 * * 1"` (weekly, Monday) — refreshes per-repo work stats.
#[event(scheduled)]
pub async fn scheduled_handler(event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    let token = match env.secret("GITHUB_TOKEN") {
        Ok(s) => s.to_string(),
        Err(e) => {
            console_error!("GITHUB_TOKEN secret error: {e}");
            return;
        }
    };

    match event.cron().as_str() {
        "0 13 * * 1" => refresh_work_stats(env, &token).await,
        _ => refresh_github_stats(env, &token).await,
    }
}

async fn refresh_github_stats(env: Env, token: &str) {
    let kv = match env.kv("GITHUB_STATS") {
        Ok(k) => k,
        Err(e) => {
            console_error!("GITHUB_STATS KV binding error: {e}");
            return;
        }
    };
    match fetch_from_github(token).await {
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

async fn refresh_work_stats(env: Env, token: &str) {
    let kv = match env.kv("WORK_STATS") {
        Ok(k) => k,
        Err(e) => {
            console_error!("WORK_STATS KV binding error: {e}");
            return;
        }
    };
    let fresh = fetch_work_stats(token, &crate::config::work::work_config().tracked_slugs).await;
    match serde_json::to_string(&fresh) {
        Ok(json) => match kv.put("work-stats", json) {
            Ok(builder) => {
                if let Err(e) = builder.execute().await {
                    console_error!("KV work-stats write error: {e}");
                } else {
                    console_log!(
                        "Work stats refreshed successfully ({} repos)",
                        fresh.repos.len()
                    );
                }
            }
            Err(e) => console_error!("KV work-stats put setup error: {e}"),
        },
        Err(e) => console_error!("Work stats serialisation error: {e}"),
    }
}
