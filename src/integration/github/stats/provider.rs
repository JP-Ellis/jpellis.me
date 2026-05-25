//! Platform-specific GitHub statistics providers.
//!
//! Provides a [`StatsProvider`] enum that abstracts over two implementations:
//!
//! - **[`FileStatsProvider`]** (native): reads from and writes to
//!   `./target/cache/github-stats.json`.  Serves stale data immediately and
//!   spawns a background refresh when the cache is more than one hour old.
//!
//! - **[`KvStatsProvider`]** (CF Workers): reads from and writes to the
//!   `GITHUB_STATS` KV namespace with the same stale-while-revalidate logic,
//!   using `ctx.wait_until()` for the background refresh.

use crate::integration::github::stats::model::GitHubStats;

/// Abstracts over native-file and CF-Workers-KV stats sources.
///
/// Inject into Leptos context so that [`get_github_stats`] is cfg-free.
///
/// [`get_github_stats`]: crate::integration::github::stats::server_fn::get_github_stats
#[derive(Clone)]
pub enum StatsProvider {
    /// Local development: caches stats to `./target/cache/github-stats.json`.
    #[cfg(not(target_arch = "wasm32"))]
    File(FileStatsProvider),

    /// Cloudflare Workers production: caches stats in the `GITHUB_STATS` KV
    /// namespace.
    #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
    Kv(KvStatsProvider),

    /// Placeholder variant so the enum is non-empty in the browser hydrate
    /// build, where neither `File` nor `Kv` are compiled in.  This variant is
    /// never constructed or matched at runtime.
    #[cfg(all(target_arch = "wasm32", not(feature = "ssr")))]
    #[doc(hidden)]
    _Unreachable,
}

impl StatsProvider {
    /// Creates a file-backed provider for native Axum dev.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn file(token: String) -> Self {
        Self::File(FileStatsProvider { token })
    }

    /// Creates a KV-backed provider for CF Workers production.
    #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
    pub fn kv(
        kv: worker::kv::KvStore,
        ctx: std::sync::Arc<worker::Context>,
        token: String,
    ) -> Self {
        Self::Kv(KvStatsProvider { kv, ctx, token })
    }

    /// Returns GitHub stats using the appropriate backing store.
    ///
    /// Never fails: all error paths fall back to hardcoded placeholder data.
    pub async fn get(&self) -> GitHubStats {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Self::File(p) => p.get().await,

            #[cfg(all(target_arch = "wasm32", feature = "ssr"))]
            Self::Kv(p) => p.get().await,

            #[cfg(all(target_arch = "wasm32", not(feature = "ssr")))]
            Self::_Unreachable => {
                unreachable!("StatsProvider::_Unreachable must never be constructed")
            }
        }
    }
}

// ─── FileStatsProvider ───────────────────────────────────────────────────────

/// Caches GitHub stats in `./target/cache/github-stats.json`.
///
/// Serves stale data immediately (stale-while-revalidate) and spawns a
/// background Tokio task when the cache is older than one hour.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct FileStatsProvider {
    token: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl FileStatsProvider {
    const CACHE_PATH: &'static str = "./target/cache/github-stats.json";
    const TTL_SECS: i64 = 3600;

    async fn get(&self) -> GitHubStats {
        use chrono::Utc;

        use crate::integration::github::stats::defaults::fallback_stats;
        use crate::integration::github::stats::fetch::fetch_from_github;

        if let Ok(data) = tokio::fs::read_to_string(Self::CACHE_PATH).await
            && let Ok(stats) = serde_json::from_str::<GitHubStats>(&data)
        {
            let age = (Utc::now() - stats.fetched_at).num_seconds();
            if age < Self::TTL_SECS {
                return stats;
            }
            // Stale: serve now, refresh in background.
            if !self.token.is_empty() {
                let token = self.token.clone();
                tokio::task::spawn(async move {
                    if let Ok(fresh) = fetch_from_github(&token).await {
                        let _ = Self::write_cache(&fresh).await;
                    }
                });
            }
            return stats;
        }
        // Cold start.
        if self.token.is_empty() {
            leptos::logging::warn!("GITHUB_TOKEN not set; serving fallback stats");
            return fallback_stats();
        }
        match fetch_from_github(&self.token).await {
            Ok(fresh) => {
                let _ = Self::write_cache(&fresh).await;
                fresh
            }
            Err(e) => {
                leptos::logging::error!("GitHub fetch failed: {e}");
                fallback_stats()
            }
        }
    }

    async fn write_cache(stats: &GitHubStats) -> Result<(), Box<dyn std::error::Error>> {
        let dir = std::path::Path::new("./target/cache");
        tokio::fs::create_dir_all(dir).await?;
        let json = serde_json::to_string(stats)?;
        tokio::fs::write(Self::CACHE_PATH, json).await?;
        Ok(())
    }
}

// ─── KvStatsProvider ─────────────────────────────────────────────────────────

/// Caches GitHub stats in the `GITHUB_STATS` Cloudflare KV namespace.
///
/// Uses `ctx.wait_until()` for the background stale-while-revalidate refresh.
#[cfg(all(target_arch = "wasm32", feature = "ssr"))]
#[derive(Clone)]
pub struct KvStatsProvider {
    kv: worker::kv::KvStore,
    /// Wrapped in `Arc` because `worker::Context` does not implement `Clone`.
    ctx: std::sync::Arc<worker::Context>,
    token: String,
}

#[cfg(all(target_arch = "wasm32", feature = "ssr"))]
impl KvStatsProvider {
    const TTL_SECS: i64 = 3600;

    pub(crate) async fn get(&self) -> GitHubStats {
        use chrono::Utc;

        use crate::integration::github::stats::fetch::fetch_from_github;

        match self.kv.get("stats").json::<GitHubStats>().await {
            Ok(Some(stats)) => {
                let age = (Utc::now() - stats.fetched_at).num_seconds();
                if age > Self::TTL_SECS {
                    let kv2 = self.kv.clone();
                    let token2 = self.token.clone();
                    self.ctx.wait_until(async move {
                        if let Ok(fresh) = fetch_from_github(&token2).await {
                            Self::write_kv_cache(&kv2, &fresh).await;
                        }
                    });
                }
                stats
            }
            Ok(None) => self.cold_fetch().await,
            Err(e) => {
                leptos::logging::warn!("KV deserialization error (treating as miss): {e}");
                self.cold_fetch().await
            }
        }
    }

    async fn cold_fetch(&self) -> GitHubStats {
        use crate::integration::github::stats::defaults::fallback_stats;
        use crate::integration::github::stats::fetch::fetch_from_github;

        match fetch_from_github(&self.token).await {
            Ok(fresh) => {
                Self::write_kv_cache(&self.kv, &fresh).await;
                fresh
            }
            Err(e) => {
                leptos::logging::error!("Cold-start fetch failed: {e}");
                fallback_stats()
            }
        }
    }

    async fn write_kv_cache(kv: &worker::kv::KvStore, stats: &GitHubStats) {
        match serde_json::to_string(stats) {
            Ok(json) => match kv.put("stats", json) {
                Ok(builder) => {
                    if let Err(e) = builder.execute().await {
                        leptos::logging::error!("KV write error: {e}");
                    }
                }
                Err(e) => leptos::logging::error!("KV put setup failed: {e}"),
            },
            Err(e) => leptos::logging::error!("Serialise failed: {e}"),
        }
    }
}
