//! Page-level Leptos components (one per route).
#![expect(
    clippy::pub_use,
    reason = "module facade: re-exporting page components at the `pages` level"
)]

/// Blog listing and individual post pages.
pub mod blog;
/// Contact page with email, PGP, and social links.
pub mod contact;
/// Home / landing page.
pub mod home;
/// 404 not-found page.
pub mod not_found;
/// Projects index and detail pages.
pub mod projects;
/// Résumé page with roles, publications, and honours.
pub mod resume;

pub use blog::BlogListPage;
pub use blog::post::BlogPostPage;
pub use contact::ContactPage;
pub use home::HomePage;
pub use not_found::NotFoundPage;
pub use projects::ProjectsPage;
pub use projects::detail::ProjectDetailPage;
pub use resume::ResumePage;
