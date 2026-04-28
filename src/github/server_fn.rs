//! Server function for GitHub statistics.
//!
//! Fetches [`GitHubStats`] via a [`StatsProvider`] stored in Leptos context.
//! The provider is injected by the application entry point (`main.rs` on
//! native, `workers.rs` on CF Workers), so this function contains no
//! platform-specific logic beyond the `SendWrapper` on WASM SSR (where KV
//! futures are `!Send` because of `JsFuture`, but the runtime is always
//! single-threaded so `SendWrapper` is safe).

use leptos::prelude::*;

use crate::github::model::GitHubStats;
#[cfg(any(not(target_arch = "wasm32"), feature = "ssr"))]
use crate::github::provider::StatsProvider;

/// Fetches GitHub stats using the [`StatsProvider`] from Leptos context.
///
/// # Errors
///
/// Returns `Err` only when the `StatsProvider` is absent from Leptos context.
/// All other failure modes (missing token, API errors, KV errors) are handled
/// inside the provider and fall back to [`crate::github::defaults::fallback_stats`].
#[server]
pub async fn get_github_stats() -> Result<GitHubStats, ServerFnError> {
    let provider = use_context::<StatsProvider>()
        .ok_or_else(|| ServerFnError::new("StatsProvider not in Leptos context"))?;

    // On CF Workers (wasm32 + ssr), KV operations use JsFuture which is !Send.
    // SendWrapper makes the future Send; this is safe because the runtime is
    // single-threaded.
    #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
    let stats = send_wrapper::SendWrapper::new(provider.get()).await;

    #[cfg(not(all(target_arch = "wasm32", feature = "ssr")))]
    let stats = provider.get().await;

    Ok(stats)
}
