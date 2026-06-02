//! CIE L\*a\*b\* (CIELAB) perceptually uniform colour space (D65 illuminant).

use crate::colour::Axis;
use crate::colour::ColourSpace;
use crate::colour::SpaceShape;
use crate::colour::parse;
use crate::colour::xyz::linear_rgb_to_xyz;
use crate::colour::xyz::xyz_to_linear_rgb;

/// The CIE L\*a\*b\* (CIELAB) perceptually uniform colour space (D65 illuminant).
#[non_exhaustive]
pub struct CieLab;

/// D65 white-point X tristimulus value.
const XN: f32 = 0.95047;
/// D65 white-point Y tristimulus value (luminance = 1).
const YN: f32 = 1.00000;
/// D65 white-point Z tristimulus value.
const ZN: f32 = 1.08883;

/// CIELAB cube-root companding function.
fn f(t: f32) -> f32 {
    const DELTA: f32 = 6.0 / 29.0;
    if t > DELTA * DELTA * DELTA {
        t.cbrt()
    } else {
        t / (3.0 * DELTA * DELTA) + 4.0 / 29.0
    }
}

/// Inverse of the CIELAB cube-root companding function.
fn f_inv(t: f32) -> f32 {
    const DELTA: f32 = 6.0 / 29.0;
    if t > DELTA {
        t * t * t
    } else {
        3.0 * DELTA * DELTA * (t - 4.0 / 29.0)
    }
}

/// Convert linear sRGB to CIE L\*a\*b\* via CIE XYZ and the D65 white point.
pub(crate) fn linear_rgb_to_cie_lab(rgb: [f32; 3]) -> [f32; 3] {
    let [x, y, z] = linear_rgb_to_xyz(rgb);
    let l = 116.0 * f(y / YN) - 16.0;
    let a = 500.0 * (f(x / XN) - f(y / YN));
    let b = 200.0 * (f(y / YN) - f(z / ZN));
    [l, a, b]
}

/// Convert CIE L\*a\*b\* back to linear sRGB via the D65 white point and CIE XYZ.
pub(crate) fn cie_lab_to_linear_rgb(lab: [f32; 3]) -> [f32; 3] {
    let [l, a, b] = lab;
    let fy = (l + 16.0) / 116.0;
    let fx = a / 500.0 + fy;
    let fz = fy - b / 200.0;
    let xyz = [XN * f_inv(fx), YN * f_inv(fy), ZN * f_inv(fz)];
    xyz_to_linear_rgb(xyz)
}

impl ColourSpace for CieLab {
    fn name(&self) -> &'static str {
        "CIE Lab"
    }

    fn axes(&self) -> [Axis; 3] {
        [
            Axis {
                name: "L*",
                range: (0.0, 100.0),
                circular: false,
            },
            Axis {
                name: "a*",
                range: (-128.0, 128.0),
                circular: false,
            },
            Axis {
                name: "b*",
                range: (-128.0, 128.0),
                circular: false,
            },
        ]
    }

    fn shape(&self) -> SpaceShape {
        SpaceShape::GamutClipped
    }

    fn to_linear_rgb(&self, coord: [f32; 3]) -> Option<[f32; 3]> {
        let linear = cie_lab_to_linear_rgb(coord);
        linear
            .iter()
            .all(|&c| (-1e-4..=1.0 + 1e-4).contains(&c))
            .then_some(linear)
    }

    fn from_linear_rgb(&self, linear_rgb: [f32; 3]) -> [f32; 3] {
        linear_rgb_to_cie_lab(linear_rgb)
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
                linear_rgb_to_cie_lab(linear)
            })
            .or_else(|| parse::css_function(input, "lab", &self.axes()))
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
        let coord = CieLab.from_linear_rgb([0.0, 0.0, 0.0]);
        assert_abs_diff_eq!(coord[0], 0.0, epsilon = 1e-3);
    }

    #[test]
    fn white_is_l100() {
        let coord = CieLab.from_linear_rgb([1.0, 1.0, 1.0]);
        assert_abs_diff_eq!(coord[0], 100.0, epsilon = 1e-2);
        assert_abs_diff_eq!(coord[1], 0.0, epsilon = 1e-2);
        assert_abs_diff_eq!(coord[2], 0.0, epsilon = 1e-2);
    }

    #[test]
    fn round_trip() {
        let linear = [0.6274_f32, 0.0399, 0.0];
        let coord = CieLab.from_linear_rgb(linear);
        let back = CieLab.to_linear_rgb(coord).unwrap();
        assert_abs_diff_eq!(back[0], linear[0], epsilon = 1e-4);
        assert_abs_diff_eq!(back[1], linear[1], epsilon = 1e-4);
        assert_abs_diff_eq!(back[2], linear[2], epsilon = 1e-4);
    }
}
