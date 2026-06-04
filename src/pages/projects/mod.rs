#![expect(
    clippy::shadow_reuse,
    reason = "Leptos #[component] macro internally re-binds function parameters"
)]

use leptos::prelude::*;
use stylance::import_style;

use crate::components::Band;
use crate::components::Footer;
use crate::components::Masthead;
use crate::config::projects::projects_config;
use crate::integration::ProjectsStats;
use crate::integration::get_projects_stats;
use crate::integration::github::projects::model::RepoStats;

import_style!(style, "projects.module.scss");

// ─── Project detail pages ─────────────────────────────────────────────────────

/// Runtime configuration controlling which activity panels render on a detail page.
#[derive(Clone, Copy)]
pub struct ActivityConfig {
    /// Show the latest release chip.
    pub release: bool,
    /// Show the recent commits panel.
    pub recent_commits: bool,
    /// Show the open pull requests chip.
    pub open_prs: bool,
}

/// A project detail page loaded from `content/projects/*.md` at compile time.
#[derive(Clone, Copy)]
pub struct ProjectPage {
    /// URL slug matching the content filename and `ProjectEntry::name`.
    pub slug: &'static str,
    /// Display title for the page heading and `<title>` tag.
    pub title: &'static str,
    /// GitHub slug, e.g. `"JP-Ellis/tikz-feynman"`.
    pub github: &'static str,
    /// Short tagline used in the hero and meta description.
    pub tagline: &'static str,
    /// Which activity panels are enabled for this project.
    pub activity: ActivityConfig,
    /// Pre-rendered HTML body from the markdown content file.
    pub body_html: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/project_pages.rs"));

/// Returns the project detail page for the given slug, if one exists.
///
/// The slug must match the `slug` field in the markdown frontmatter exactly.
/// Called by `ProjectsRow` (to determine link target) and `ProjectDetailPage`
/// (to look up page content from the route parameter).
///
/// # Arguments
///
/// * `slug` - The URL slug to look up, e.g. `"tikz-feynman"`.
///
/// # Returns
///
/// `Some(&'static ProjectPage)` if a matching page exists, `None` otherwise.
pub fn find_project_page(slug: &str) -> Option<&'static ProjectPage> {
    PROJECT_PAGES.iter().find(|p| p.slug == slug)
}

/// Individual project detail page, loaded from `content/projects/*.md`.
pub mod detail;

// ─── Types ───────────────────────────────────────────────────────────────────

/// Development status of a project entry.
#[derive(Clone, Copy)]
pub enum Status {
    /// Actively developed; new features and fixes land regularly.
    Active,
    /// Feature-complete and stable; bug fixes accepted.
    Maintained,
    /// Read-only; no further development planned.
    Archived,
    /// Delivered as a discrete engagement; not actively maintained.
    Shipped,
    /// Work-in-progress; not yet publicly released.
    Wip,
}

