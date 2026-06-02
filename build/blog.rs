//! Blog post collection and Rust code generation.
//!
//! Walks `content/blog/`, parses TOML frontmatter and Markdown bodies, and
//! emits a `static POSTS: &[BlogPost]` array for inclusion in the main crate.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use toml::value::Datetime;

use super::markdown::postprocess_pymdownx;
use super::markdown::render;

/// TOML frontmatter fields for a blog post.
#[derive(Deserialize)]
pub struct Frontmatter {
    /// Post title shown in listings and the post header.
    pub title: String,
    /// Publication date; used for sorting and display.
    pub date: Datetime,
    /// Optional short description used as the excerpt when no `<!-- more -->` marker exists.
    #[serde(default)]
    pub description: Option<String>,
    /// Optional URL pointing to the original source of the post.
    #[serde(default)]
    pub source: Option<String>,
    /// Classification tags for the post.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Explicit URL slug; derived from the filename when absent.
    #[serde(default)]
    pub slug: Option<String>,
}

/// A fully processed blog post ready for code generation.
pub struct Post {
    /// URL-safe slug used to construct the post's route.
    pub slug: String,
    /// Display title of the post.
    pub title: String,
    /// ISO 8601 date string (from the TOML `Datetime`).
    pub date: String,
    /// Optional short description of the post.
    pub description: Option<String>,
    /// Optional URL to the original source.
    pub source: Option<String>,
    /// Classification tags.
    pub tags: Vec<String>,
    /// HTML excerpt shown in post listings.
    pub excerpt_html: String,
    /// Full HTML body of the post.
    pub body_html: String,
}

/// Parse TOML frontmatter and the Markdown body from raw file content.
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
pub fn parse_frontmatter(raw_content: &str) -> Option<(Frontmatter, &str)> {
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

    let fm: Frontmatter = toml::from_str(fm_str).ok()?;
    Some((fm, body))
}

/// Derive a URL slug from a file path or an explicit frontmatter override.
///
/// If no slug is supplied, the file stem is used. A numeric date prefix of the
/// form `NN-NN ` (e.g. `01-15 `) is stripped before slugification.
///
/// # Arguments
///
/// * `path` - Path to the Markdown file.
/// * `fm_slug` - Explicit slug from frontmatter, if present.
///
/// # Returns
///
/// A lowercase, hyphen-separated slug string.
#[expect(
    clippy::string_slice,
    reason = "Indexing based on found byte offsets, or known delimiter lengths"
)]
pub fn derive_slug(path: &Path, fm_slug: Option<&str>) -> String {
    if let Some(slug) = fm_slug {
        return slug.to_owned();
    }

    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let short_stem = if stem.len() > 6
        && stem[..2].chars().all(|c| c.is_ascii_digit())
        && stem.chars().nth(2) == Some('-')
        && stem[3..5].chars().all(|c| c.is_ascii_digit())
        && stem.chars().nth(5) == Some(' ')
    {
        &stem[6..]
    } else {
        stem
    };

    let slug: String = short_stem
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

/// Remove all heading elements (`<h1>`–`<h6>`) from an HTML string.
///
/// # Arguments
///
/// * `html` - HTML string possibly containing heading elements.
///
/// # Returns
///
/// A new `String` with all heading tags and their content removed.
#[expect(
    clippy::string_slice,
    clippy::arithmetic_side_effects,
    reason = "Indexing based on found byte offsets returned by String::find"
)]
fn strip_headings(html: &str) -> String {
    let mut result = html.to_owned();
    for n in 1_u8..=6 {
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
                    let prefix = result[..start].trim_end().to_owned();
                    let suffix = result[end..].trim_start().to_owned();
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
    result.trim().to_owned()
}

/// Extract the excerpt HTML from a post's rendered body.
///
/// Resolution order:
/// 1. HTML up to the first `<!-- more -->` marker (headings stripped).
/// 2. `<p>description</p>` if a description is provided in frontmatter.
/// 3. The first `<p>…</p>` paragraph in the body.
/// 4. The full body as a fallback.
///
/// # Arguments
///
/// * `body_html` - The full rendered HTML body.
/// * `description` - Optional frontmatter description text.
///
/// # Returns
///
/// An HTML string suitable for use as a post excerpt.
#[expect(
    clippy::string_slice,
    clippy::arithmetic_side_effects,
    reason = "Indexing based on found byte offsets returned by str::find"
)]
fn split_excerpt(body_html: &str, description: Option<&str>) -> String {
    if let Some(pos) = body_html.find("<!-- more -->") {
        return strip_headings(body_html[..pos].trim());
    }
    if let Some(desc) = description {
        return format!("<p>{desc}</p>");
    }
    if let Some(start) = body_html.find("<p>")
        && let Some(rel_end) = body_html[start..].find("</p>")
    {
        return body_html[start..start + rel_end + 4].to_owned();
    }
    body_html.to_owned()
}

