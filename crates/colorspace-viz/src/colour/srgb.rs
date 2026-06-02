//! sRGB colour space implementation (IEC 61966-2-1).

use crate::colour::Axis;
use crate::colour::ColourSpace;
use crate::colour::SpaceShape;
use crate::colour::parse;

/// The sRGB colour space (IEC 61966-2-1).
#[non_exhaustive]
pub struct SRgb;

/// Apply the sRGB electro-optical transfer function (gamma expand a single channel).
pub(crate) fn srgb_channel_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Apply the sRGB opto-electronic transfer function (gamma compress a single channel).
pub(crate) fn linear_to_srgb_channel(c: f32) -> f32 {
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

impl ColourSpace for SRgb {
    fn name(&self) -> &'static str {
        "sRGB"
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
        Some([
            srgb_channel_to_linear(r),
            srgb_channel_to_linear(g),
            srgb_channel_to_linear(b),
        ])
    }

    fn from_linear_rgb(&self, linear_rgb: [f32; 3]) -> [f32; 3] {
        let [r, g, b] = linear_rgb;
        [
            linear_to_srgb_channel(r.clamp(0.0, 1.0)),
            linear_to_srgb_channel(g.clamp(0.0, 1.0)),
            linear_to_srgb_channel(b.clamp(0.0, 1.0)),
        ]
    }

    fn parse_css(&self, input: &str) -> Option<[f32; 3]> {
        // sRGB accepts hex and bare r,g,b values (0–1)
        parse::hex(input).or_else(|| parse::bare_values(input, &self.axes()))
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::colour::ColourSpace;

    #[test]
    fn red_round_trips() {
        let space = SRgb;
        let coord = [1.0_f32, 0.0, 0.0];
        let linear = space.to_linear_rgb(coord).unwrap();
        let back = space.from_linear_rgb(linear);
        assert_abs_diff_eq!(back[0], coord[0], epsilon = 1e-5);
        assert_abs_diff_eq!(back[1], coord[1], epsilon = 1e-5);
        assert_abs_diff_eq!(back[2], coord[2], epsilon = 1e-5);
    }

    #[test]
    fn black_is_zero_linear() {
        let space = SRgb;
        let linear = space.to_linear_rgb([0.0, 0.0, 0.0]).unwrap();
        assert_abs_diff_eq!(linear[0], 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(linear[1], 0.0, epsilon = 1e-6);
        assert_abs_diff_eq!(linear[2], 0.0, epsilon = 1e-6);
    }

    #[test]
    fn white_is_one_linear() {
        let space = SRgb;
        let linear = space.to_linear_rgb([1.0, 1.0, 1.0]).unwrap();
        assert_abs_diff_eq!(linear[0], 1.0, epsilon = 1e-5);
        assert_abs_diff_eq!(linear[1], 1.0, epsilon = 1e-5);
        assert_abs_diff_eq!(linear[2], 1.0, epsilon = 1e-5);
    }

    #[test]
    fn out_of_gamut_returns_none() {
        let space = SRgb;
        // Negative values are out of sRGB gamut
        assert!(space.to_linear_rgb([-0.1, 0.5, 0.5]).is_none());
        assert!(space.to_linear_rgb([0.5, 1.1, 0.5]).is_none());
    }
}