impl Status {
    /// Returns the human-readable label for this status.
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

/// Link target for a project entry row.
#[derive(Clone, Copy)]
pub enum ProjectLink {
    /// GitHub slug, e.g. `"pact-foundation/pact-python"`.
    /// Builds `https://github.com/{slug}` and participates in the star fetch.
    GitHub(&'static str),
    /// Arbitrary external URL — no star fetch.
    External(&'static str),
}

/// A single row in the projects index table.
#[derive(Clone, Copy)]
pub struct ProjectEntry {
    /// Short project name, also used as the URL path segment.
    pub name: &'static str,
    /// Category label displayed in the "kind" column (e.g. `"OSS · library"`).
    pub kind: &'static str,
    /// Technology stack summary (e.g. `"rust · python · ffi"`).
    pub stack: &'static str,
    /// One-sentence description shown in the summary column.
    pub summary: &'static str,
    /// Development status of this project.
    pub status: Status,
    /// Optional link target for the project row.
    pub link: Option<ProjectLink>,
}

// ─── Project data ─────────────────────────────────────────────────────────────

/// All projects displayed on the projects index page.
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
        name: "borrow-checker",
        kind: "personal · app",
        stack: "rust",
        summary: "Personal finance app for tracking accounts, spending, and cashflow — named for the compiler feature that stops you spending twice.",
        status: Status::Wip,
        link: Some(ProjectLink::GitHub("JP-Ellis/borrow-checker")),
    },
    ProjectEntry {
        name: "amber-api",
        kind: "personal · library",
        stack: "rust",
        summary: "Rust client for Amber Electric's real-time energy pricing API.",
        status: Status::Maintained,
        link: Some(ProjectLink::GitHub("JP-Ellis/amber-api")),
    },
    ProjectEntry {
        name: "enphase-api",
        kind: "personal · library",
        stack: "rust",
        summary: "Rust client for the Enphase solar monitoring API.",
        status: Status::Maintained,
        link: Some(ProjectLink::GitHub("JP-Ellis/enphase-api")),
    },
    ProjectEntry {
        name: "repo-manage",
        kind: "personal · tool",
        stack: "python",
        summary: "Convenience CLI for applying consistent settings and CI workflows across a fleet of personal repositories.",
        status: Status::Maintained,
        link: Some(ProjectLink::GitHub("JP-Ellis/repo-manage")),
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
/// # use jpellis_me::pages::projects::format_stars;
/// assert_eq!(format_stars(664), "664");
/// assert_eq!(format_stars(1400), "1.4k");
/// ```
#[expect(
    clippy::float_arithmetic,
    reason = "float division for human-readable star count formatting is intentional"
)]
pub fn format_stars(n: u32) -> String {
    if n >= 1000 {
        format!("{:.1}k", f64::from(n) / 1000.0)
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

/// Looks up [`RepoStats`] for a project entry from a fetched [`ProjectsStats`].
pub fn find_repo_stats<'a>(
    entry: &ProjectEntry,
    stats: &'a ProjectsStats,
) -> Option<&'a RepoStats> {
    let slug = github_slug(entry)?;
    stats.repos.iter().find(|r| r.slug == slug)
}

// ─── ProjectsBand ────────────────────────────────────────────────────────────

/// Returns the total star count across all tracked repositories.
fn total_stars(stats: &ProjectsStats) -> u32 {
    stats.repos.iter().map(|r| r.stars).sum()
}

/// Returns the total fork count across all tracked repositories.
fn total_forks(stats: &ProjectsStats) -> u32 {
    stats.repos.iter().map(|r| r.forks).sum()
}

#[component]
fn ProjectsBand(
    /// Fetched project stats used to populate star and fork totals; `None` renders skeletons.
    stats: Option<ProjectsStats>,
) -> impl IntoView {
    let stars: AnyView = match &stats {
        Some(s) => format_stars(total_stars(s)).into_any(),
        None => view! { <span class=style::row_stars_skeleton aria-hidden="true" /> }.into_any(),
    };
    let forks: AnyView = match &stats {
        Some(s) => total_forks(s).to_string().into_any(),
        None => view! { <span class=style::row_stars_skeleton aria-hidden="true" /> }.into_any(),
    };

    view! {
        <Band test_id="projects-band">
            <div class=format!("container {}", style::band_inner)>

                <div class=style::stats_row>
                    <dl class=style::stat_item>
                        <dt class=style::stat_value>{stars}</dt>
                        <dd class=style::stat_label>
                            <span class="sr-only">"Total "</span>
                            "stars"
                        </dd>
                    </dl>
                    <dl class=style::stat_item>
                        <dt class=style::stat_value>{forks}</dt>
                        <dd class=style::stat_label>"forks"</dd>
                    </dl>
                    <dl class=style::stat_item>
                        <dt class=style::stat_value>"400+"</dt>
                        <dd class=style::stat_label>"citations"</dd>
                    </dl>
                    <dl class=style::stat_item>
                        <dt class=style::stat_value>"2015"</dt>
                        <dd class=style::stat_label>"open source since"</dd>
                    </dl>
                </div>

                <div class=format!("eyebrow-grid {}", style::fingerprint_row)>
                    <span class="eyebrow eyebrow--muted">"The stack"</span>
                    <div class=style::fingerprint_tags>
                        <span class="tag tag--pill">"rust"</span>
                        <span class="tag tag--pill">"python"</span>
                        <span class="tag tag--pill">"latex"</span>
                        <span class="tag tag--pill">"tikz"</span>
                        <span class="tag tag--pill">"nix"</span>
                        <span class="tag tag--pill">"shell"</span>
                    </div>
                </div>

            </div>
        </Band>
    }
}

// ─── ProjectsRow ─────────────────────────────────────────────────────────────

#[component]
fn ProjectsRow(
    /// The project entry to render.
    entry: &'static ProjectEntry,
    /// Fetched GitHub stats for this project; `None` renders a star skeleton.
    repo: Option<RepoStats>,
) -> impl IntoView {
    let status_class = match entry.status {
        Status::Active => style::row_status_active,
        Status::Maintained => style::row_status_maintained,
        Status::Archived => style::row_status_archived,
        Status::Shipped => style::row_status_shipped,
        Status::Wip => style::row_status_wip,
    };

    let stars_cell = match &repo {
        Some(r) => format_stars(r.stars).into_any(),
        None => match entry.link {
            Some(ProjectLink::GitHub(_)) => {
                view! { <span class=style::row_stars_skeleton aria-hidden="true" /> }.into_any()
            }
            _ => "—".into_any(),
        },
    };

    // Detail page takes priority over GitHub / external links
    let name_cell = if find_project_page(entry.name).is_some() {
        view! {
            <a
                href=format!("/projects/{}", entry.name)
                class=format!("{} {}", style::row_name, style::row_name_link)
            >
                {entry.name}
            </a>
        }
        .into_any()
    } else {
        match entry.link {
            Some(ProjectLink::GitHub(slug)) => view! {
                <a
                    href=format!("https://github.com/{slug}")
                    class=format!("{} {}", style::row_name, style::row_name_link)
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    {entry.name}
                </a>
            }
            .into_any(),
            Some(ProjectLink::External(url)) => view! {
                <a
                    href=url
                    class=format!("{} {}", style::row_name, style::row_name_link)
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    {entry.name}
                </a>
            }
            .into_any(),
            None => view! { <span class=style::row_name>{entry.name}</span> }.into_any(),
        }
    };

    view! {
        <div class=format!("{} rule-list", style::index_row) data-testid="projects-row">
            {name_cell}
            <span class=style::row_kind>{entry.kind}</span>
            <div>
                <span class=style::row_summary>{entry.summary}</span>
                <span class=style::row_stack>{entry.stack}</span>
            </div>
            <span class=style::row_stars>{stars_cell}</span>
            <span class=format!(
                "{} {}",
                style::row_status,
                status_class,
            )>"● " {entry.status.label()}</span>
        </div>
    }
}

// ─── ProjectsIndex ───────────────────────────────────────────────────────────

#[component]
fn ProjectsIndex(
    /// Fetched project stats passed down to each row; `None` renders star skeletons.
    stats: Option<ProjectsStats>,
) -> impl IntoView {
    let rows = PROJECTS
        .iter()
        .map(|entry| {
            let repo = stats
                .as_ref()
                .and_then(|s| find_repo_stats(entry, s))
                .cloned();
            view! { <ProjectsRow entry=entry repo=repo /> }
        })
        .collect_view();

    view! {
        <section class=style::index_section>
            <div class="container">
                <div class="eyebrow-grid">
                    <span class="eyebrow">"Index"</span>
                    <div>
                        <div class=format!("{} rule-section", style::index_header)>
                            <span>"repo"</span>
                            <span>"kind"</span>
                            <span>"summary"</span>
                            <span class=style::row_stars>"★"</span>
                            <span>"status"</span>
                        </div>
                        {rows}
                    </div>
                </div>
            </div>
        </section>
    }
}

// ─── OpenContributions ───────────────────────────────────────────────────────

#[component]
fn OpenContributions() -> impl IntoView {
    let links = projects_config()
        .oss_contribs
        .iter()
        .take(20)
        .enumerate()
        .map(|(i, c)| {
            let href = format!("https://github.com/{}", c.slug);
            let name = c.name.clone();
            let stars_label = format!(
                "{}★",
                format_stars(u32::try_from(c.stars).unwrap_or(u32::MAX))
            );
            let sep: Option<&'static str> = (i > 0).then_some(" · ");
            view! {
                {sep}
                <a href=href target="_blank" rel="noopener noreferrer">
                    {name}
                </a>
                " "
                <span class=style::contrib_stars aria-label=format!("{} stars", c.stars)>
                    {stars_label}
                </span>
            }
        })
        .collect_view();

    view! {
        <section class=style::contrib_section>
            <div class="container">
                <div class="eyebrow-grid">
                    <span class="eyebrow">"Open source"</span>
                    <div class=style::contrib_body>
                        <p class=style::contrib_prose>
                            "Beyond named projects, I make regular contributions across
                            the open-source ecosystem — bug fixes, small features, and
                            documentation wherever I use the tools."
                        </p>
                        <p class=style::contrib_links>{links} " · and others"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

// ─── ProjectsPage ────────────────────────────────────────────────────────────

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let projects_res = LocalResource::new(get_projects_stats);

    view! {
        <Masthead />
        <main>
            <section class=style::hero>
                <div class="container">
                    <p class="eyebrow">"Projects"</p>
                    <h1>"Things I've " <em>"built"</em> ", still standing."</h1>
                    <p class=style::lead>
                        "A small index. Each item links to a write-up, the README, "
                        "or — for the academic ones — the paper."
                    </p>
                </div>
            </section>

            <Suspense fallback=move || {
                view! {
                    <ProjectsBand stats=None />
                    <ProjectsIndex stats=None />
                }
            }>
                {move || {
                    let stats: Option<ProjectsStats> = projects_res
                        .get()
                        .and_then(std::result::Result::ok);
                    view! {
                        <ProjectsBand stats=stats.clone() />
                        <ProjectsIndex stats=stats />
                    }
                        .into_any()
                }}
            </Suspense>
            <OpenContributions />
        </main>
        <Footer />
    }
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
        assert_eq!(PROJECTS.len(), 14);
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn all_github_project_slugs_are_tracked_in_config() {
        let tracked: std::collections::HashSet<&str> = crate::config::projects::projects_config()
            .tracked_slugs
            .iter()
            .map(std::string::String::as_str)
            .collect();
        for p in PROJECTS {
            if let Some(ProjectLink::GitHub(slug)) = p.link {
                assert!(
                    tracked.contains(slug),
                    "project '{}' slug '{}' is missing from tracked_slugs in src/config/projects.toml",
                    p.name,
                    slug,
                );
            }
        }
    }

    #[test]
    fn find_project_page_returns_none_for_unknown_slug() {
        assert!(find_project_page("no-such-project-xyz").is_none());
    }

    #[test]
    fn project_pages_count_matches_content_files() {
        // If this fails, a content/projects/*.md file has malformed frontmatter
        // and build.rs silently skipped it.
        assert_eq!(PROJECT_PAGES.len(), 4);
    }

    #[test]
    fn projects_row_slug_matches_entry_name_for_all_detail_pages() {
        // For every detail page, entry.name must match page.slug exactly.
        // ProjectsRow uses entry.name for the URL; find_project_page matches slug.
        // If these diverge, the link silently 404s.
        for page in PROJECT_PAGES {
            let found = PROJECTS.iter().any(|entry| entry.name == page.slug);
            assert!(
                found,
                "ProjectPage slug {:?} has no matching ProjectEntry with the same name. \
                 Either add an entry to PROJECTS or fix the slug in the content file.",
                page.slug
            );
        }
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
