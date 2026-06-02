//! Platform-specific projects-stats providers.
//!
//! - **[`ProjectsStatsProvider`]** (native): reads/writes `./target/cache/projects-stats.json`.
//!   Serves stale data immediately; background-refreshes when older than 1 day.
//!   The hard TTL for a synchronous re-fetch is 7 days.
//!
//! - **[`ProjectsStatsProvider`]** (CF Workers): reads/writes the `PROJECTS_STATS` KV
//!   namespace with the same stale-while-revalidate logic.
//!
//! - **[`ProjectsStatsProvider::Fallback`]** (CF Workers, degraded): always returns
//!   an empty [`ProjectsStats`].  Used when the `PROJECTS_STATS` KV binding is
//!   unavailable so the site still renders rather than returning a 500.

use crate::integration::github::projects::model::ProjectsStats;

/// Abstracts over native-file and CF-Workers-KV projects-stats sources.
#[non_exhaustive]
#[derive(Clone)]
pub enum ProjectsStatsProvider {
    /// Local development: caches to `./target/cache/projects-stats.json`.
    #[cfg(not(target_arch = "wasm32"))]
    File(FileProjectsStatsProvider),

    /// Cloudflare Workers: caches in the `PROJECTS_STATS` KV namespace.
    #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
    Kv(KvProjectsStatsProvider),

    /// Degraded fallback when the `PROJECTS_STATS` KV binding is unavailable.
    /// Always returns an empty [`ProjectsStats`]; never constructed in a correctly
    /// configured deployment.
    #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
    Fallback,

    /// Placeholder for the browser hydrate build (never constructed at runtime).
    #[cfg(all(target_arch = "wasm32", not(feature = "ssr")))]
    #[doc(hidden)]
    _Unreachable,
}

impl ProjectsStatsProvider {
    /// Creates a file-backed provider for native Axum dev.
    #[must_use = "the constructed provider is discarded if not used"]
    #[inline]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn file(token: String) -> Self {
        Self::File(FileProjectsStatsProvider { token })
    }

    /// Creates a KV-backed provider for CF Workers production, or a fallback
    /// provider if the KV binding is unavailable.
    ///
    /// Accepts the raw `Result` from `env.kv()` so the caller never needs to
    /// match on it; binding errors are logged and handled here.  `token` is
    /// `None` when `GITHUB_TOKEN` is absent — the provider will still serve
    /// cached data but won't refresh it.
    #[must_use = "the constructed provider is discarded if not used"]
    #[inline]
    #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
    pub fn kv(
        kv: worker::Result<worker::kv::KvStore>,
        ctx: std::sync::Arc<worker::Context>,
        token: Option<&str>,
    ) -> Self {
        match kv {
            Ok(kv_store) => Self::Kv(KvProjectsStatsProvider {
                kv: kv_store,
                ctx,
                token: token.map(ToOwned::to_owned),
            }),
            Err(e) => {
                leptos::logging::error!(
                    "PROJECTS_STATS binding unavailable: {e}; using fallback projects stats"
                );
                Self::Fallback
            }
        }
    }

    /// Returns projects stats using the appropriate backing store.
    ///
    /// Never fails: on all error paths returns an empty [`ProjectsStats`] with no repos.
    #[must_use = "the fetched stats are discarded if not used"]
    #[inline]
    #[cfg_attr(
        all(target_arch = "wasm32", not(feature = "ssr")),
        expect(
            clippy::unused_async,
            reason = "async is required for API consistency across targets; hydrate build has no await points"
        )
    )]
    pub async fn get(&self) -> ProjectsStats {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::File(p) => p.get().await,

            #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
            Self::Kv(p) => p.get().await,

            #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
            Self::Fallback => {
                crate::integration::github::projects::defaults::fallback_projects_stats()
            }

            #[cfg(all(target_arch = "wasm32", not(feature = "ssr")))]
            Self::_Unreachable => {
                unreachable!("ProjectsStatsProvider::_Unreachable must never be constructed")
            }
        }
    }
}

// ─── FileProjectsStatsProvider ───────────────────────────────────────────────

