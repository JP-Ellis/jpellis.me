//! GitHub statistics data model and API integration.

pub mod defaults;
#[cfg(any(not(target_arch = "wasm32"), feature = "ssr"))]
pub mod fetch;
pub mod model;
pub mod provider;
pub mod server_fn;

pub use defaults::fallback_stats;
pub use model::ActivityKind;
pub use model::ActivityState;
pub use model::GitHubStats;
pub use provider::StatsProvider;
