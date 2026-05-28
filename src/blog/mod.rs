#[derive(Debug)]
pub struct BlogPost {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub description: Option<&'static str>,
    pub source: Option<&'static str>,
    pub tags: &'static [&'static str],
    pub excerpt_html: &'static str,
    pub body_html: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/blog_posts.rs"));

pub fn find_post(slug: &str) -> Option<&'static BlogPost> {
    POSTS.iter().find(|p| p.slug == slug)
}

pub fn source_domain(url: &str) -> Option<&str> {
    url.splitn(3, '/')
        .nth(2)
        .and_then(|rest| rest.split('/').next())
}

pub fn format_date(date: &str) -> String {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() < 3 {
        return date.to_string();
    }
    let month = match parts[1] {
        "01" => "Jan",
        "02" => "Feb",
        "03" => "Mar",
        "04" => "Apr",
        "05" => "May",
        "06" => "Jun",
        "07" => "Jul",
        "08" => "Aug",
        "09" => "Sep",
        "10" => "Oct",
        "11" => "Nov",
        "12" => "Dec",
        _ => parts[1],
    };
    let day = parts[2].trim_start_matches('0');
    format!(
        "{} {} {}",
        if day.is_empty() { "1" } else { day },
        month,
        parts[0]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn posts_sorted_by_date_descending() {
        for window in POSTS.windows(2) {
            assert!(
                window[0].date >= window[1].date,
                "posts out of order: '{}' ({}) should come after '{}' ({})",
                window[0].slug,
                window[0].date,
                window[1].slug,
                window[1].date,
            );
        }
    }

    #[test]
    fn all_required_fields_non_empty() {
        for post in POSTS {
            assert!(!post.slug.is_empty(), "empty slug");
            assert!(!post.title.is_empty(), "empty title in '{}'", post.slug);
            assert!(!post.date.is_empty(), "empty date in '{}'", post.slug);
            assert!(!post.body_html.is_empty(), "empty body in '{}'", post.slug);
            assert!(
                !post.excerpt_html.is_empty(),
                "empty excerpt in '{}'",
                post.slug
            );
        }
    }

    #[test]
    fn slugs_are_unique() {
        let mut slugs: Vec<&str> = POSTS.iter().map(|p| p.slug).collect();
        slugs.sort_unstable();
        let before = slugs.len();
        slugs.dedup();
        pretty_assertions::assert_eq!(slugs.len(), before, "duplicate slugs in POSTS");
    }

    #[test]
    fn dates_are_iso_format() {
        for post in POSTS {
            assert!(
                post.date.len() == 10
                    && post.date.chars().nth(4) == Some('-')
                    && post.date.chars().nth(7) == Some('-'),
                "date '{}' in '{}' is not YYYY-MM-DD",
                post.date,
                post.slug,
            );
        }
    }

    #[test]
    fn find_post_returns_correct_post() {
        assert!(!POSTS.is_empty(), "POSTS must contain at least one post");
        let first = POSTS.first().unwrap();
        let found = find_post(first.slug);
        assert!(found.is_some(), "find_post failed for '{}'", first.slug);
        pretty_assertions::assert_eq!(found.unwrap().slug, first.slug);
    }

    #[test]
    fn find_post_returns_none_for_missing_slug() {
        assert!(find_post("this-slug-does-not-exist-xyz").is_none());
    }

    #[test]
    fn pymdownx_tabs_render_correctly() {
        let post = find_post("functional-arguments").expect("functional-arguments post not found");
        assert!(
            post.body_html.contains(r#"<div class="tabs">"#),
            "Expected tab groups in body_html — first 300 chars: {}",
            &post.body_html[..300.min(post.body_html.len())]
        );
        assert!(
            !post.body_html.contains("<p>/// tab"),
            "Raw /// tab markers must not appear in rendered body_html"
        );
    }

    #[test]
    fn pymdownx_details_render_correctly() {
        let post = find_post("functional-arguments").expect("functional-arguments post not found");
        assert!(
            post.body_html.contains("<details>"),
            "Expected <details> elements in body_html"
        );
        assert!(
            !post.body_html.contains("<p>/// details"),
            "Raw /// details markers must not appear in rendered body_html"
        );
    }
}
