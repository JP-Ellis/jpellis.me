#![expect(
    clippy::shadow_reuse,
    reason = "Leptos #[component] macro internally re-binds function parameters"
)]

use leptos::prelude::*;
use serde::Deserialize;
use stylance::import_style;

import_style!(style, "role.module.scss");

/// A work or education role entry on the CV.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Role {
    /// Start date of the role (e.g. "Sep 2023").
    pub from: String,
    /// End date of the role (e.g. "Present").
    pub to: String,
    /// Organisation or institution name.
    pub org: String,
    /// Location of the role.
    pub loc: String,
    /// Job title or degree name.
    pub title: String,
    /// Short subtitle or technology stack summary.
    pub sub: String,
    /// Main prose body describing the role.
    pub body: String,
    /// Skill/technology tags for this role.
    pub tags: Vec<String>,
    /// Whether this role is highlighted in the featured band.
    #[serde(default)]
    pub featured: bool,
}

/// The highlighted current role, rendered inside the dark Band.
/// Shows a prose paragraph on the left and tag pills on the right.
///
/// # Arguments
///
/// * `role` - The featured role data (must have `featured = true`).
#[component]
pub fn FeaturedRole(
    /// The featured role data to render (must have `featured = true`).
    role: Role,
) -> impl IntoView {
    view! {
        <div class=style::featured_inner>
            <p class=style::featured_body>{role.body}</p>
            <div class=style::featured_tags>
                {role
                    .tags
                    .into_iter()
                    .map(|tag| {
                        view! { <span class="tag tag--pill">{tag}</span> }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

/// One row in the "Earlier" timeline section.
///
/// # Arguments
///
/// * `role` - The role data to render.
/// * `row_index` - Row index (0 gets `rule-section` weight, others get `rule-list`).
#[component]
pub fn TimelineRow(
    /// The role data to render.
    role: Role,
    /// Row index; 0 gets `rule-section` weight, others get `rule-list`.
    row_index: usize,
) -> impl IntoView {
    let border = if row_index == 0 {
        "rule-section"
    } else {
        "rule-list"
    };
    view! {
        <div class=format!("{} {}", style::timeline_row, border)>
            <div class=style::date_col>
                <div class=style::date_from>{role.from}</div>
                <div class=style::date_arrow aria-hidden="true">
                    "↓"
                </div>
                <div class=style::date_to>{role.to}</div>
                <div class=style::date_loc>{role.loc}</div>
            </div>
            <div>
                <div class=format!("{} eyebrow--muted", style::role_org)>{role.org}</div>
                <h3>{role.title}</h3>
                <p class=style::role_sub>{role.sub}</p>
            </div>
            <div>
                <p class=style::role_body>{role.body}</p>
                <div class=style::role_tags>
                    {role
                        .tags
                        .into_iter()
                        .map(|tag| {
                            view! { <span class="tag tag--hash">{tag}</span> }
                        })
                        .collect_view()}
                </div>
            </div>
        </div>
    }
}