/// Collect all blog posts under `dir`, processing each Markdown file found.
///
/// Traverses subdirectories recursively. Files that fail to parse are skipped
/// with a `cargo:warning` diagnostic.
///
/// # Arguments
///
/// * `dir` - Root directory to search for `.md` files.
/// * `tab_counter` - Shared counter for generating unique tab group IDs.
///
/// # Returns
///
/// A `Vec<Post>` in filesystem traversal order (not yet sorted by date).
pub fn collect_posts(dir: &Path, tab_counter: &mut usize) -> Vec<Post> {
    let mut posts = Vec::new();
    visit_dir(dir, &mut posts, tab_counter);
    posts
}

/// Recursively walk `dir` and push a [`Post`] for each valid `.md` file found.
///
/// # Arguments
///
/// * `dir` - Directory to walk.
/// * `posts` - Accumulator for processed posts.
/// * `tab_counter` - Shared counter for tab group ID generation.
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

/// Parse a single Markdown file into a [`Post`].
///
/// Returns `None` if the file cannot be read or its frontmatter is invalid.
///
/// # Arguments
///
/// * `path` - Path to the `.md` file.
/// * `tab_counter` - Shared counter for tab group ID generation.
///
/// # Returns
///
/// `Some(Post)` on success, `None` on parse failure.
fn process_file(path: &Path, tab_counter: &mut usize) -> Option<Post> {
    let content = fs::read_to_string(path).ok()?;
    let (fm, body_md) = parse_frontmatter(&content)?;
    let slug = derive_slug(path, fm.slug.as_deref());
    let date = fm.date.to_string();
    let raw_html = render(body_md);
    let body_html = postprocess_pymdownx(&raw_html, tab_counter);
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

/// Generate a Rust source file containing a `static POSTS: &[BlogPost]` array.
///
/// # Arguments
///
/// * `posts` - Slice of processed blog posts to emit.
///
/// # Returns
///
/// A `String` containing valid Rust source code for inclusion via `include!`.
pub fn generate_code(posts: &[Post]) -> String {
    let mut code = String::new();
    writeln!(
        code,
        "/// Compiled blog posts (generated by the build script from Markdown)."
    )
    .expect("writing to String is infallible");
    writeln!(code, "pub static POSTS: &[BlogPost] = &[").expect("writing to String is infallible");
    for post in posts {
        writeln!(code, "    BlogPost {{").expect("writing to String is infallible");
        writeln!(code, "        slug: {:?},", post.slug).expect("writing to String is infallible");
        writeln!(code, "        title: {:?},", post.title)
            .expect("writing to String is infallible");
        writeln!(code, "        date: {:?},", post.date).expect("writing to String is infallible");
        match &post.description {
            Some(d) => writeln!(code, "        description: Some({d:?}),")
                .expect("writing to String is infallible"),
            None => writeln!(code, "        description: None,")
                .expect("writing to String is infallible"),
        }
        match &post.source {
            Some(s) => writeln!(code, "        source: Some({s:?}),")
                .expect("writing to String is infallible"),
            None => {
                writeln!(code, "        source: None,").expect("writing to String is infallible");
            }
        }
        write!(code, "        tags: &[").expect("writing to String is infallible");
        for tag in &post.tags {
            write!(code, "{tag:?}, ").expect("writing to String is infallible");
        }
        writeln!(code, "],").expect("writing to String is infallible");
        writeln!(code, "        excerpt_html: {:?},", post.excerpt_html)
            .expect("writing to String is infallible");
        writeln!(code, "        body_html: {:?},", post.body_html)
            .expect("writing to String is infallible");
        writeln!(code, "    }},").expect("writing to String is infallible");
    }
    writeln!(code, "];").expect("writing to String is infallible");
    code
}
