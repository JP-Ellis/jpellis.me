pub mod github;

pub use github::stats::defaults::fallback_stats;
pub use github::stats::model::ActivityItem;
pub use github::stats::model::ActivityKind;
pub use github::stats::model::ActivityState;
pub use github::stats::model::GitHubStats;
pub use github::stats::provider::StatsProvider;
pub use github::stats::server_fn::get_github_stats;
