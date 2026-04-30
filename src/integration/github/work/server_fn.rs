//! Server function for work statistics.

use leptos::prelude::*;

use crate::integration::github::work::model::WorkStats;
#[cfg(any(not(target_arch = "wasm32"), feature = "ssr"))]
use crate::integration::github::work::provider::WorkStatsProvider;

/// Fetches work stats using the [`WorkStatsProvider`] from Leptos context.
///
/// # Errors
///
/// Returns `Err` only when `WorkStatsProvider` is absent from Leptos context.
/// All other failure modes fall back to an empty [`WorkStats`] inside the provider.
#[server]
pub async fn get_work_stats() -> Result<WorkStats, ServerFnError> {
    let provider = use_context::<WorkStatsProvider>()
        .ok_or_else(|| ServerFnError::new("WorkStatsProvider not in Leptos context"))?;

    #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
    let stats = send_wrapper::SendWrapper::new(provider.get()).await;

    #[cfg(not(all(target_arch = "wasm32", feature = "ssr")))]
    let stats = provider.get().await;

    Ok(stats)
}
