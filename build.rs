use std::env;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use toml::value::Datetime;

fn main() {
    println!("cargo:rerun-if-changed=content/blog");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = PathBuf::from(&out_dir).join("blog_posts.rs");

    let mut posts: Vec<Post> = collect_posts(Path::new("content/blog"));
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    let code = generate_code(&posts);
    fs::write(&out_path, code).expect("failed to write blog_posts.rs");
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

fn collect_posts(dir: &Path) -> Vec<Post> {
    let mut posts = Vec::new();
    visit_dir(dir, &mut posts);
    posts
}

fn visit_dir(dir: &Path, posts: &mut Vec<Post>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        println!("cargo:rerun-if-changed={}", path.display());
        if path.is_dir() {
            visit_dir(&path, posts);
        } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
            if let Some(post) = process_file(&path) {
                posts.push(post);
            } else {
                println!("cargo:warning=Skipped (parse error): {}", path.display());
            }
        }
    }
}

fn process_file(path: &Path) -> Option<Post> {
    let content = fs::read_to_string(path).ok()?;
    let (fm, body_md) = parse_frontmatter(&content)?;
    let slug = derive_slug(path, fm.slug.as_deref());
    let date = fm.date.to_string();
    let body_html = render_markdown(body_md);
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
