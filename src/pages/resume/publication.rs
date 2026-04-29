use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Publication {
    pub authors: String,
    pub title: String,
    pub journal: String,
    pub year: u32,
    pub doi: Option<String>,
    pub arxiv: Option<String>,
}
