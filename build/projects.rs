//! Project page collection and Rust code generation.
//!
//! Walks `content/projects/`, parses TOML frontmatter and Markdown bodies,
//! and emits a `static PROJECT_PAGES: &[ProjectPage]` array for inclusion in
//! the main crate.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use super::markdown::render;

/// TOML frontmatter fields for a project detail page.
#[derive(Deserialize)]
pub struct ProjectFrontmatter {
    /// Display title of the project.
    pub title: String,
    /// URL-safe slug used to construct the project's route.
    pub slug: String,
    /// GitHub repository in `owner/repo` form.
    pub github: String,
    /// Short one-line description shown in project listings.
    pub tagline: String,
    /// Controls which live-activity widgets are shown on the project page.
    #[serde(default)]
    pub activity: ProjectActivityFrontmatter,
}

/// Activity display flags for a project page.
///
/// All fields default to `true`; only set a field to `false` to disable it.
#[derive(Deserialize)]
pub struct ProjectActivityFrontmatter {
    /// Whether to show the latest release widget.
    #[serde(default = "bool_true")]
    pub release: bool,
    /// Whether to show the recent commits widget.
    #[serde(default = "bool_true")]
    pub recent_commits: bool,
    /// Whether to show the open pull requests widget.
    #[serde(default = "bool_true")]
    pub open_prs: bool,
}

impl Default for ProjectActivityFrontmatter {
    fn default() -> Self {
        Self {
            release: true,
            recent_commits: true,
            open_prs: true,
        }
    }
}

/// Default value provider returning `true`, used by serde field defaults.
fn bool_true() -> bool {
    true
}

/// A fully processed project detail page ready for code generation.
pub struct ProjectPageData {
    /// URL-safe slug for routing.
    pub slug: String,
    /// Display title of the project.
    pub title: String,
    /// GitHub repository in `owner/repo` form.
    pub github: String,
    /// Short one-line tagline for listings.
    pub tagline: String,
    /// Whether to display the latest release widget.
    pub activity_release: bool,
    /// Whether to display the recent commits widget.
    pub activity_recent_commits: bool,
    /// Whether to display the open pull requests widget.
    pub activity_open_prs: bool,
    /// Full HTML body of the project page.
    pub body_html: String,
}

/// Parse TOML frontmatter and the Markdown body from raw project file content.
///
/// Frontmatter must be delimited by `+++` lines at the start of the file.
///
/// # Arguments
///
/// * `raw_content` - Raw file content including the frontmatter delimiters.
///
/// # Returns
///
/// `Some((frontmatter, body_markdown))` on success, or `None` if the
/// frontmatter is missing or cannot be deserialized.
#[expect(
    clippy::string_slice,
    clippy::arithmetic_side_effects,
    reason = "Indexing based on found byte offsets, or known delimiter lengths"
)]
pub fn parse_project_frontmatter(raw_content: &str) -> Option<(ProjectFrontmatter, &str)> {
    let content = raw_content.trim_start();
    let after_open = content
        .strip_prefix("+++\n")
        .or_else(|| content.strip_prefix("+++\r\n"))?;
    let close_pos = after_open.find("\n+++")?;
    let fm_str = &after_open[..close_pos];
    let rest = &after_open[close_pos + 4..];
    let body = rest
        .strip_prefix('\n')
        .or_else(|| rest.strip_prefix("\r\n"))
        .unwrap_or(rest);
    let fm: ProjectFrontmatter = toml::from_str(fm_str).ok()?;
    Some((fm, body))
}

/// Collect all project detail pages under `dir`.
///
/// Non-Markdown files and files with invalid frontmatter are silently skipped.
/// Results are sorted by slug for deterministic output.
///
/// # Arguments
///
/// * `dir` - Directory containing project `.md` files.
///
/// # Returns
///
/// A `Vec<ProjectPageData>` sorted by slug.
pub fn collect_project_pages(dir: &Path) -> Vec<ProjectPageData> {
    let Ok(entries) = fs::read_dir(dir) else {
        return vec![];
    };
    let mut pages: Vec<ProjectPageData> = entries
        .flatten()
        .filter(|e| e.path().extension().and_then(|ext| ext.to_str()) == Some("md"))
        .filter_map(|e| process_project_file(&e.path()))
        .collect();
    pages.sort_by(|a, b| a.slug.cmp(&b.slug));
    pages
}

