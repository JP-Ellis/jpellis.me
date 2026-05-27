use std::env;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use toml::value::Datetime;

fn main() {
    println!("cargo:rerun-if-changed=content/blog");
    println!("cargo:rerun-if-changed=content/projects");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    // MARK: Blog posts
    let blog_out_path = PathBuf::from(&out_dir).join("blog_posts.rs");
    let mut tab_counter = 0usize;
    let mut posts: Vec<Post> = collect_posts(Path::new("content/blog"), &mut tab_counter);
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    let code = generate_code(&posts);
    fs::write(&blog_out_path, code).expect("failed to write blog_posts.rs");

    // MARK: Project pages
    let pages_out_path = PathBuf::from(&out_dir).join("project_pages.rs");
    let project_pages = collect_project_pages(Path::new("content/projects"));
    let pages_code = generate_project_pages_code(&project_pages);
    fs::write(&pages_out_path, pages_code).expect("failed to write project_pages.rs");
}

// ── Frontmatter ───────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct Frontmatter {
    title: String,
    date: Datetime,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    source: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    slug: Option<String>,
}

fn parse_frontmatter(content: &str) -> Option<(Frontmatter, &str)> {
    let content = content.trim_start();
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

    let fm: Frontmatter = toml::from_str(fm_str).ok()?;
    Some((fm, body))
}

// ── Slug derivation ───────────────────────────────────────────────────────

fn derive_slug(path: &Path, fm_slug: Option<&str>) -> String {
    if let Some(slug) = fm_slug {
        return slug.to_string();
    }
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let stem = if stem.len() > 6
        && stem[..2].chars().all(|c| c.is_ascii_digit())
        && stem.chars().nth(2) == Some('-')
        && stem[3..5].chars().all(|c| c.is_ascii_digit())
        && stem.chars().nth(5) == Some(' ')
    {
        &stem[6..]
    } else {
        stem
    };
    let slug: String = stem
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();
    slug.split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

// ── Markdown rendering ────────────────────────────────────────────────────

fn render_markdown(content: &str) -> String {
    use pulldown_cmark::Options;
    use pulldown_cmark::Parser;
    use pulldown_cmark::html;
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);
    let parser = Parser::new_ext(content, opts);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}

// ── PyMdown post-processing ──────────────────────────────────────────────

/// Parse "tab | Title" or "details | Example" into (block_type, title).
///
/// # Arguments
///
/// * `s` - The string after `/// ` (e.g., `"tab | Before"` or `"details | Example"`)
///
/// # Returns
///
/// A tuple `(block_type, title)` where both are `&str` slices of the input.
fn parse_pymdownx_type_title(s: &str) -> (&str, &str) {
    match s.split_once(" | ") {
        Some((block_type, title)) => (block_type.trim(), title.trim()),
        None => (s.trim(), ""),
    }
}

/// Extract inner HTML up to the next `<p>///</p>` closer.
///
/// Returns `Some((inner_html, html_after_closer))`, or `None` if no closer exists.
/// The single `\n` that pulldown-cmark emits after `</p>` is stripped from
/// `html_after_closer` automatically.
///
/// # Arguments
///
/// * `html` - The HTML string to search for a closer
///
/// # Returns
///
/// `Some((inner_html, rest))` if a closer is found; `None` otherwise.
fn find_block_content(html: &str) -> Option<(&str, &str)> {
    const CLOSER: &str = "<p>///</p>";
    let pos = html.find(CLOSER)?;
    let after = &html[pos + CLOSER.len()..];
    let after = after.strip_prefix('\n').unwrap_or(after);
    Some((&html[..pos], after))
}

