//! Interactive colour space visualiser for the browser.
//!
//! Provides a WebAssembly entry point ([`init`]) that mounts a 3-D WebGL render
//! of the gamut solid for several colour spaces, along with 2-D cross-section
//! slice panels and an interactive colour-line picker.

// MARK: Crate-level lint suppressions
//
// These lints are suppressed for the entire crate because they describe
// intentional patterns specific to colour math and browser WASM code.

// Colour math and GPU geometry rely on float arithmetic throughout.
#![expect(
    clippy::float_arithmetic,
    reason = "colour math requires floating-point arithmetic"
)]
// Matrix coefficients (OkLab, CIE XYZ, …) are scientific constants that must
// not be reformatted with underscores.
#![expect(
    clippy::unreadable_literal,
    reason = "colour-space matrix coefficients are scientific constants"
)]
#![expect(
    clippy::excessive_precision,
    reason = "colour-space matrix coefficients require full scientific precision"
)]
// GPU geometry mixes usize, u32, and f32 heavily.
#![expect(
    clippy::cast_possible_truncation,
    reason = "GPU buffer sizes and loop indices always fit in u32 for the grid sizes used"
)]
#![expect(
    clippy::cast_precision_loss,
    reason = "loop counter to f32 for GPU geometry; precision is acceptable for visual output"
)]
// Generic "as" conversion lint is too broad for geometry/colour code.
#![expect(
    clippy::as_conversions,
    reason = "intentional float/int conversions throughout colour math and GPU geometry"
)]
// Geometry code and hex parsing index fixed-size arrays and ASCII slices by design.
#![expect(
    clippy::indexing_slicing,
    reason = "geometry arrays and ASCII hex-string byte slices are indexed by design"
)]
// Integer arithmetic in geometry/GPU context; overflow is not a concern.
#![expect(
    clippy::arithmetic_side_effects,
    reason = "geometry and GPU index arithmetic; integer overflow is not a concern here"
)]
// WASM DOM operations: failure means the page is fatally broken.
#![expect(
    clippy::expect_used,
    reason = "WASM DOM operations use expect() because failure signals a catastrophically broken page"
)]
// All public panic sites are WASM DOM calls that cannot fail in normal browser use.
#![expect(
    clippy::missing_panics_doc,
    reason = "public functions only panic via DOM expect() calls that cannot fail in a running browser"
)]
// Casts from signed integer types to usize/u8 are guarded by range checks at the call site.
#![expect(
    clippy::cast_sign_loss,
    reason = "all sign-sensitive casts are preceded by explicit range clamps or guards"
)]

use wasm_bindgen::prelude::*;

/// Colour math primitives for all supported colour spaces.
pub mod colour;

pub mod render;
#[cfg(target_arch = "wasm32")]
mod ui;

/// Initialise and mount the colour space visualiser into the page.
///
/// Call this once after the DOM is ready. It finds the `#colorspace-viz`
/// element, attaches the WebGL canvas, slice panels, and event listeners,
/// then starts the render loop.
#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
    #[cfg(target_arch = "wasm32")]
    {
        use std::cell::RefCell;
        use std::rc::Rc;

        let window = web_sys::window().expect("no window");
        let document = window.document().expect("no document");

        let app = ui::mount(&document);

        let state = Rc::new(RefCell::new(render::RenderState::new()));

        for i in 0..3 {
            ui::slices::render_slice(&app.slices[i].canvas, colour::SPACES[0], i, 0.5, None);
        }

        ui::events::wire_all(&document, &app, Rc::clone(&state));
        render::start_loop(app.canvas, state);
    }
}
