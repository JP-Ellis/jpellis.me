//! OkLCh cylindrical colour space derived from OkLab (Björn Ottosson, 2020).

use crate::colour::Axis;
use crate::colour::ColourSpace;
use crate::colour::SpaceShape;
use crate::colour::oklab::linear_rgb_to_oklab;
use crate::colour::oklab::oklab_to_linear_rgb;
use crate::colour::parse;

/// The OkLCh cylindrical colour space derived from OkLab (Björn Ottosson, 2020).
#[non_exhaustive]
pub struct OkLch;

/// Convert OkLab coordinates to cylindrical OkLCh representation.
pub(crate) fn oklab_to_oklch(lab: [f32; 3]) -> [f32; 3] {
    let [l, a, b] = lab;
    let c = (a * a + b * b).sqrt();
    let h = b.atan2(a).to_degrees().rem_euclid(360.0) / 360.0;
    [l, c, h]
}

/// Convert cylindrical OkLCh coordinates back to OkLab.
pub(crate) fn oklch_to_oklab(lch: [f32; 3]) -> [f32; 3] {
    let [l, c, h] = lch;
    let h_rad = h * 360.0_f32.to_radians();
    [l, c * h_rad.cos(), c * h_rad.sin()]
}

impl ColourSpace for OkLch {
    fn name(&self) -> &'static str {
        "OkLCh"
    }

    fn axes(&self) -> [Axis; 3] {
        [
            Axis {
                name: "L",
                range: (0.0, 1.0),
                circular: false,
            },
            Axis {
                name: "C",
                range: (0.0, 0.4),
                circular: false,
            },
            Axis {
                name: "h",
                range: (0.0, 1.0),
                circular: true,
            },
        ]
    }

    fn shape(&self) -> SpaceShape {
        SpaceShape::GamutClipped
    }

    fn to_linear_rgb(&self, coord: [f32; 3]) -> Option<[f32; 3]> {
        let lab = oklch_to_oklab(coord);
        let linear = oklab_to_linear_rgb(lab);
        linear
            .iter()
            .all(|&c| (-1e-4..=1.0 + 1e-4).contains(&c))
            .then_some(linear)
    }

    fn from_linear_rgb(&self, linear_rgb: [f32; 3]) -> [f32; 3] {
        oklab_to_oklch(linear_rgb_to_oklab(linear_rgb))
    }

    fn parse_css(&self, input: &str) -> Option<[f32; 3]> {
        parse::hex(input)
            .map(|srgb| {
                use crate::colour::srgb::srgb_channel_to_linear;
                let linear = [
                    srgb_channel_to_linear(srgb[0]),
                    srgb_channel_to_linear(srgb[1]),
                    srgb_channel_to_linear(srgb[2]),
                ];
                oklab_to_oklch(linear_rgb_to_oklab(linear))
            })
            .or_else(|| parse::css_function(input, "oklch", &self.axes()))
            .or_else(|| parse::bare_values(input, &self.axes()))
    }
}
