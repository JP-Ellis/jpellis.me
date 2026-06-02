use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

/// Latest release info for a repository.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseInfo {
    /// Release tag name, e.g. `"v3.1.0"`.
    pub tag: String,
    /// ISO 8601 publication timestamp, e.g. `"2025-01-14T10:00:00Z"`.
    pub date: String,
    /// GitHub release URL.
    pub url: String,
}

/// A single (non-bot) commit for a repository.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitInfo {
    /// 7-character short SHA.
    pub sha: String,
    /// First line of the commit message, truncated at 72 chars.
    pub message: String,
    /// ISO 8601 commit timestamp.
    pub date: String,
    /// Commit author display name (bots excluded upstream).
    pub author: String,
    /// GitHub commit URL.
    pub url: String,
}

/// Stars, forks, and activity for a single GitHub repository.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepoStats {
    /// Repository slug, e.g. `"JP-Ellis/tikz-feynman"`.
    pub slug: String,
    /// Number of GitHub stars.
    pub stars: u32,
    /// Number of GitHub forks.
    pub forks: u32,
    /// Number of open issues.
    #[serde(default)]
    pub open_issues: u32,
    /// Number of watchers.
    #[serde(default)]
    pub watchers: u32,
    /// Latest release, or `None` if the repo has no releases.
    #[serde(default)]
    pub latest_release: Option<ReleaseInfo>,
    /// Recent non-bot commits, capped at 5.
    #[serde(default)]
    pub recent_commits: Vec<CommitInfo>,
    /// Number of open pull requests (capped at 100).
    #[serde(default)]
    pub open_prs: u32,
}

/// Cached per-repository stats.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectsStats {
    /// Timestamp when the stats were fetched.
    pub fetched_at: DateTime<Utc>,
    /// Per-repository stats, one entry per tracked slug.
    pub repos: Vec<RepoStats>,
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone as _;
    use pretty_assertions::assert_eq;

    use super::*;

    fn sample() -> ProjectsStats {
        ProjectsStats {
            fetched_at: Utc
                .with_ymd_and_hms(2026, 4, 30, 12, 0, 0)
                .single()
                .expect("valid test date"),
            repos: vec![
                RepoStats {
                    slug: "JP-Ellis/tikz-feynman".to_owned(),
                    stars: 158,
                    forks: 22,
                    open_issues: 5,
                    watchers: 12,
                    latest_release: Some(ReleaseInfo {
                        tag: "v3.1.0".to_owned(),
                        date: "2025-01-14T10:00:00Z".to_owned(),
                        url: "https://github.com/JP-Ellis/tikz-feynman/releases/tag/v3.1.0"
                            .to_owned(),
                    }),
                    recent_commits: vec![CommitInfo {
                        sha: "a1b2c3d".to_owned(),
                        message: "Fix diagram spacing".to_owned(),
                        date: "2025-05-01T09:00:00Z".to_owned(),
                        author: "JP-Ellis".to_owned(),
                        url: "https://github.com/JP-Ellis/tikz-feynman/commit/a1b2c3d".to_owned(),
                    }],
                    open_prs: 3,
                },
                RepoStats {
                    slug: "pact-foundation/pact-python".to_owned(),
                    stars: 664,
                    forks: 148,
                    open_issues: 12,
                    watchers: 20,
                    latest_release: None,
                    recent_commits: vec![],
                    open_prs: 0,
                },
            ],
        }
    }

    #[test]
    fn round_trips_through_json() {
        let original = sample();
        let json = serde_json::to_string(&original).expect("serialize");
        let decoded: ProjectsStats = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded, original);
    }

    #[test]
    fn repo_stats_preserves_zero_counts() {
        let repo = RepoStats {
            slug: "JP-Ellis/dotfiles".to_owned(),
            stars: 2,
            forks: 0,
            open_issues: 0,
            watchers: 1,
            latest_release: None,
            recent_commits: vec![],
            open_prs: 0,
        };
        let json = serde_json::to_string(&repo).expect("serialize");
        let decoded: RepoStats = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded.forks, 0);
        assert_eq!(decoded.latest_release, None);
    }

    #[test]
    fn repo_stats_round_trips_with_activity_fields() {
        let repo = RepoStats {
            slug: "JP-Ellis/tikz-feynman".to_owned(),
            stars: 158,
            forks: 22,
            open_issues: 5,
            watchers: 12,
            latest_release: Some(ReleaseInfo {
                tag: "v3.1.0".to_owned(),
                date: "2025-01-14T10:00:00Z".to_owned(),
                url: "https://github.com/JP-Ellis/tikz-feynman/releases/tag/v3.1.0".to_owned(),
            }),
            recent_commits: vec![CommitInfo {
                sha: "a1b2c3d".to_owned(),
                message: "Fix diagram spacing".to_owned(),
                date: "2025-05-01T09:00:00Z".to_owned(),
                author: "JP-Ellis".to_owned(),
                url: "https://github.com/JP-Ellis/tikz-feynman/commit/a1b2c3d".to_owned(),
            }],
            open_prs: 3,
        };
        let json = serde_json::to_string(&repo).expect("serialize");
        let decoded: RepoStats = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded, repo);
    }

    #[test]
    fn repo_stats_no_release_round_trips() {
        let repo = RepoStats {
            slug: "JP-Ellis/rust-skiplist".to_owned(),
            stars: 10,
            forks: 2,
            open_issues: 0,
            watchers: 1,
            latest_release: None,
            recent_commits: vec![],
            open_prs: 0,
        };
        let json = serde_json::to_string(&repo).expect("serialize");
        let decoded: RepoStats = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded.latest_release, None);
        assert_eq!(decoded.recent_commits, vec![]);
    }
}