/// Render consecutive `/// tab` blocks as a CSS-only interactive tab group.
///
/// `tabs` is `(label, inner_html)` pairs. `counter` is incremented once per
/// call and provides unique `name`/`id` attributes across all blog posts.
///
/// # Arguments
///
/// * `tabs` - Slice of `(label, inner_html)` pairs, one per tab.
/// * `counter` - Mutable counter for generating unique IDs; incremented once per call.
///
/// # Returns
///
/// A `String` containing the full tab-group HTML.
fn render_tab_group(tabs: &[(String, String)], counter: &mut usize) -> String {
    let g = *counter;
    *counter += 1;
    let mut out = String::new();
    out.push_str("<div class=\"tabs\">\n");
    for (i, _) in tabs.iter().enumerate() {
        let checked = if i == 0 { " checked" } else { "" };
        writeln!(
            out,
            "<input class=\"tab-radio\" type=\"radio\" \
             name=\"tabs-{g}\" id=\"tab-{g}-{i}\"{checked}>"
        )
        .expect("writing to String is infallible");
    }
    out.push_str("<div class=\"tab-bar\">\n");
    for (i, (title, _)) in tabs.iter().enumerate() {
        writeln!(out, "<label for=\"tab-{g}-{i}\">{title}</label>")
            .expect("writing to String is infallible");
    }
    out.push_str("</div>\n");
    for (_, content) in tabs {
        out.push_str("<section class=\"tab-panel\">\n");
        let inner = content.trim();
        if !inner.is_empty() {
            out.push_str(inner);
            out.push('\n');
        }
        out.push_str("</section>\n");
    }
    out.push_str("</div>\n");
    out
}

/// Render a `/// details | Title` block as a native `<details>/<summary>` element.
///
/// # Arguments
///
/// * `title` - The summary text; if empty, defaults to `"Details"`.
/// * `content` - The inner HTML for the details body.
///
/// # Returns
///
/// A `String` containing the `<details>` HTML.
fn render_details_html(title: &str, content: &str) -> String {
    let summary = if title.is_empty() { "Details" } else { title };
    let inner = content.trim();
    format!("<details>\n<summary>{summary}</summary>\n{inner}\n</details>\n")
}

/// Scan `pulldown-cmark`-rendered HTML for PyMdown extension markers and
/// replace them with proper tab-group or details HTML.
///
/// pulldown-cmark renders `/// tab | Before` as `<p>/// tab | Before</p>` and
/// `///` (closer) as `<p>///</p>`. This function finds those marker paragraphs,
/// extracts the inner HTML between them, and emits the replacement HTML.
///
/// Consecutive `/// tab` blocks are collapsed into one group.
///
/// # Arguments
///
/// * `html` - The rendered HTML string from pulldown-cmark.
/// * `counter` - Mutable counter for generating unique tab group IDs.
///
/// # Returns
///
/// A new `String` with all PyMdown markers replaced by proper HTML.
fn postprocess_pymdownx(html: &str, counter: &mut usize) -> String {
    let mut result = String::with_capacity(html.len());
    let mut remaining = html;

    loop {
        let Some(offset) = remaining.find("<p>/// ") else {
            result.push_str(remaining);
            break;
        };

        result.push_str(&remaining[..offset]);
        remaining = &remaining[offset..];

        // Parse `<p>/// type | title</p>`
        let after_open = &remaining[3..]; // skip "<p>"
        let Some(close_pos) = after_open.find("</p>") else {
            result.push_str("<p>");
            remaining = after_open;
            continue;
        };
        let opener_text = &after_open[..close_pos]; // "/// tab | Before"
        let after_close = &after_open[close_pos + 4..]; // after "</p>"
        let after_close = after_close.strip_prefix('\n').unwrap_or(after_close);

        let Some(type_and_title) = opener_text.strip_prefix("/// ") else {
            result.push_str(&remaining[..close_pos + 7]);
            remaining = after_close;
            continue;
        };

        let (block_type, title) = parse_pymdownx_type_title(type_and_title);

        let Some((inner_html, after_closer)) = find_block_content(after_close) else {
            // No matching closer — emit as literal text and continue scanning
            result.push_str("<p>");
            result.push_str(opener_text);
            result.push_str("</p>\n");
            remaining = after_close;
            continue;
        };

        match block_type {
            "tab" => {
                let mut tabs = vec![(title.to_string(), inner_html.to_string())];
                let mut current = after_closer;

                // Collect adjacent tab blocks into the same group
                loop {
                    let peek = current.trim_start_matches(['\n', '\r']);
                    if !peek.starts_with("<p>/// tab") {
                        break;
                    }
                    let after_p = &peek[3..]; // skip "<p>"
                    let Some(ep) = after_p.find("</p>") else {
                        break;
                    };
                    let opener2 = &after_p[..ep];
                    let after_close2 = &after_p[ep + 4..];
                    let after_close2 = after_close2.strip_prefix('\n').unwrap_or(after_close2);
                    let ta = opener2.strip_prefix("/// tab").unwrap_or("");
                    let title2 = ta.strip_prefix(" | ").unwrap_or("").trim();
                    let Some((inner2, after_closer2)) = find_block_content(after_close2) else {
                        break;
                    };
                    tabs.push((title2.to_string(), inner2.to_string()));
                    current = after_closer2;
                }

                result.push_str(&render_tab_group(&tabs, counter));
                remaining = current;
            }
            "details" => {
                result.push_str(&render_details_html(title, inner_html));
                remaining = after_closer;
            }
            _ => {
                // Unknown block type — pass through as literal text
                result.push_str("<p>");
                result.push_str(opener_text);
                result.push_str("</p>\n");
                remaining = after_close;
            }
        }
    }

    result
}

