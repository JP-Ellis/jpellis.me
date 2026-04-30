use leptos::prelude::*;
use stylance::import_style;

use crate::components::Band;
use crate::components::Footer;
use crate::components::Masthead;
use crate::integration::WorkStats;
use crate::integration::get_work_stats;
use crate::integration::github::work::model::RepoStats;

import_style!(style, "work.module.scss");

// ─── Types ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub enum Status {
    Active,
    Maintained,
    Archived,
    Shipped,
    Wip,
}

impl Status {
    fn label(self) -> &'static str {
        match self {
            Status::Active => "active",
            Status::Maintained => "maintained",
            Status::Archived => "archived",
            Status::Shipped => "shipped",
            Status::Wip => "wip",
        }
    }
}

#[derive(Clone, Copy)]
pub enum ProjectLink {
    /// GitHub slug, e.g. `"pact-foundation/pact-python"`.
    /// Builds `https://github.com/{slug}` and participates in the star fetch.
    GitHub(&'static str),
    /// Arbitrary external URL — no star fetch.
    External(&'static str),
}

#[derive(Clone, Copy)]
pub struct ProjectEntry {
    pub name: &'static str,
    pub kind: &'static str,
    pub stack: &'static str,
    pub summary: &'static str,
    pub status: Status,
    pub link: Option<ProjectLink>,
}

// ─── Project data ─────────────────────────────────────────────────────────────

pub const PROJECTS: &[ProjectEntry] = &[
    ProjectEntry {
        name: "pact-python",
        kind: "OSS · library",
        stack: "rust · python · ffi",
        summary: "Python bindings for Pact, rebuilt over a Rust FFI core. The version most pact-python users actually reach for.",
        status: Status::Active,
        link: Some(ProjectLink::GitHub("pact-foundation/pact-python")),
    },
    ProjectEntry {
        name: "tikz-feynman",
        kind: "OSS · LaTeX",
        stack: "tex · tikz",
        summary: "A LaTeX package for typesetting Feynman diagrams with TikZ; adopted across particle physics with 400+ academic citations.",
        status: Status::Maintained,
        link: Some(ProjectLink::GitHub("JP-Ellis/tikz-feynman")),
    },
    ProjectEntry {
        name: "rust-skiplist",
        kind: "OSS · library",
        stack: "rust",
        summary: "Skiplist data structure implementation in Rust.",
        status: Status::Maintained,
        link: Some(ProjectLink::GitHub("JP-Ellis/rust-skiplist")),
    },
    ProjectEntry {
        name: "mathematica-notebook-filter",
        kind: "OSS · tool",
        stack: "rust",
        summary: "Filters Mathematica notebooks for clean version-control diffs.",
        status: Status::Maintained,
        link: Some(ProjectLink::GitHub("JP-Ellis/mathematica-notebook-filter")),
    },
    ProjectEntry {
        name: "simpler-wick",
        kind: "OSS · LaTeX",
        stack: "tex · tikz",
        summary: "LaTeX macros for Wick contractions in quantum field theory.",
        status: Status::Maintained,
        link: Some(ProjectLink::GitHub("JP-Ellis/simpler-wick")),
    },
    ProjectEntry {
        name: "boltzmann-solver",
        kind: "research · numerics",
        stack: "rust",
        summary: "Solver for highly-coupled Boltzmann equations with rates spanning many decades; custom quadrature.",
        status: Status::Archived,
        link: Some(ProjectLink::GitHub("hep-rs/boltzmann-solver")),
    },
    ProjectEntry {
        name: "dotfiles",
        kind: "personal · glue",
        stack: "nix · shell",
        summary: "Opinionated dotfiles: nix flake, fish prompt, helix config, backup scripts.",
        status: Status::Maintained,
        link: Some(ProjectLink::GitHub("JP-Ellis/dotfiles")),
    },
    ProjectEntry {
        name: "jpellis.me",
        kind: "personal · site",
        stack: "rust · leptos",
        summary: "This site, rewritten in Rust and Leptos.",
        status: Status::Wip,
        link: Some(ProjectLink::GitHub("JP-Ellis/jpellis.me")),
    },
    ProjectEntry {
        name: "azure data accelerator",
        kind: "consulting · internal",
        stack: "azure · bicep · python",
        summary: "Modular, configuration-driven accelerator for deploying standardised Azure data-platform infrastructure; used as an internal KPMG template.",
        status: Status::Shipped,
        link: None,
    },
    ProjectEntry {
        name: "pactflow-ai",
        kind: "work · internal",
        stack: "rust · python · ai",
        summary: "AI-powered platform that automates the creation and maintenance of contract tests, integrating into existing development tools to save teams up to 60% of manual testing time.",
        status: Status::Active,
        link: Some(ProjectLink::External("https://pactflow.io/ai/")),
    },
];

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Formats a star count as `"1.4k"` for counts ≥ 1000, plain digits otherwise.
///
/// # Arguments
///
/// * `n` - Star count.
///
/// # Returns
///
/// Formatted string.
///
/// # Example
///
/// ```
/// # use jpellis_me::pages::work::format_stars;
/// assert_eq!(format_stars(664), "664");
/// assert_eq!(format_stars(1400), "1.4k");
/// ```
pub fn format_stars(n: u32) -> String {
    if n >= 1000 {
        format!("{:.1}k", n as f64 / 1000.0)
    } else {
        n.to_string()
    }
}

/// Returns the GitHub slug for a project, if it has one.
pub fn github_slug(entry: &ProjectEntry) -> Option<&'static str> {
    match entry.link {
        Some(ProjectLink::GitHub(slug)) => Some(slug),
        _ => None,
    }
}

/// Looks up [`RepoStats`] for a project entry from a fetched [`WorkStats`].
pub fn find_repo_stats<'a>(entry: &ProjectEntry, stats: &'a WorkStats) -> Option<&'a RepoStats> {
    let slug = github_slug(entry)?;
    stats.repos.iter().find(|r| r.slug == slug)
}

