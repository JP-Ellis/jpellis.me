//! Linear sRGB colour space (gamma-removed scene-linear light).

use crate::colour::Axis;
use crate::colour::ColourSpace;
use crate::colour::SpaceShape;
use crate::colour::parse;
use crate::colour::srgb::srgb_channel_to_linear;

/// Linear sRGB — the sRGB colour space with gamma removed (scene-linear light).
#[non_exhaustive]
pub struct LinearSRgb;

impl ColourSpace for LinearSRgb {
    fn name(&self) -> &'static str {
        "Linear sRGB"
    }

    fn axes(&self) -> [Axis; 3] {
        [
            Axis {
                name: "R",
                range: (0.0, 1.0),
                circular: false,
            },
            Axis {
                name: "G",
                range: (0.0, 1.0),
                circular: false,
            },
            Axis {
                name: "B",
                range: (0.0, 1.0),
                circular: false,
            },
        ]
    }

    fn shape(&self) -> SpaceShape {
        SpaceShape::Cube
    }

    fn to_linear_rgb(&self, coord: [f32; 3]) -> Option<[f32; 3]> {
        let [r, g, b] = coord;
        if !(0.0..=1.0).contains(&r) || !(0.0..=1.0).contains(&g) || !(0.0..=1.0).contains(&b) {
            return None;
        }
        Some(coord)
    }

    fn from_linear_rgb(&self, linear_rgb: [f32; 3]) -> [f32; 3] {
        let [r, g, b] = linear_rgb;
        [r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0)]
    }

    fn parse_css(&self, input: &str) -> Option<[f32; 3]> {
        // Linear sRGB has no CSS functional notation; accept hex (converts via sRGB gamma)
        // or bare values
        if let Some(srgb) = parse::hex(input) {
            let linear = [
                srgb_channel_to_linear(srgb[0]),
                srgb_channel_to_linear(srgb[1]),
                srgb_channel_to_linear(srgb[2]),
            ];
            return Some(linear);
        }
        parse::bare_values(input, &self.axes())
    }
}
