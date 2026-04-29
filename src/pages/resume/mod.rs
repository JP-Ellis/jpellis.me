use std::sync::LazyLock;

use leptos::prelude::*;
use serde::Deserialize;
use stylance::import_style;

use crate::components::Band;
use crate::components::Footer;
use crate::components::Masthead;

mod honour;
mod publication;
mod role;

use honour::Honour;
use honour::HonourRow;
use publication::Publication;
use publication::PublicationRow;
use role::FeaturedRole;
use role::Role;
use role::TimelineRow;

import_style!(style, "resume.module.scss");

#[derive(Deserialize)]
struct ResumeData {
    roles: Vec<Role>,
    publications: Vec<Publication>,
    honours: Vec<Honour>,
}

// Content is embedded at compile-time via `include_str!`; a panic here is a
// deploy-time programmer error (bad TOML or wrong data), not a runtime user error.
static RESUME: LazyLock<ResumeData> = LazyLock::new(|| {
    let data: ResumeData = toml::from_str(include_str!("../../../content/resume.toml"))
        .expect("content/resume.toml is invalid TOML");
    let featured_count = data.roles.iter().filter(|r| r.featured).count();
    assert_eq!(
        featured_count, 1,
        "content/resume.toml must have exactly one role with featured = true"
    );
    data
});

/// Resume page.
#[component]
pub fn ResumePage() -> impl IntoView {
    let roles = RESUME.roles.clone();
    let publications = RESUME.publications.clone();
    let honours = RESUME.honours.clone();

    let (featured_vec, rest): (Vec<_>, Vec<_>) = roles.into_iter().partition(|r| r.featured);
    let featured = featured_vec
        .into_iter()
        .next()
        .expect("featured role missing — verified in LazyLock");

    view! {
        <Masthead />
        <main>
            // ── Hero ──────────────────────────────────────────────
            <section class=style::hero>
                <div class="container">
                    <p class="eyebrow">"Résumé"</p>
                    <h1>
                        "A working " <em>"history"</em> " — fifteen years between "
                        <span class=style::hero_accent>"physics"</span> " and software."
                    </h1>
                    <p class=style::hero_lead>
                        "Reverse-chronological. A printable "
                        <a
                            href="/JoshuaEllis.pdf"
                            class="btn"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            "PDF"
                        </a> " is also available."
                    </p>
                </div>
            </section>

            // ── Featured role ─────────────────────────────────────
            <Band>
                <div class="container">
                    <div class=style::band_inner>
                        <div class=style::band_header>
                            <div>
                                <span class="eyebrow">"Now"</span>
                                <p class=style::band_title>
                                    {featured.title.clone()} " at " <em>{featured.org.clone()}</em>
                                    "."
                                </p>
                            </div>
                            <span class=style::band_date>
                                {format!("{} — {}", featured.from, featured.to)}
                            </span>
                        </div>
                        <FeaturedRole role=featured />
                    </div>
                </div>
            </Band>

            // ── Timeline ─────────────────────────────────────────
            <section class=style::timeline_section>
                <div class="container">
                    <div class="eyebrow-grid">
                        <span class="eyebrow">"Earlier"</span>
                        <div>
                            {rest
                                .into_iter()
                                .enumerate()
                                .map(|(i, role)| view! { <TimelineRow role=role index=i /> })
                                .collect_view()}
                        </div>
                    </div>
                </div>
            </section>

            // ── Publications ─────────────────────────────────────
            <section class=style::publications_section>
                <div class="container">
                    <div class="eyebrow-grid">
                        <span class="eyebrow">"Publications"</span>
                        <div>
                            {publications
                                .into_iter()
                                .enumerate()
                                .map(|(i, pub_)| {
                                    view! { <PublicationRow publication=pub_ index=i /> }
                                })
                                .collect_view()}
                        </div>
                    </div>
                </div>
            </section>

            // ── Honours & Awards ──────────────────────────────────
            <section class=style::honours_section>
                <div class="container">
                    <div class="eyebrow-grid">
                        <span class="eyebrow">"Honours & Awards"</span>
                        <div>
                            {honours
                                .into_iter()
                                .enumerate()
                                .map(|(i, h)| view! { <HonourRow honour=h index=i /> })
                                .collect_view()}
                        </div>
                    </div>
                </div>
            </section>
        </main>
        <Footer />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_resume() -> ResumeData {
        toml::from_str(include_str!("../../../content/resume.toml"))
            .expect("content/resume.toml should be valid TOML")
    }

    #[test]
    fn resume_toml_parses() {
        let data = parse_resume();
        assert!(!data.roles.is_empty(), "should have at least one role");
        assert!(!data.publications.is_empty(), "should have publications");
        assert!(!data.honours.is_empty(), "should have honours");
    }

    #[test]
    fn exactly_one_featured_role() {
        let data = parse_resume();
        let count = data.roles.iter().filter(|r| r.featured).count();
        assert_eq!(count, 1, "exactly one role must have featured = true");
    }

    #[test]
    fn all_roles_have_required_fields() {
        let data = parse_resume();
        for role in &data.roles {
            assert!(!role.from.is_empty(), "role.from must not be empty");
            assert!(!role.to.is_empty(), "role.to must not be empty");
            assert!(!role.org.is_empty(), "role.org must not be empty");
            assert!(!role.title.is_empty(), "role.title must not be empty");
            assert!(!role.body.is_empty(), "role.body must not be empty");
        }
    }

    #[test]
    fn all_publications_have_required_fields() {
        let data = parse_resume();
        for pub_ in &data.publications {
            assert!(!pub_.title.is_empty(), "pub_.title must not be empty");
            assert!(
                !pub_.authors.is_empty(),
                "pub_.authors must not be empty (title: {})",
                pub_.title
            );
            assert!(
                !pub_.journal.is_empty(),
                "pub_.journal must not be empty (title: {})",
                pub_.title
            );
            assert!(
                pub_.year > 2000,
                "pub_.year must be > 2000, got {}",
                pub_.year
            );
        }
    }
}
