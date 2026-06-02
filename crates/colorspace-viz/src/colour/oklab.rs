//! OkLab perceptually uniform colour space (Björn Ottosson, 2020).

use crate::colour::Axis;
use crate::colour::ColourSpace;
use crate::colour::SpaceShape;
use crate::colour::parse;

/// The OkLab perceptually uniform colour space (Björn Ottosson, 2020).
#[non_exhaustive]
pub struct OkLab;

/// Convert linear sRGB to OkLab using the Ottosson M1 and M2 matrices.
pub(crate) fn linear_rgb_to_oklab(rgb: [f32; 3]) -> [f32; 3] {
    let [r, g, b] = rgb;
    // Linear sRGB → LMS (Ottosson M1)
    let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
    let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
    let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

    // Cube root
    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    // LMS → OkLab (Ottosson M2)
    [
        0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
        1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
        0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
    ]
}

/// Convert OkLab back to linear sRGB using the inverse Ottosson matrices.
pub(crate) fn oklab_to_linear_rgb(lab: [f32; 3]) -> [f32; 3] {
    let [l, a, b] = lab;
    // OkLab → cube-root LMS (inverse M2)
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b;

    // Cube
    let lc = l_ * l_ * l_;
    let mc = m_ * m_ * m_;
    let sc = s_ * s_ * s_;

    // LMS → linear sRGB (inverse M1)
    [
        4.0767416621 * lc - 3.3077115913 * mc + 0.2309699292 * sc,
        -1.2684380046 * lc + 2.6097574011 * mc - 0.3413193965 * sc,
        -0.0041960863 * lc - 0.7034186147 * mc + 1.7076147010 * sc,
    ]
}

fn is_in_srgb_gamut(linear: [f32; 3]) -> bool {
    linear.iter().all(|&c| (-1e-4..=1.0 + 1e-4).contains(&c))
}

impl ColourSpace for OkLab {
    fn name(&self) -> &'static str {
        "OkLab"
    }

    fn axes(&self) -> [Axis; 3] {
        [
            Axis {
                name: "L",
                range: (0.0, 1.0),
                circular: false,
            },
            Axis {
                name: "a",
                range: (-0.5, 0.5),
                circular: false,
            },
            Axis {
                name: "b",
                range: (-0.5, 0.5),
                circular: false,
            },
        ]
    }

    fn shape(&self) -> SpaceShape {
        SpaceShape::GamutClipped
    }

    fn to_linear_rgb(&self, coord: [f32; 3]) -> Option<[f32; 3]> {
        let linear = oklab_to_linear_rgb(coord);
        is_in_srgb_gamut(linear).then_some(linear)
    }

    fn from_linear_rgb(&self, linear_rgb: [f32; 3]) -> [f32; 3] {
        linear_rgb_to_oklab(linear_rgb)
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
                linear_rgb_to_oklab(linear)
            })
            .or_else(|| parse::css_function(input, "oklab", &self.axes()))
            .or_else(|| parse::bare_values(input, &self.axes()))
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::colour::ColourSpace;

    #[test]
    fn black_maps_to_zero_lab() {
        let coord = OkLab.from_linear_rgb([0.0, 0.0, 0.0]);
        assert_abs_diff_eq!(coord[0], 0.0, epsilon = 1e-4); // L = 0
        assert_abs_diff_eq!(coord[1], 0.0, epsilon = 1e-4); // a ≈ 0
        assert_abs_diff_eq!(coord[2], 0.0, epsilon = 1e-4); // b ≈ 0
    }

    #[test]
    fn white_maps_to_l1_ab0() {
        let coord = OkLab.from_linear_rgb([1.0, 1.0, 1.0]);
        assert_abs_diff_eq!(coord[0], 1.0, epsilon = 1e-3); // L = 1
        assert_abs_diff_eq!(coord[1], 0.0, epsilon = 1e-3); // a ≈ 0
        assert_abs_diff_eq!(coord[2], 0.0, epsilon = 1e-3); // b ≈ 0
    }

    #[test]
    fn round_trip_orange() {
        // #d43400 in linear sRGB (approx)
        let linear = [0.6274, 0.0399, 0.0];
        let coord = OkLab.from_linear_rgb(linear);
        let back = OkLab.to_linear_rgb(coord).unwrap();
        assert_abs_diff_eq!(back[0], linear[0], epsilon = 1e-4);
        assert_abs_diff_eq!(back[1], linear[1], epsilon = 1e-4);
        assert_abs_diff_eq!(back[2], linear[2], epsilon = 1e-4);
    }
}
