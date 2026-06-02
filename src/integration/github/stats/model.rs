use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

/// GitHub statistics including contributions and recent activity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GitHubStats {
    /// Timestamp when the stats were fetched.
    pub fetched_at: DateTime<Utc>,
    /// Total number of contributions in the period (all types including reviews).
    pub total_contributions: u32,
    /// Number of commits in the period.
    pub commit_contributions: u32,
    /// Number of pull requests opened in the period.
    pub pr_contributions: u32,
    /// Number of issues opened in the period.
    pub issue_contributions: u32,
    /// Number of public repositories.
    pub public_repos: u32,
    /// Start date of the contribution period.
    pub period_from: NaiveDate,
    /// End date of the contribution period.
    pub period_to: NaiveDate,
    /// Contribution data by week.
    pub contribution_weeks: Vec<ContributionWeek>,
    /// Recent activity items.
    pub recent_activity: Vec<ActivityItem>,
}

/// A week of contribution data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContributionWeek {
    /// Daily contribution counts for the week.
    pub days: Vec<ContributionDay>,
}

/// A single day's contribution count.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContributionDay {
    /// Date of the contribution.
    pub date: NaiveDate,
    /// Number of contributions on this day.
    pub count: u32,
}

/// An activity item (commit, PR, or issue).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActivityItem {
    /// Type of activity.
    pub kind: ActivityKind,
    /// Repository name.
    pub repo: String,
    /// Activity title (commit message, PR title, etc.).
    pub title: String,
    /// URL to the activity.
    pub url: String,
    /// State of the activity (if applicable).
    pub state: Option<ActivityState>,
    /// When the activity was created.
    pub created_at: DateTime<Utc>,
}

/// Type of GitHub activity.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ActivityKind {
    /// A commit to a repository.
    Commit,
    /// A pull request.
    PullRequest,
    /// An issue.
    Issue,
}

/// State of an activity item.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ActivityState {
    /// The activity is open.
    Open,
    /// The activity is closed.
    Closed,
    /// The activity is merged (for PRs).
    Merged,
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone as _;
    use pretty_assertions::assert_eq;

    use super::*;

    fn sample_stats() -> GitHubStats {
        GitHubStats {
            fetched_at: Utc
                .with_ymd_and_hms(2026, 4, 28, 10, 0, 0)
                .single()
                .expect("valid test date literal"),
            total_contributions: 1247,
            commit_contributions: 950,
            pr_contributions: 45,
            issue_contributions: 32,
            public_repos: 14,
            period_from: NaiveDate::from_ymd_opt(2025, 4, 28).expect("valid test date literal"),
            period_to: NaiveDate::from_ymd_opt(2026, 4, 28).expect("valid test date literal"),
            contribution_weeks: vec![ContributionWeek {
                days: vec![ContributionDay {
                    date: NaiveDate::from_ymd_opt(2025, 4, 28).expect("valid test date literal"),
                    count: 3,
                }],
            }],
            recent_activity: vec![ActivityItem {
                kind: ActivityKind::Commit,
                repo: "pact-python".to_owned(),
                title: "feat: bind verifier results".to_owned(),
                url: "https://github.com/JP-Ellis/pact-python/commit/abc".to_owned(),
                state: None,
                created_at: Utc
                    .with_ymd_and_hms(2026, 4, 28, 6, 0, 0)
                    .single()
                    .expect("valid test date literal"),
            }],
        }
    }

    #[test]
    #[expect(clippy::unwrap_used, reason = "test helper — panicking is acceptable")]
    fn round_trips_through_json() {
        let stats = sample_stats();
        let json = serde_json::to_string(&stats).unwrap();
        let decoded: GitHubStats = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, stats);
    }

    #[test]
    #[expect(clippy::unwrap_used, reason = "test helper — panicking is acceptable")]
    fn activity_state_serialises_to_snake_case() {
        let item = ActivityItem {
            kind: ActivityKind::PullRequest,
            repo: "repo".to_owned(),
            title: "title".to_owned(),
            url: "url".to_owned(),
            state: Some(ActivityState::Merged),
            created_at: Utc
                .with_ymd_and_hms(2026, 4, 28, 6, 0, 0)
                .single()
                .expect("valid test date literal"),
        };
        let json = serde_json::to_string(&item).unwrap();
        let decoded: ActivityItem = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.kind, ActivityKind::PullRequest);
        assert_eq!(decoded.state, Some(ActivityState::Merged));
    }
}
