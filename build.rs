//! Build script: compiles blog posts and project pages from Markdown into
//! static `&[BlogPost]` / `&[ProjectPage]` arrays that are `include!`-d by
//! the main crate.
#![expect(
    clippy::expect_used,
    reason = "build scripts should panic with a message on fatal errors"
)]
#![expect(
    clippy::use_debug,
    reason = "Debug-formatting string values to emit Rust string literals"
)]

#[path = "build/mod.rs"]
mod build;

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Compile content directory into Rust source files written to `OUT_DIR`.
fn main() {
    println!("cargo:rerun-if-changed=content");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    let blog_out_path = PathBuf::from(&out_dir).join("blog_posts.rs");
    let mut tab_counter = 0_usize;
    let mut posts = build::blog::collect_posts(Path::new("content/blog"), &mut tab_counter);
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    let code = build::blog::generate_code(&posts);
    fs::write(&blog_out_path, code).expect("failed to write blog_posts.rs");

    let pages_out_path = PathBuf::from(&out_dir).join("project_pages.rs");
    let project_pages = build::projects::collect_project_pages(Path::new("content/projects"));
    let pages_code = build::projects::generate_project_pages_code(&project_pages);
    fs::write(&pages_out_path, pages_code).expect("failed to write project_pages.rs");
}
