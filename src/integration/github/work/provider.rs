//! Platform-specific work-stats providers.
//!
//! - **[`WorkStatsProvider`]** (native): reads/writes `./target/cache/work-stats.json`.
//!   Serves stale data immediately; background-refreshes when older than 1 day.
//!   The hard TTL for a synchronous re-fetch is 7 days.
//!
//! - **[`WorkStatsProvider`]** (CF Workers): reads/writes the `WORK_STATS` KV
//!   namespace with the same stale-while-revalidate logic.
//!
//! - **[`WorkStatsProvider::Fallback`]** (CF Workers, degraded): always returns
//!   an empty [`WorkStats`].  Used when the `WORK_STATS` KV binding is
//!   unavailable so the site still renders rather than returning a 500.

use crate::integration::github::work::model::WorkStats;

/// Abstracts over native-file and CF-Workers-KV work-stats sources.
#[derive(Clone)]
pub enum WorkStatsProvider {
    /// Local development: caches to `./target/cache/work-stats.json`.
    #[cfg(not(target_arch = "wasm32"))]
    File(FileWorkStatsProvider),

    /// Cloudflare Workers: caches in the `WORK_STATS` KV namespace.
    #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
    Kv(KvWorkStatsProvider),

    /// Degraded fallback when the `WORK_STATS` KV binding is unavailable.
    /// Always returns an empty [`WorkStats`]; never constructed in a correctly
    /// configured deployment.
    #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
    Fallback,

    /// Placeholder for the browser hydrate build (never constructed at runtime).
    #[cfg(all(target_arch = "wasm32", not(feature = "ssr")))]
    #[doc(hidden)]
    _Unreachable,
}

impl WorkStatsProvider {
    /// Creates a file-backed provider for native Axum dev.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn file(token: String) -> Self {
        Self::File(FileWorkStatsProvider { token })
    }

    /// Creates a KV-backed provider for CF Workers production, or a fallback
    /// provider if the KV binding is unavailable.
    ///
    /// Accepts the raw `Result` from `env.kv()` so the caller never needs to
    /// match on it; binding errors are logged and handled here.  `token` is
    /// `None` when `GITHUB_TOKEN` is absent — the provider will still serve
    /// cached data but won't refresh it.
    #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
    pub fn kv(
        kv: worker::Result<worker::kv::KvStore>,
        ctx: std::sync::Arc<worker::Context>,
        token: Option<&str>,
    ) -> Self {
        match kv {
            Ok(kv) => Self::Kv(KvWorkStatsProvider {
                kv,
                ctx,
                token: token.map(ToOwned::to_owned),
            }),
            Err(e) => {
                leptos::logging::error!(
                    "WORK_STATS binding unavailable: {e}; using fallback work stats"
                );
                Self::Fallback
            }
        }
    }

    /// Returns work stats using the appropriate backing store.
    ///
    /// Never fails: on all error paths returns an empty [`WorkStats`] with no repos.
    pub async fn get(&self) -> WorkStats {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::File(p) => p.get().await,

            #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
            Self::Kv(p) => p.get().await,

            #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
            Self::Fallback => crate::integration::github::work::defaults::fallback_work_stats(),

            #[cfg(all(target_arch = "wasm32", not(feature = "ssr")))]
            Self::_Unreachable => {
                unreachable!("WorkStatsProvider::_Unreachable must never be constructed")
            }
        }
    }
}

// ─── FileWorkStatsProvider ───────────────────────────────────────────────────

