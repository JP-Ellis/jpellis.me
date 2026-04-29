use serde::Deserialize;

/// A work or education role entry on the CV.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Role {
    /// Start date of the role (e.g. "Sep 2023").
    pub from: String,
    /// End date of the role (e.g. "Present").
    pub to: String,
    /// Organisation or institution name.
    pub org: String,
    /// Location of the role.
    pub loc: String,
    /// Job title or degree name.
    pub title: String,
    /// Short subtitle or technology stack summary.
    pub sub: String,
    /// Main prose body describing the role.
    pub body: String,
    /// Skill/technology tags for this role.
    pub tags: Vec<String>,
    /// Whether this role is highlighted in the featured band.
    #[serde(default)]
    pub featured: bool,
}
