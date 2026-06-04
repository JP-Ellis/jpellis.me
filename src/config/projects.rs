use std::sync::OnceLock;

use serde::Deserialize;

/// Lazily initialised cache for the parsed projects configuration.
static CACHE: OnceLock<ProjectsConfig> = OnceLock::new();

/// Configuration for the projects page, loaded from `projects.toml`.
#[derive(Debug, Deserialize)]
pub struct ProjectsConfig {
    /// GitHub slugs to track for star/fork counts.
    /// Must cover every `ProjectLink::GitHub` entry in `pages::projects::PROJECTS`.
    #[cfg(feature = "ssr")]
    pub tracked_slugs: Vec<String>,
    /// Misc OSS contributions shown at the bottom of the projects page.
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

/// Raw TOML content of the embedded projects configuration file.
const PROJECTS_CONFIG_TOML: &str = include_str!("projects.toml");

/// Returns the parsed projects configuration, initialised once.
#[expect(
    clippy::expect_used,
    reason = "static initializer: corrupt embedded config should panic at startup"
)]
pub fn projects_config() -> &'static ProjectsConfig {
    CACHE.get_or_init(|| {
        toml::from_str(PROJECTS_CONFIG_TOML).expect("src/config/projects.toml is not valid TOML")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "ssr")]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn tracked_slugs_are_valid_format() {
        for slug in &projects_config().tracked_slugs {
            assert!(!slug.is_empty(), "empty tracked slug");
            assert!(slug.contains('/'), "tracked slug missing '/': {slug}");
        }
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn oss_contribs_slugs_are_valid_format() {
        for c in &projects_config().oss_contribs {
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
