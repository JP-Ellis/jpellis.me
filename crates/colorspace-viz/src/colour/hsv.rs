//! HSV (hue–saturation–value) colour space derived from sRGB.

use crate::colour::Axis;
use crate::colour::ColourSpace;
use crate::colour::SpaceShape;
use crate::colour::parse;
use crate::colour::srgb::linear_to_srgb_channel;
use crate::colour::srgb::srgb_channel_to_linear;

/// The HSV (hue–saturation–value) colour space derived from sRGB.
#[non_exhaustive]
pub struct Hsv;

/// Convert sRGB to HSV; hue is normalised to 0–1.
#[expect(
    clippy::float_cmp,
    reason = "exact equality on max is intentional for hue sector selection"
)]
fn srgb_to_hsv(rgb: [f32; 3]) -> [f32; 3] {
    let [r, g, b] = rgb;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let v = max;
    let s = if max < 1e-6 { 0.0 } else { delta / max };
    let h = if delta < 1e-6 {
        0.0
    } else if max == r {
        ((g - b) / delta).rem_euclid(6.0) / 6.0
    } else if max == g {
        ((b - r) / delta + 2.0) / 6.0
    } else {
        ((r - g) / delta + 4.0) / 6.0
    };

    [h, s, v]
}

/// Convert HSV back to sRGB; hue is expected to be normalised 0–1.
fn hsv_to_srgb(hsv: [f32; 3]) -> [f32; 3] {
    let [h, s, v] = hsv;
    let c = v * s;
    let h6 = h * 6.0;
    let x = c * (1.0 - (h6.rem_euclid(2.0) - 1.0).abs());
    let (r1, g1, b1) = match h6 as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    [r1 + m, g1 + m, b1 + m]
}

impl ColourSpace for Hsv {
    fn name(&self) -> &'static str {
        "HSV"
    }

    fn axes(&self) -> [Axis; 3] {
        [
            Axis {
                name: "H",
                range: (0.0, 1.0),
                circular: true,
            },
            Axis {
                name: "S",
                range: (0.0, 1.0),
                circular: false,
            },
            Axis {
                name: "V",
                range: (0.0, 1.0),
                circular: false,
            },
        ]
    }

    fn shape(&self) -> SpaceShape {
        SpaceShape::Cone
    }

    fn to_linear_rgb(&self, coord: [f32; 3]) -> Option<[f32; 3]> {
        let srgb = hsv_to_srgb(coord);
        Some([
            srgb_channel_to_linear(srgb[0].clamp(0.0, 1.0)),
            srgb_channel_to_linear(srgb[1].clamp(0.0, 1.0)),
            srgb_channel_to_linear(srgb[2].clamp(0.0, 1.0)),
        ])
    }

    fn from_linear_rgb(&self, linear_rgb: [f32; 3]) -> [f32; 3] {
        let srgb = [
            linear_to_srgb_channel(linear_rgb[0]),
            linear_to_srgb_channel(linear_rgb[1]),
            linear_to_srgb_channel(linear_rgb[2]),
        ];
        srgb_to_hsv(srgb)
    }

    fn parse_css(&self, input: &str) -> Option<[f32; 3]> {
        parse::hex(input)
            .map(srgb_to_hsv)
            .or_else(|| parse::bare_values(input, &self.axes()))
    }
}