/// Caches work stats in `./target/cache/work-stats.json`.
///
/// Hard TTL: 7 days. SWR threshold: 1 day.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct FileWorkStatsProvider {
    token: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl FileWorkStatsProvider {
    const CACHE_PATH: &'static str = "./target/cache/work-stats.json";
    /// Stale-while-revalidate threshold: 1 day.
    const SWR_SECS: i64 = 86_400;
    /// Hard TTL: 7 days. Data older than this is re-fetched synchronously.
    const HARD_TTL_SECS: i64 = 604_800;

    async fn get(&self) -> WorkStats {
        use chrono::Utc;

        use crate::config::work::work_config;
        use crate::integration::github::work::fetch::fetch_work_stats;

        let slugs = &work_config().tracked_slugs;

        if let Ok(data) = tokio::fs::read_to_string(Self::CACHE_PATH).await
            && let Ok(stats) = serde_json::from_str::<WorkStats>(&data)
        {
            let age = (Utc::now() - stats.fetched_at).num_seconds();
            if age < Self::HARD_TTL_SECS {
                if age >= Self::SWR_SECS && !self.token.is_empty() {
                    let token = self.token.clone();
                    let slugs = slugs.clone();
                    tokio::task::spawn(async move {
                        let fresh = fetch_work_stats(&token, &slugs).await;
                        let _ = Self::write_cache(&fresh).await;
                    });
                }
                return stats;
            }
        }

        if self.token.is_empty() {
            leptos::logging::warn!("GITHUB_TOKEN not set; serving empty work stats");
            return empty_stats();
        }
        let fresh = fetch_work_stats(&self.token, slugs).await;
        let _ = Self::write_cache(&fresh).await;
        fresh
    }

    async fn write_cache(stats: &WorkStats) -> Result<(), Box<dyn std::error::Error>> {
        let dir = std::path::Path::new("./target/cache");
        tokio::fs::create_dir_all(dir).await?;
        let json = serde_json::to_string(stats)?;
        tokio::fs::write(Self::CACHE_PATH, json).await?;
        Ok(())
    }
}

// ─── KvWorkStatsProvider ─────────────────────────────────────────────────────

/// Caches work stats in the `WORK_STATS` Cloudflare KV namespace.
///
/// Hard TTL: 7 days. SWR threshold: 1 day.
#[cfg(all(target_arch = "wasm32", feature = "ssr"))]
#[derive(Clone)]
pub struct KvWorkStatsProvider {
    kv: worker::kv::KvStore,
    ctx: std::sync::Arc<worker::Context>,
    /// `None` when `GITHUB_TOKEN` was absent at startup; cached data is still
    /// served but background/cold-start refreshes are skipped.
    token: Option<String>,
}

#[cfg(all(target_arch = "wasm32", feature = "ssr"))]
impl KvWorkStatsProvider {
    const SWR_SECS: i64 = 86_400;
    const HARD_TTL_SECS: i64 = 604_800;

    pub(crate) async fn get(&self) -> WorkStats {
        use chrono::Utc;

        use crate::config::work::work_config;
        use crate::integration::github::work::fetch::fetch_work_stats;

        let slugs = work_config().tracked_slugs.clone();

        match self.kv.get("work-stats").json::<WorkStats>().await {
            Ok(Some(stats)) => {
                let age = (Utc::now() - stats.fetched_at).num_seconds();
                if age >= Self::SWR_SECS && age < Self::HARD_TTL_SECS {
                    if let Some(token) = self.token.clone() {
                        let kv2 = self.kv.clone();
                        self.ctx.wait_until(async move {
                            let fresh = fetch_work_stats(&token, &slugs).await;
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
                leptos::logging::warn!("KV work-stats deserialization error: {e}");
                self.cold_fetch().await
            }
        }
    }

    async fn cold_fetch(&self) -> WorkStats {
        use crate::config::work::work_config;
        use crate::integration::github::work::fetch::fetch_work_stats;

        let Some(token) = &self.token else {
            leptos::logging::warn!("GITHUB_TOKEN not set; serving fallback work stats");
            return crate::integration::github::work::defaults::fallback_work_stats();
        };

        let fresh = fetch_work_stats(token, &work_config().tracked_slugs).await;
        Self::write_kv_cache(&self.kv, &fresh).await;
        fresh
    }

    async fn write_kv_cache(kv: &worker::kv::KvStore, stats: &WorkStats) {
        match serde_json::to_string(stats) {
            Ok(json) => match kv.put("work-stats", json) {
                Ok(builder) => {
                    if let Err(e) = builder.execute().await {
                        leptos::logging::error!("KV work-stats write error: {e}");
                    }
                }
                Err(e) => leptos::logging::error!("KV work-stats put setup failed: {e}"),
            },
            Err(e) => leptos::logging::error!("work-stats serialise failed: {e}"),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn empty_stats() -> WorkStats {
    WorkStats {
        fetched_at: chrono::Utc::now(),
        repos: vec![],
    }
}
