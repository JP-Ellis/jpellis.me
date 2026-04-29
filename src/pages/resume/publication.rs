use leptos::prelude::*;
use serde::Deserialize;
use stylance::import_style;

import_style!(style, "publication.module.scss");

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

/// One row in the Publications section.
///
/// # Arguments
///
/// * `publication` - The publication data to render.
/// * `index` - Row index (0 gets `rule-section` weight, others get `rule-list`).
#[component]
pub fn PublicationRow(publication: Publication, index: usize) -> impl IntoView {
    let border = if index == 0 {
        "rule-section"
    } else {
        "rule-list"
    };
    let doi_link = publication.doi.map(|doi| {
        let url = format!("https://doi.org/{doi}");
        view! {
            <a href=url class="btn" target="_blank" rel="noopener noreferrer">
                "↗ doi"
            </a>
        }
    });
    let arxiv_link = publication.arxiv.map(|arxiv| {
        let url = format!("https://arxiv.org/abs/{arxiv}");
        view! {
            <a href=url class="btn" target="_blank" rel="noopener noreferrer">
                "↗ arXiv"
            </a>
        }
    });
    view! {
        <div class=format!("{} {}", style::publication_row, border)>
            <span class=style::pub_year>{publication.year.to_string()}</span>
            <div>
                <p class=style::pub_authors>{publication.authors}</p>
                <p class=style::pub_title>{publication.title}</p>
                <p class=style::pub_journal>{publication.journal}</p>
                <div class=style::pub_links>{doi_link} {arxiv_link}</div>
            </div>
        </div>
    }
}
