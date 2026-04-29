use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Role {
    pub from: String,
    pub to: String,
    pub org: String,
    pub loc: String,
    pub title: String,
    pub sub: String,
    pub body: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub featured: bool,
}
