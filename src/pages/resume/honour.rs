use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Honour {
    pub year: u32,
    pub award: String,
    pub org: String,
}
