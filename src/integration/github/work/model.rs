use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

/// Stars and forks for a single GitHub repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepoStats {
    /// Repository slug, e.g. `"pact-foundation/pact-python"`.
    pub slug: String,
    /// Number of GitHub stars.
    pub stars: u32,
    /// Number of GitHub forks.
    pub forks: u32,
}

/// Cached per-repository star and fork counts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkStats {
    /// Timestamp when the stats were fetched.
    pub fetched_at: DateTime<Utc>,
    /// Per-repository stats, one entry per tracked slug.
    pub repos: Vec<RepoStats>,
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use pretty_assertions::assert_eq;

    use super::*;

    fn sample() -> WorkStats {
        WorkStats {
            fetched_at: Utc
                .with_ymd_and_hms(2026, 4, 30, 12, 0, 0)
                .single()
                .expect("valid test date"),
            repos: vec![
                RepoStats {
                    slug: "JP-Ellis/tikz-feynman".to_string(),
                    stars: 158,
                    forks: 22,
                },
                RepoStats {
                    slug: "pact-foundation/pact-python".to_string(),
                    stars: 664,
                    forks: 148,
                },
            ],
        }
    }

    #[test]
    fn round_trips_through_json() {
        let original = sample();
        let json = serde_json::to_string(&original).expect("serialize");
        let decoded: WorkStats = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded, original);
    }

    #[test]
    fn repo_stats_preserves_zero_counts() {
        let repo = RepoStats {
            slug: "JP-Ellis/dotfiles".to_string(),
            stars: 2,
            forks: 0,
        };
        let json = serde_json::to_string(&repo).expect("serialize");
        let decoded: RepoStats = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded.forks, 0);
    }
}
