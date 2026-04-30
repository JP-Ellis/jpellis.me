use std::sync::OnceLock;

use serde::Deserialize;

static CACHE: OnceLock<WorkConfig> = OnceLock::new();

/// Configuration for the work page, loaded from `work.toml`.
#[derive(Debug, Deserialize)]
pub struct WorkConfig {
    /// GitHub slugs to track for star/fork counts.
    /// Must cover every `ProjectLink::GitHub` entry in `pages::work::PROJECTS`.
    pub tracked_slugs: Vec<String>,
    /// Misc OSS contributions shown at the bottom of the work page.
    #[serde(default)]
    pub oss_contribs: Vec<OssContrib>,
}

/// A single misc OSS contribution entry.
#[derive(Debug, Clone, Deserialize)]
pub struct OssContrib {
    /// GitHub slug, e.g. `"home-assistant/core"`.
    pub slug: String,
    /// Short display name, e.g. `"home-assistant"`.
    pub name: String,
    /// GitHub star count at the time the script last ran. Zero if unknown.
    #[serde(default)]
    pub stars: u64,
}

const WORK_CONFIG_TOML: &str = include_str!("work.toml");

/// Returns the parsed work configuration, initialised once.
pub fn work_config() -> &'static WorkConfig {
    CACHE.get_or_init(|| {
        toml::from_str(WORK_CONFIG_TOML).expect("src/config/work.toml is not valid TOML")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracked_slugs_are_valid_format() {
        for slug in &work_config().tracked_slugs {
            assert!(!slug.is_empty(), "empty tracked slug");
            assert!(slug.contains('/'), "tracked slug missing '/': {slug}");
        }
    }

    #[test]
    fn oss_contribs_slugs_are_valid_format() {
        for c in &work_config().oss_contribs {
            assert!(!c.slug.is_empty(), "empty oss_contrib slug");
            assert!(
                c.slug.contains('/'),
                "oss_contrib slug missing '/': {}",
                c.slug
            );
            assert!(!c.name.is_empty(), "empty oss_contrib name for {}", c.slug);
        }
    }
}
