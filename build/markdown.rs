//! Markdown rendering and PyMdown-compatible post-processing.
//!
//! [`render`] converts Markdown to HTML via pulldown-cmark.
//! [`postprocess_pymdownx`] then scans the resulting HTML for
//! `/// tab | Title` / `/// details | Title` / `///` marker paragraphs and
//! replaces them with CSS-only tab groups or native `<details>` elements.

use std::fmt::Write as _;

/// Render a Markdown string to HTML with common extensions enabled.
///
/// # Arguments
///
/// * `content` - Raw Markdown source.
///
/// # Returns
///
/// An HTML string produced by `pulldown-cmark`.
pub fn render(content: &str) -> String {
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

/// Parse `"type | Title"` or `"type"` into `(block_type, title)`.
///
/// # Arguments
///
/// * `s` - The string after `/// ` (e.g., `"tab | Before"` or `"details | Example"`).
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
/// Returns `Some((inner_html, html_after_closer))`, or `None` if no closer
/// exists. The single `\n` that `pulldown-cmark` emits after `</p>` is stripped
/// from `html_after_closer` automatically.
///
/// # Arguments
///
/// * `html` - The HTML string to search for a closer.
///
/// # Returns
///
/// `Some((inner_html, rest))` if a closer is found; `None` otherwise.
#[expect(
    clippy::string_slice,
    clippy::arithmetic_side_effects,
    reason = "Indexing based on found byte offsets and known constant lengths"
)]
fn find_block_content(html: &str) -> Option<(&str, &str)> {
    const CLOSER: &str = "<p>///</p>";
    let pos = html.find(CLOSER)?;
    let raw_after = &html[pos + CLOSER.len()..];
    let after = raw_after.strip_prefix('\n').unwrap_or(raw_after);
    Some((&html[..pos], after))
}

/// Render consecutive `/// tab` blocks as a CSS-only interactive tab group.
///
/// `tabs` is `(label, inner_html)` pairs. `counter` is incremented once per
/// call to provide unique `name`/`id` attributes across all blog posts.
///
/// # Arguments
///
/// * `tabs` - Slice of `(label, inner_html)` pairs, one per tab.
/// * `counter` - Mutable counter for generating unique IDs; incremented once per call.
///
/// # Returns
///
/// A `String` containing the full tab-group HTML.
#[expect(
    clippy::arithmetic_side_effects,
    reason = "counter is bounded by the number of tab groups in a build, which cannot overflow usize"
)]
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

/// Scan `pulldown-cmark`-rendered HTML for `PyMdown` extension markers and
/// replace them with proper tab-group or details HTML.
///
/// `pulldown-cmark` renders `/// tab | Before` as `<p>/// tab | Before</p>` and
/// `///` (closer) as `<p>///</p>`. This function finds those marker paragraphs,
/// extracts the inner HTML between them, and emits the replacement HTML.
///
/// Consecutive `/// tab` blocks are collapsed into one group.
///
/// # Arguments
///
/// * `html` - The rendered HTML string from `pulldown-cmark`.
/// * `counter` - Mutable counter for generating unique tab group IDs.
///
/// # Returns
///
/// A new `String` with all `PyMdown` markers replaced by proper HTML.
#[expect(
    clippy::string_slice,
    clippy::arithmetic_side_effects,
    reason = "Indexing based on found byte offsets and known constant tag lengths"
)]
pub fn postprocess_pymdownx(html: &str, counter: &mut usize) -> String {
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
        let raw_after_close = &after_open[close_pos + 4..]; // after "</p>"
        let after_close = raw_after_close
            .strip_prefix('\n')
            .unwrap_or(raw_after_close);

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
                let mut tabs = vec![(title.to_owned(), inner_html.to_owned())];
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
                    let raw_after_tab_close = &after_p[ep + 4..];
                    let after_tab_close = raw_after_tab_close
                        .strip_prefix('\n')
                        .unwrap_or(raw_after_tab_close);
                    let ta = opener2.strip_prefix("/// tab").unwrap_or("");
                    let title2 = ta.strip_prefix(" | ").unwrap_or("").trim();
                    let Some((inner2, after_tab_closer)) = find_block_content(after_tab_close)
                    else {
                        break;
                    };
                    tabs.push((title2.to_owned(), inner2.to_owned()));
                    current = after_tab_closer;
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
