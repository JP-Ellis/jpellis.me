use leptos::prelude::*;
use serde::Deserialize;

mod honour;
mod publication;
mod role;

use honour::Honour;
use publication::Publication;
use role::Role;

#[derive(Deserialize)]
struct ResumeData {
    roles: Vec<Role>,
    publications: Vec<Publication>,
    honours: Vec<Honour>,
}

/// Resume page stub.
#[component]
pub fn ResumePage() -> impl IntoView {
    view! { <p>"TODO"</p> }
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
            assert!(!pub_.title.is_empty());
            assert!(!pub_.authors.is_empty());
            assert!(!pub_.journal.is_empty());
            assert!(pub_.year > 2000);
        }
    }
}
