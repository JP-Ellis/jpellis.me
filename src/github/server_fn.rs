//! Server function for GitHub statistics.
//!
//! Provides a single [`get_github_stats`] server function that retrieves
//! [`GitHubStats`] using the appropriate strategy for the current platform:
//!
//! - **Native (Axum dev):** Calls the GitHub API directly via
//!   [`fetch_from_github`]. Falls back to [`fallback_stats`] if
//!   `GITHUB_TOKEN` is unset or the request fails.
//!
//! - **WASM32 (CF Workers):** Reads from KV with stale-while-revalidate logic.
//!   Stale entries (older than one hour) trigger a background refresh via
//!   `ctx.wait_until()` while the cached data is returned immediately. On cache
//!   miss a live fetch is performed; if that also fails, [`fallback_stats`] is
//!   returned.

use leptos::prelude::*;

use crate::github::defaults::fallback_stats;
use crate::github::model::GitHubStats;

/// Newtype wrapper for the GitHub personal access token in Leptos context.
///
/// This is used to avoid ambiguity when storing a `String` in Leptos context,
/// ensuring only the GitHub token is retrieved and not any other `String` value.
#[derive(Clone)]
pub struct GitHubToken(pub String);

#[cfg(target_arch = "wasm32")]
const CACHE_TTL_SECS: i64 = 3600; // 1 hour

/// Fetches GitHub stats, using KV cache on CF Workers and direct API on Axum dev.
///
/// On native (Axum dev): calls the GitHub API directly; falls back to hardcoded
/// placeholder data if `GITHUB_TOKEN` is unset or the API call fails.
///
/// On WASM32 (CF Workers): reads from KV with stale-while-revalidate logic.
/// If the cached value is older than one hour, a background refresh is triggered
/// via `ctx.wait_until()` and the (slightly stale) data is returned immediately.
/// On cache miss, performs a live fetch and populates KV.
///
/// # Returns
///
/// A [`GitHubStats`] struct. Never returns `Err` to the client — all failure modes
/// fall back to placeholder data or the existing stale cache.
///
/// # Errors
///
/// Returns `Err(ServerFnError)` only if the Leptos context is missing required
/// values (KV store, CF context, or token). This indicates a configuration error
/// in the Workers fetch handler and should not occur in production.
#[server]
pub async fn get_github_stats() -> Result<GitHubStats, ServerFnError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::github::fetch::fetch_from_github;

        // Local Axum dev: call GitHub API directly, no caching.
        let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
        if token.is_empty() {
            leptos::logging::warn!("GITHUB_TOKEN not set; serving fallback stats");
            return Ok(fallback_stats());
        }
        match fetch_from_github(&token).await {
            Ok(stats) => Ok(stats),
            Err(e) => {
                leptos::logging::error!("GitHub fetch failed: {e}");
                Ok(fallback_stats())
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        cf_workers_get_stats().await
    }
}

#[cfg(target_arch = "wasm32")]
async fn cf_workers_get_stats() -> Result<GitHubStats, ServerFnError> {
    use chrono::Utc;
    use leptos::prelude::use_context;
    use worker::Context;
    use worker::kv::KvStore;

    use crate::github::fetch::fetch_from_github;

    let kv =
        use_context::<KvStore>().ok_or_else(|| ServerFnError::new("KV store not in context"))?;
    let ctx =
        use_context::<Context>().ok_or_else(|| ServerFnError::new("CF context not in context"))?;
    let token = use_context::<GitHubToken>()
        .ok_or_else(|| ServerFnError::new("GITHUB_TOKEN not in context"))?
        .0;

    match kv.get("stats").json::<GitHubStats>().await {
        Ok(Some(stats)) => {
            let age = Utc::now() - stats.fetched_at;
            if age.num_seconds() > CACHE_TTL_SECS {
                // Stale: serve immediately, refresh in background.
                let kv2 = kv.clone();
                let token2 = token.clone();
                ctx.wait_until(async move {
                    if let Ok(fresh) = fetch_from_github(&token2).await {
                        match serde_json::to_string(&fresh) {
                            Ok(json) => match kv2.put("stats", json) {
                                Ok(builder) => {
                                    let _ = builder.execute().await;
                                }
                                Err(e) => {
                                    leptos::logging::error!(
                                        "KV put setup failed in background: {e}"
                                    );
                                }
                            },
                            Err(e) => {
                                leptos::logging::error!(
                                    "Serialize failed in background refresh: {e}"
                                );
                            }
                        }
                    }
                });
            }
            Ok(stats)
        }
        // TODO: deduplicate – the Ok(None) and Err(_) arms share identical
        // cold-start logic. Extract into a helper once async fn items inside
        // cfg blocks are better supported in stable Rust.
        Ok(None) => {
            // Cache miss: block on live fetch (cold start).
            match fetch_from_github(&token).await {
                Ok(fresh) => {
                    match serde_json::to_string(&fresh) {
                        Ok(json) => match kv.put("stats", json) {
                            Ok(builder) => {
                                let _ = builder.execute().await;
                            }
                            Err(e) => {
                                leptos::logging::error!("KV put setup failed: {e}");
                            }
                        },
                        Err(e) => {
                            leptos::logging::error!("Serialize failed: {e}");
                        }
                    }
                    Ok(fresh)
                }
                Err(e) => {
                    leptos::logging::error!("Cold-start fetch failed: {e}");
                    Ok(fallback_stats())
                }
            }
        }
        Err(e) => {
            leptos::logging::warn!("KV deserialization error (treating as miss): {e}");
            // Cache miss: block on live fetch (cold start).
            match fetch_from_github(&token).await {
                Ok(fresh) => {
                    match serde_json::to_string(&fresh) {
                        Ok(json) => match kv.put("stats", json) {
                            Ok(builder) => {
                                let _ = builder.execute().await;
                            }
                            Err(e) => {
                                leptos::logging::error!("KV put setup failed: {e}");
                            }
                        },
                        Err(e) => {
                            leptos::logging::error!("Serialize failed: {e}");
                        }
                    }
                    Ok(fresh)
                }
                Err(e) => {
                    leptos::logging::error!("Cold-start fetch failed: {e}");
                    Ok(fallback_stats())
                }
            }
        }
    }
}
