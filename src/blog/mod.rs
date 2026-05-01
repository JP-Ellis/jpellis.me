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

pub fn format_date(date: &str) -> String {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() < 2 {
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
    format!("{} {}", month, parts[0])
}
