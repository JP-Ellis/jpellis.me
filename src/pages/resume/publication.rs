use serde::Deserialize;

/// A peer-reviewed publication entry on the CV.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Publication {
    /// Author list as a formatted string.
    pub authors: String,
    /// Full title of the paper.
    pub title: String,
    /// Journal or conference name.
    pub journal: String,
    /// Year of publication.
    pub year: u32,
    /// DOI identifier, if available (URL derived at render time).
    pub doi: Option<String>,
    /// arXiv identifier, if available (URL derived at render time).
    pub arxiv: Option<String>,
}
