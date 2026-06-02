pub mod blog;
pub mod colorspace_visualizer;
pub mod contact;
pub mod home;
pub mod not_found;
pub mod projects;
pub mod resume;

pub use blog::BlogListPage;
pub use blog::post::BlogPostPage;
pub use colorspace_visualizer::ColorspaceVisualizerPage;
pub use contact::ContactPage;
pub use home::HomePage;
pub use not_found::NotFoundPage;
pub use projects::ProjectsPage;
pub use projects::detail::ProjectDetailPage;
pub use resume::ResumePage;
