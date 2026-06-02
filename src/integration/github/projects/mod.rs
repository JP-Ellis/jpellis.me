//! Per-repository statistics and activity fetched from the GitHub REST API.
#![expect(
    clippy::pub_use,
    reason = "module facade: re-exporting types for a clean public API surface"
)]
#![expect(
    clippy::module_name_repetitions,
    reason = "re-exported names include the module name for clarity at call sites"
)]

/// Default/fallback project stats when the API is unavailable.
pub mod defaults;
#[cfg(any(not(target_arch = "wasm32"), feature = "ssr"))]
/// Live data fetching from the GitHub REST API.
pub mod fetch;
/// Data model types for project/repository stats.
pub mod model;
/// Stats provider abstraction (cache + background refresh).
pub mod provider;
/// Leptos server function exposing project stats to the client.
pub mod server_fn;

pub use model::ProjectsStats;
pub use model::RepoStats;
pub use provider::ProjectsStatsProvider;
pub use server_fn::get_projects_stats;
