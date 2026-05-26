#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"

[dependencies]
image = "0.25"
ico = "0.3"
---

//! Generates derivative favicon assets from `public/favicon.png`.
//!
//! Must be run from the repository root:
//!
//!   ./scripts/generate-favicons.rs

use std::io::BufWriter;

use image::imageops::FilterType;

fn main() {
    let src = image::open("public/favicon.png")
        .expect("failed to open public/favicon.png — run from repo root");

    // PNG derivatives: (output_path, size_px)
    let pngs: &[(&str, u32)] = &[
        ("public/favicon-16x16.png", 16),
        ("public/favicon-32x32.png", 32),
        ("public/apple-touch-icon.png", 180),
        ("public/android-chrome-192x192.png", 192),
        ("public/android-chrome-512x512.png", 512),
        ("public/mstile-150x150.png", 150),
    ];

    for (path, size) in pngs {
        let resized = src.resize_exact(*size, *size, FilterType::Lanczos3);
        resized.save(path).unwrap_or_else(|e| panic!("failed to save {path}: {e}"));
        println!("  wrote {path} ({size}×{size})");
    }

    // favicon.ico: multi-resolution (16, 32, 48 px layers)
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    for size in [16u32, 32, 48] {
        let resized = src.resize_exact(size, size, FilterType::Lanczos3).into_rgba8();
        let (w, h) = resized.dimensions();
        let icon_image = ico::IconImage::from_rgba_data(w, h, resized.into_raw());
        icon_dir.add_entry(
            ico::IconDirEntry::encode(&icon_image).unwrap_or_else(|e| panic!("ICO encode error: {e}")),
        );
    }
    let ico_path = "public/favicon.ico";
    let ico_file = std::fs::File::create(ico_path).expect("failed to create favicon.ico");
    icon_dir
        .write(BufWriter::new(ico_file))
        .expect("failed to write favicon.ico");
    println!("  wrote {ico_path} (16×16, 32×32, 48×48)");

    println!("Done — {} files written", pngs.len() + 1);
}
