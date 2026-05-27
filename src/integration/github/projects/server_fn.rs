//! Server function for projects statistics.

use leptos::prelude::*;

use crate::integration::github::projects::model::ProjectsStats;
#[cfg(any(not(target_arch = "wasm32"), feature = "ssr"))]
use crate::integration::github::projects::provider::ProjectsStatsProvider;

/// Fetches projects stats using the [`ProjectsStatsProvider`] from Leptos context.
///
/// # Errors
///
/// Returns `Err` only when `ProjectsStatsProvider` is absent from Leptos context.
/// All other failure modes fall back to an empty [`ProjectsStats`] inside the provider.
#[server]
pub async fn get_projects_stats() -> Result<ProjectsStats, ServerFnError> {
    let provider = use_context::<ProjectsStatsProvider>()
        .ok_or_else(|| ServerFnError::new("ProjectsStatsProvider not in Leptos context"))?;

    #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
    let stats = send_wrapper::SendWrapper::new(provider.get()).await;

    #[cfg(not(all(target_arch = "wasm32", feature = "ssr")))]
    let stats = provider.get().await;

    Ok(stats)
}
