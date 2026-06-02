//! CIE L\*C\*h° cylindrical colour space derived from CIELAB (D65 illuminant).

use crate::colour::Axis;
use crate::colour::ColourSpace;
use crate::colour::SpaceShape;
use crate::colour::cie_lab::cie_lab_to_linear_rgb;
use crate::colour::cie_lab::linear_rgb_to_cie_lab;
use crate::colour::parse;

/// The CIE L\*C\*h° cylindrical colour space derived from CIELAB (D65 illuminant).
#[non_exhaustive]
pub struct CieLch;

/// Convert CIELAB coordinates to cylindrical LCh representation.
fn lab_to_lch(lab: [f32; 3]) -> [f32; 3] {
    let [l, a, b] = lab;
    let c = (a * a + b * b).sqrt();
    let h = b.atan2(a).to_degrees().rem_euclid(360.0) / 360.0;
    [l, c, h]
}

/// Convert cylindrical LCh coordinates back to CIELAB.
fn lch_to_lab(lch: [f32; 3]) -> [f32; 3] {
    let [l, c, h] = lch;
    let h_rad = h * 360.0_f32.to_radians();
    [l, c * h_rad.cos(), c * h_rad.sin()]
}

impl ColourSpace for CieLch {
    fn name(&self) -> &'static str {
        "CIE LCh"
    }

    fn axes(&self) -> [Axis; 3] {
        [
            Axis {
                name: "L*",
                range: (0.0, 100.0),
                circular: false,
            },
            Axis {
                name: "C*",
                range: (0.0, 180.0),
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
        let lab = lch_to_lab(coord);
        let linear = cie_lab_to_linear_rgb(lab);
        linear
            .iter()
            .all(|&c| (-1e-4..=1.0 + 1e-4).contains(&c))
            .then_some(linear)
    }

    fn from_linear_rgb(&self, linear_rgb: [f32; 3]) -> [f32; 3] {
        lab_to_lch(linear_rgb_to_cie_lab(linear_rgb))
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
                lab_to_lch(linear_rgb_to_cie_lab(linear))
            })
            .or_else(|| parse::css_function(input, "lch", &self.axes()))
            .or_else(|| parse::bare_values(input, &self.axes()))
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::colour::ColourSpace;

    #[test]
    fn black_is_l0() {
        let coord = CieLch.from_linear_rgb([0.0, 0.0, 0.0]);
        assert_abs_diff_eq!(coord[0], 0.0, epsilon = 1e-3);
    }

    #[test]
    fn white_is_l100() {
        let coord = CieLch.from_linear_rgb([1.0, 1.0, 1.0]);
        assert_abs_diff_eq!(coord[0], 100.0, epsilon = 1e-2);
        assert_abs_diff_eq!(coord[1], 0.0, epsilon = 1e-2);
    }

    #[test]
    fn round_trip() {
        let linear = [0.6274_f32, 0.0399, 0.0];
        let coord = CieLch.from_linear_rgb(linear);
        let back = CieLch.to_linear_rgb(coord).unwrap();
        assert_abs_diff_eq!(back[0], linear[0], epsilon = 1e-4);
        assert_abs_diff_eq!(back[1], linear[1], epsilon = 1e-4);
        assert_abs_diff_eq!(back[2], linear[2], epsilon = 1e-4);
    }
}