/// Parse a single Markdown file into a [`ProjectPageData`].
///
/// Returns `None` if the file cannot be read or its frontmatter is invalid.
///
/// # Arguments
///
/// * `path` - Path to the `.md` file.
///
/// # Returns
///
/// `Some(ProjectPageData)` on success, `None` on parse failure.
fn process_project_file(path: &Path) -> Option<ProjectPageData> {
    println!("cargo:rerun-if-changed={}", path.display());
    let content = fs::read_to_string(path).ok()?;
    let (fm, body_md) = parse_project_frontmatter(&content)?;
    let body_html = render(body_md);
    Some(ProjectPageData {
        slug: fm.slug,
        title: fm.title,
        github: fm.github,
        tagline: fm.tagline,
        activity_release: fm.activity.release,
        activity_recent_commits: fm.activity.recent_commits,
        activity_open_prs: fm.activity.open_prs,
        body_html,
    })
}

/// Generate a Rust source file containing a `static PROJECT_PAGES: &[ProjectPage]` array.
///
/// # Arguments
///
/// * `pages` - Slice of processed project pages to emit.
///
/// # Returns
///
/// A `String` containing valid Rust source code for inclusion via `include!`.
pub fn generate_project_pages_code(pages: &[ProjectPageData]) -> String {
    let mut code = String::new();
    writeln!(
        code,
        "/// Compiled project detail pages (generated by the build script from Markdown)."
    )
    .expect("writing to String is infallible");
    writeln!(code, "pub static PROJECT_PAGES: &[ProjectPage] = &[")
        .expect("writing to String is infallible");
    for page in pages {
        writeln!(code, "    ProjectPage {{").expect("writing to String is infallible");
        writeln!(code, "        slug: {:?},", page.slug).expect("writing to String is infallible");
        writeln!(code, "        title: {:?},", page.title)
            .expect("writing to String is infallible");
        writeln!(code, "        github: {:?},", page.github)
            .expect("writing to String is infallible");
        writeln!(code, "        tagline: {:?},", page.tagline)
            .expect("writing to String is infallible");
        writeln!(code, "        activity: ActivityConfig {{")
            .expect("writing to String is infallible");
        writeln!(code, "            release: {:?},", page.activity_release)
            .expect("writing to String is infallible");
        writeln!(
            code,
            "            recent_commits: {:?},",
            page.activity_recent_commits
        )
        .expect("writing to String is infallible");
        writeln!(code, "            open_prs: {:?},", page.activity_open_prs)
            .expect("writing to String is infallible");
        writeln!(code, "        }},").expect("writing to String is infallible");
        writeln!(code, "        body_html: {:?},", page.body_html)
            .expect("writing to String is infallible");
        writeln!(code, "    }},").expect("writing to String is infallible");
    }
    writeln!(code, "];").expect("writing to String is infallible");
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_project_frontmatter_minimal() {
        let input = "+++\ntitle = \"TikZ-Feynman\"\nslug = \"tikz-feynman\"\ngithub = \"JP-Ellis/tikz-feynman\"\ntagline = \"Feynman diagrams in LaTeX\"\n+++\n\nBody here.";
        let (fm, body) = parse_project_frontmatter(input).expect("should parse");
        assert_eq!(fm.slug, "tikz-feynman");
        assert_eq!(fm.title, "TikZ-Feynman");
        assert_eq!(fm.github, "JP-Ellis/tikz-feynman");
        assert_eq!(fm.tagline, "Feynman diagrams in LaTeX");
        // All activity fields default to true when [activity] section is omitted
        assert!(fm.activity.release);
        assert!(fm.activity.recent_commits);
        assert!(fm.activity.open_prs);
        assert_eq!(body.trim(), "Body here.");
    }

    #[test]
    fn parse_project_frontmatter_with_activity_override() {
        let input = "+++\ntitle = \"rust-skiplist\"\nslug = \"rust-skiplist\"\ngithub = \"JP-Ellis/rust-skiplist\"\ntagline = \"Skip list in Rust\"\n\n[activity]\nrecent_commits = false\n+++\n\nContent.";
        let (fm, _) = parse_project_frontmatter(input).expect("should parse");
        assert!(fm.activity.release); // defaulted to true
        assert!(!fm.activity.recent_commits); // explicitly false
        assert!(fm.activity.open_prs); // defaulted to true
    }

    #[test]
    fn parse_project_frontmatter_returns_none_without_delimiters() {
        let input = "title = \"No delimiters\"\n";
        assert!(parse_project_frontmatter(input).is_none());
    }
}
