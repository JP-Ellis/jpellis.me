#[cfg(any(not(target_arch = "wasm32"), feature = "ssr"))]
pub mod fetch;
pub mod model;

pub use model::RepoStats;
pub use model::WorkStats;
