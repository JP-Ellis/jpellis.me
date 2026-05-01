use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=content/blog");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir).join("blog_posts.rs");

    fs::write(&out_path, "pub static POSTS: &[BlogPost] = &[];\n").unwrap();
}
