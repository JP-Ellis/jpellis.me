#[cfg(any(not(target_arch = "wasm32"), feature = "ssr"))]
pub mod fetch;
pub mod model;
pub mod provider;
pub mod server_fn;

pub use model::RepoStats;
pub use model::WorkStats;
pub use provider::WorkStatsProvider;
pub use server_fn::get_work_stats;
