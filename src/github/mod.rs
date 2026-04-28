//! GitHub statistics data model and API integration.

pub mod defaults;
pub mod fetch;
pub mod model;
pub mod server_fn;

pub use defaults::fallback_stats;
pub use model::ActivityItem;
pub use model::ActivityKind;
pub use model::ActivityState;
pub use model::ContributionDay;
pub use model::ContributionWeek;
pub use model::GitHubStats;
#[cfg(target_arch = "wasm32")]
pub use server_fn::GitHubToken;
