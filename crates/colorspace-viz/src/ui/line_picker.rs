use web_sys::HtmlElement;

use crate::colour::ColourSpace;
use crate::colour::srgb::linear_to_srgb_channel;
use crate::render::geometry::map_to_gamut;
use crate::render::geometry::oog_coord_to_linear_by_idx;

#[expect(
    clippy::cast_sign_loss,
    reason = "clamped to [0.0, 1.0] before multiplying by 255 — always non-negative"
)]
fn linear_to_css_hex(linear: [f32; 3]) -> String {
    let r = (linear_to_srgb_channel(linear[0]).clamp(0.0, 1.0) * 255.0) as u8;
    let g = (linear_to_srgb_channel(linear[1]).clamp(0.0, 1.0) * 255.0) as u8;
    let b = (linear_to_srgb_channel(linear[2]).clamp(0.0, 1.0) * 255.0) as u8;
    format!("#{r:02x}{g:02x}{b:02x}")
}

/// Update the gradient bar to show the colour path from `a` to `b` through `space`.
///
/// Samples 16 evenly-spaced points along the straight line from `a` to `b` in
/// colour-space coordinates, converts each to sRGB hex, and sets a CSS
/// `linear-gradient` on the element's `background` style.
///
/// When `fallback` is `true`, out-of-gamut points are shown at their CSS
/// gamut-mapped colour instead of grey.
pub fn update_gradient_bar(
    bar: &HtmlElement,
    a: [f32; 3],
    b: [f32; 3],
    space_idx: usize,
    fallback: bool,
) {
    let space = crate::colour::SPACES[space_idx];
    const N: usize = 16;
    let mut stops: Vec<String> = Vec::with_capacity(N);
    for i in 0..N {
        let t = i as f32 / (N - 1) as f32;
        let coord = [
            a[0] + t * (b[0] - a[0]),
            a[1] + t * (b[1] - a[1]),
            a[2] + t * (b[2] - a[2]),
        ];
        let rgb = match space.to_linear_rgb(coord) {
            Some(rgb) => rgb,
            None if fallback => {
                let raw_linear = oog_coord_to_linear_by_idx(coord, space_idx);
                map_to_gamut(raw_linear).unwrap_or([0.5, 0.5, 0.5])
            }
            None => [0.5, 0.5, 0.5],
        };
        stops.push(linear_to_css_hex(rgb));
    }
    let gradient = format!("linear-gradient(to right, {})", stops.join(", "));
    bar.style()
        .set_property("background", &gradient)
        .expect("set background");
}

/// Clear the gradient bar back to its default placeholder appearance.
pub fn clear_gradient_bar(bar: &HtmlElement) {
    bar.style()
        .remove_property("background")
        .expect("remove background");
}
