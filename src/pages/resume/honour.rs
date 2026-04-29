use leptos::prelude::*;
use serde::Deserialize;
use stylance::import_style;

import_style!(style, "honour.module.scss");

/// An honour or award entry on the CV.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Honour {
    /// Year the award was received.
    pub year: u32,
    /// Name of the award.
    pub award: String,
    /// Organisation that granted the award.
    pub org: String,
}

/// One row in the Honours & Awards compact grid.
///
/// # Arguments
///
/// * `honour` - The honour data to render.
/// * `index` - Row index (0 gets `rule-section` weight, others get `rule-list`).
#[component]
pub fn HonourRow(honour: Honour, index: usize) -> impl IntoView {
    let border = if index == 0 {
        "rule-section"
    } else {
        "rule-list"
    };
    view! {
        <div class=format!("{} {}", style::honour_row, border)>
            <span class="eyebrow--muted">{honour.year.to_string()}</span>
            <span>{honour.award}</span>
            <span class="eyebrow--muted">{honour.org}</span>
        </div>
    }
}
