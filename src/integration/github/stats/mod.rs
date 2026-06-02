//! GitHub statistics data model and API integration.
#![expect(
    clippy::pub_use,
    reason = "module facade: re-exporting types for a clean public API surface"
)]
#![expect(
    clippy::module_name_repetitions,
    reason = "re-exported names include the module name for clarity at call sites"
)]

/// Default/fallback GitHub stats when the API is unavailable.
pub mod defaults;
#[cfg(any(not(target_arch = "wasm32"), feature = "ssr"))]
/// Live data fetching from the GitHub GraphQL and REST APIs.
pub mod fetch;
/// Data model types for GitHub stats.
pub mod model;
/// Stats provider abstraction (cache + background refresh).
pub mod provider;
/// Leptos server function exposing GitHub stats to the client.
pub mod server_fn;

pub use defaults::fallback_stats;
pub use model::ActivityItem;
pub use model::ActivityKind;
pub use model::ActivityState;
pub use model::GitHubStats;
pub use provider::StatsProvider;