// ─── Components ───────────────────────────────────────────────────────────────

#[component]
pub fn WorkPage() -> impl IntoView {
    view! { <div>"Work page coming soon"</div> }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn format_stars_below_1000() {
        assert_eq!(format_stars(0), "0");
        assert_eq!(format_stars(158), "158");
        assert_eq!(format_stars(999), "999");
    }

    #[test]
    fn format_stars_at_and_above_1000() {
        assert_eq!(format_stars(1000), "1.0k");
        assert_eq!(format_stars(1400), "1.4k");
        assert_eq!(format_stars(664), "664");
    }

    #[test]
    fn github_slug_returns_slug_for_github_link() {
        let entry = ProjectEntry {
            name: "test",
            kind: "",
            stack: "",
            summary: "",
            status: Status::Active,
            link: Some(ProjectLink::GitHub("owner/repo")),
        };
        assert_eq!(github_slug(&entry), Some("owner/repo"));
    }

    #[test]
    fn github_slug_returns_none_for_external_link() {
        let entry = ProjectEntry {
            name: "test",
            kind: "",
            stack: "",
            summary: "",
            status: Status::Active,
            link: Some(ProjectLink::External("https://example.com")),
        };
        assert_eq!(github_slug(&entry), None);
    }

    #[test]
    fn github_slug_returns_none_for_no_link() {
        let entry = ProjectEntry {
            name: "test",
            kind: "",
            stack: "",
            summary: "",
            status: Status::Active,
            link: None,
        };
        assert_eq!(github_slug(&entry), None);
    }

    #[test]
    fn projects_list_has_expected_length() {
        assert_eq!(PROJECTS.len(), 10);
    }

    #[test]
    fn all_github_projects_have_slash_in_slug() {
        for p in PROJECTS {
            if let Some(ProjectLink::GitHub(slug)) = p.link {
                assert!(
                    slug.contains('/'),
                    "project '{}' slug missing slash: {slug}",
                    p.name
                );
            }
        }
    }
}
