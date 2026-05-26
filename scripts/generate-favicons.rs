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

    let pngs: &[(&str, u32)] = &[
        ("public/favicon-16x16.png", 16),
        ("public/favicon-32x32.png", 32),
        ("public/apple-touch-icon.png", 180),
        ("public/apple-touch-icon-precomposed.png", 180),
        ("public/apple-touch-icon-60x60.png", 60),
        ("public/apple-touch-icon-60x60-precomposed.png", 60),
        ("public/apple-touch-icon-76x76.png", 76),
        ("public/apple-touch-icon-76x76-precomposed.png", 76),
        ("public/apple-touch-icon-120x120.png", 120),
        ("public/apple-touch-icon-120x120-precomposed.png", 120),
        ("public/apple-touch-icon-152x152.png", 152),
        ("public/apple-touch-icon-152x152-precomposed.png", 152),
        ("public/apple-touch-icon-180x180.png", 180),
        ("public/apple-touch-icon-180x180-precomposed.png", 180),
        ("public/android-chrome-36x36.png", 36),
        ("public/android-chrome-48x48.png", 48),
        ("public/android-chrome-72x72.png", 72),
        ("public/android-chrome-96x96.png", 96),
        ("public/android-chrome-144x144.png", 144),
        ("public/android-chrome-192x192.png", 192),
        ("public/android-chrome-256x256.png", 256),
        ("public/android-chrome-384x384.png", 384),
        ("public/android-chrome-512x512.png", 512),
        ("public/mstile-150x150.png", 150),
    ];

    for (path, size) in pngs {
        let resized = src.resize_exact(*size, *size, FilterType::Lanczos3);
        resized
            .save(path)
            .unwrap_or_else(|e| panic!("failed to save {path}: {e}"));
        println!("  wrote {path} ({size}×{size})");
    }

    // Generate favicon.ico with 16, 32, 48 px layers
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    for size in [16u32, 32, 48] {
        let resized = src.resize_exact(size, size, FilterType::Lanczos3).into_rgba8();
        let (w, h) = resized.dimensions();
        let icon_image = ico::IconImage::from_rgba_data(w, h, resized.into_raw());
        icon_dir.add_entry(
            ico::IconDirEntry::encode(&icon_image)
                .unwrap_or_else(|e| panic!("failed to encode ICO entry at {size}px: {e}")),
        );
    }
    let ico_path = "public/favicon.ico";
    let ico_file =
        std::fs::File::create(ico_path).expect("failed to create public/favicon.ico");
    icon_dir
        .write(BufWriter::new(ico_file))
        .expect("failed to write public/favicon.ico");
    println!("  wrote {ico_path} (16×16, 32×32, 48×48)");

    println!("Done — {} files written", pngs.len() + 1);
}