// ── Excerpt splitting ─────────────────────────────────────────────────────

fn strip_headings(html: &str) -> String {
    let mut result = html.to_string();
    for n in 1u8..=6 {
        let open = format!("<h{n}");
        let close = format!("</h{n}>");
        while let Some(start) = result.find(&open) {
            let after = &result[start + open.len()..];
            if !after.starts_with(['>', ' ']) {
                break;
            }
            match result[start..].find(&close) {
                Some(rel_end) => {
                    let end = start + rel_end + close.len();
                    let prefix = result[..start].trim_end().to_string();
                    let suffix = result[end..].trim_start().to_string();
                    result = match (prefix.is_empty(), suffix.is_empty()) {
                        (true, _) => suffix,
                        (_, true) => prefix,
                        _ => format!("{prefix}\n{suffix}"),
                    };
                }
                None => break,
            }
        }
    }
    result.trim().to_string()
}

fn split_excerpt(body_html: &str, description: Option<&str>) -> String {
    if let Some(pos) = body_html.find("<!-- more -->") {
        return strip_headings(body_html[..pos].trim());
    }
    if let Some(desc) = description {
        return format!("<p>{desc}</p>");
    }
    // Fall back to first <p>...</p>
    if let Some(start) = body_html.find("<p>")
        && let Some(rel_end) = body_html[start..].find("</p>")
    {
        return body_html[start..start + rel_end + 4].to_string();
    }
    body_html.to_string()
}

// ── Post collection ───────────────────────────────────────────────────────

struct Post {
    slug: String,
    title: String,
    date: String,
    description: Option<String>,
    source: Option<String>,
    tags: Vec<String>,
    excerpt_html: String,
    body_html: String,
}

fn collect_posts(dir: &Path, tab_counter: &mut usize) -> Vec<Post> {
    let mut posts = Vec::new();
    visit_dir(dir, &mut posts, tab_counter);
    posts
}

fn visit_dir(dir: &Path, posts: &mut Vec<Post>, tab_counter: &mut usize) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        println!("cargo:rerun-if-changed={}", path.display());
        if path.is_dir() {
            visit_dir(&path, posts, tab_counter);
        } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
            if let Some(post) = process_file(&path, tab_counter) {
                posts.push(post);
            } else {
                println!("cargo:warning=Skipped (parse error): {}", path.display());
            }
        }
    }
}

fn process_file(path: &Path, tab_counter: &mut usize) -> Option<Post> {
    let content = fs::read_to_string(path).ok()?;
    let (fm, body_md) = parse_frontmatter(&content)?;
    let slug = derive_slug(path, fm.slug.as_deref());
    let date = fm.date.to_string();
    let body_html = render_markdown(body_md);
    let body_html = postprocess_pymdownx(&body_html, tab_counter);
    let excerpt_html = split_excerpt(&body_html, fm.description.as_deref());
    Some(Post {
        slug,
        title: fm.title,
        date,
        description: fm.description,
        source: fm.source,
        tags: fm.tags,
        excerpt_html,
        body_html,
    })
}