/// Caches projects stats in `./target/cache/projects-stats.json`.
///
/// Hard TTL: 7 days. SWR threshold: 1 day.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct FileProjectsStatsProvider {
    /// GitHub personal access token used to authenticate API requests.
    token: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl FileProjectsStatsProvider {
    /// Path to the JSON cache file, relative to the workspace root.
    const CACHE_PATH: &'static str = "./target/cache/projects-stats.json";
    /// Stale-while-revalidate threshold: 1 day.
    const SWR_SECS: i64 = 86_400;
    /// Hard TTL: 7 days. Data older than this is re-fetched synchronously.
    const HARD_TTL_SECS: i64 = 604_800;

    /// Returns cached projects stats, refreshing in the background when stale.
    async fn get(&self) -> ProjectsStats {
        use chrono::Utc;

        use crate::config::projects::projects_config;
        use crate::integration::github::projects::fetch::fetch_projects_stats;

        let slugs = &projects_config().tracked_slugs;

        if let Ok(data) = tokio::fs::read_to_string(Self::CACHE_PATH).await
            && let Ok(stats) = serde_json::from_str::<ProjectsStats>(&data)
        {
            #[expect(
                clippy::arithmetic_side_effects,
                reason = "datetime subtraction; chrono Duration::num_seconds saturates rather than overflowing"
            )]
            let age = (Utc::now() - stats.fetched_at).num_seconds();
            if age < Self::HARD_TTL_SECS {
                if age >= Self::SWR_SECS && !self.token.is_empty() {
                    let token = self.token.clone();
                    let slugs_clone = slugs.clone();
                    tokio::task::spawn(async move {
                        let fresh = fetch_projects_stats(&token, &slugs_clone).await;
                        drop(Self::write_cache(&fresh).await);
                    });
                }
                return stats;
            }
        }

        if self.token.is_empty() {
            leptos::logging::warn!("GITHUB_TOKEN not set; serving empty projects stats");
            return empty_stats();
        }
        let fresh = fetch_projects_stats(&self.token, slugs).await;
        drop(Self::write_cache(&fresh).await);
        fresh
    }

    /// Serialises `stats` to JSON and writes it to the cache file.
    async fn write_cache(stats: &ProjectsStats) -> Result<(), Box<dyn std::error::Error>> {
        let dir = std::path::Path::new("./target/cache");
        tokio::fs::create_dir_all(dir).await?;
        let json = serde_json::to_string(stats)?;
        tokio::fs::write(Self::CACHE_PATH, json).await?;
        Ok(())
    }
}

// ─── KvProjectsStatsProvider ─────────────────────────────────────────────────

/// Caches projects stats in the `PROJECTS_STATS` Cloudflare KV namespace.
///
/// Hard TTL: 7 days. SWR threshold: 1 day.
#[cfg(all(target_arch = "wasm32", feature = "ssr"))]
#[derive(Clone)]
pub struct KvProjectsStatsProvider {
    /// The `PROJECTS_STATS` KV store binding.
    kv: worker::kv::KvStore,
    /// Worker execution context used to schedule background `waitUntil` tasks.
    ctx: std::sync::Arc<worker::Context>,
    /// `None` when `GITHUB_TOKEN` was absent at startup; cached data is still
    /// served but background/cold-start refreshes are skipped.
    token: Option<String>,
}

#[cfg(all(target_arch = "wasm32", feature = "ssr"))]
impl KvProjectsStatsProvider {
    /// Stale-while-revalidate threshold: 1 day.
    const SWR_SECS: i64 = 86_400;
    /// Hard TTL: 7 days. Data older than this is re-fetched synchronously.
    const HARD_TTL_SECS: i64 = 604_800;

    /// Returns projects stats from KV, refreshing in the background or synchronously when stale.
    pub(crate) async fn get(&self) -> ProjectsStats {
        use chrono::Utc;

        use crate::config::projects::projects_config;
        use crate::integration::github::projects::fetch::fetch_projects_stats;

        let slugs = projects_config().tracked_slugs.clone();

        match self.kv.get("projects-stats").json::<ProjectsStats>().await {
            Ok(Some(stats)) => {
                #[expect(
                    clippy::arithmetic_side_effects,
                    reason = "datetime subtraction; chrono Duration::num_seconds saturates rather than overflowing"
                )]
                let age = (Utc::now() - stats.fetched_at).num_seconds();
                if (Self::SWR_SECS..Self::HARD_TTL_SECS).contains(&age) {
                    if let Some(token) = self.token.clone() {
                        let kv2 = self.kv.clone();
                        self.ctx.wait_until(async move {
                            let fresh = fetch_projects_stats(&token, &slugs).await;
                            Self::write_kv_cache(&kv2, &fresh).await;
                        });
                    }
                } else if age >= Self::HARD_TTL_SECS {
                    return self.cold_fetch().await;
                }
                stats
            }
            Ok(None) => self.cold_fetch().await,
            Err(e) => {
                leptos::logging::warn!("KV projects-stats deserialization error: {e}");
                self.cold_fetch().await
            }
        }
    }

    /// Fetches projects stats from the GitHub API when the KV cache is absent or expired.
    async fn cold_fetch(&self) -> ProjectsStats {
        use crate::config::projects::projects_config;
        use crate::integration::github::projects::fetch::fetch_projects_stats;

        let Some(token) = &self.token else {
            leptos::logging::warn!("GITHUB_TOKEN not set; serving fallback projects stats");
            return crate::integration::github::projects::defaults::fallback_projects_stats();
        };

        let fresh = fetch_projects_stats(token, &projects_config().tracked_slugs).await;
        Self::write_kv_cache(&self.kv, &fresh).await;
        fresh
    }

    /// Serialises `stats` to JSON and writes it to the `PROJECTS_STATS` KV namespace.
    async fn write_kv_cache(kv: &worker::kv::KvStore, stats: &ProjectsStats) {
        match serde_json::to_string(stats) {
            Ok(json) => match kv.put("projects-stats", json) {
                Ok(builder) => {
                    if let Err(e) = builder.execute().await {
                        leptos::logging::error!("KV projects-stats write error: {e}");
                    }
                }
                Err(e) => leptos::logging::error!("KV projects-stats put setup failed: {e}"),
            },
            Err(e) => leptos::logging::error!("projects-stats serialise failed: {e}"),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
/// Returns an empty [`ProjectsStats`] for use when no token is configured.
fn empty_stats() -> ProjectsStats {
    ProjectsStats {
        fetched_at: chrono::Utc::now(),
        repos: vec![],
    }
}