// ── Project pages ─────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct ProjectFrontmatter {
    title: String,
    slug: String,
    github: String,
    tagline: String,
    #[serde(default)]
    activity: ProjectActivityFrontmatter,
}

/// All activity flags default to `true` — only set fields you want to disable.
#[derive(Deserialize)]
struct ProjectActivityFrontmatter {
    #[serde(default = "bool_true")]
    release: bool,
    #[serde(default = "bool_true")]
    recent_commits: bool,
    #[serde(default = "bool_true")]
    open_prs: bool,
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

fn bool_true() -> bool {
    true
}

struct ProjectPageData {
    slug: String,
    title: String,
    github: String,
    tagline: String,
    activity_release: bool,
    activity_recent_commits: bool,
    activity_open_prs: bool,
    body_html: String,
}

fn collect_project_pages(dir: &Path) -> Vec<ProjectPageData> {
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

fn process_project_file(path: &Path) -> Option<ProjectPageData> {
    println!("cargo:rerun-if-changed={}", path.display());
    let content = fs::read_to_string(path).ok()?;
    let (fm, body_md) = parse_project_frontmatter(&content)?;
    let body_html = render_markdown(body_md);
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

fn parse_project_frontmatter(content: &str) -> Option<(ProjectFrontmatter, &str)> {
    let content = content.trim_start();
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

fn generate_project_pages_code(pages: &[ProjectPageData]) -> String {
    let mut code = String::new();
    writeln!(code, "pub static PROJECT_PAGES: &[ProjectPage] = &[").unwrap();
    for page in pages {
        writeln!(code, "    ProjectPage {{").unwrap();
        writeln!(code, "        slug: {:?},", page.slug).unwrap();
        writeln!(code, "        title: {:?},", page.title).unwrap();
        writeln!(code, "        github: {:?},", page.github).unwrap();
        writeln!(code, "        tagline: {:?},", page.tagline).unwrap();
        writeln!(code, "        activity: ActivityConfig {{").unwrap();
        writeln!(code, "            release: {:?},", page.activity_release).unwrap();
        writeln!(
            code,
            "            recent_commits: {:?},",
            page.activity_recent_commits
        )
        .unwrap();
        writeln!(code, "            open_prs: {:?},", page.activity_open_prs).unwrap();
        writeln!(code, "        }},").unwrap();
        writeln!(code, "        body_html: {:?},", page.body_html).unwrap();
        writeln!(code, "    }},").unwrap();
    }
    writeln!(code, "];").unwrap();
    code
}

// ── Code generation ───────────────────────────────────────────────────────

fn generate_code(posts: &[Post]) -> String {
    let mut code = String::new();
    writeln!(code, "pub static POSTS: &[BlogPost] = &[").unwrap();
    for post in posts {
        writeln!(code, "    BlogPost {{").unwrap();
        writeln!(code, "        slug: {:?},", post.slug).unwrap();
        writeln!(code, "        title: {:?},", post.title).unwrap();
        writeln!(code, "        date: {:?},", post.date).unwrap();
        match &post.description {
            Some(d) => writeln!(code, "        description: Some({d:?}),").unwrap(),
            None => writeln!(code, "        description: None,").unwrap(),
        }
        match &post.source {
            Some(s) => writeln!(code, "        source: Some({s:?}),").unwrap(),
            None => writeln!(code, "        source: None,").unwrap(),
        }
        write!(code, "        tags: &[").unwrap();
        for tag in &post.tags {
            write!(code, "{tag:?}, ").unwrap();
        }
        writeln!(code, "],").unwrap();
        writeln!(code, "        excerpt_html: {:?},", post.excerpt_html).unwrap();
        writeln!(code, "        body_html: {:?},", post.body_html).unwrap();
        writeln!(code, "    }},").unwrap();
    }
    writeln!(code, "];").unwrap();
    code
}

#[cfg(test)]
mod project_tests {
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
