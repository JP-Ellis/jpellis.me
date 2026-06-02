//! HSL (hue–saturation–lightness) colour space derived from sRGB.

use crate::colour::Axis;
use crate::colour::ColourSpace;
use crate::colour::SpaceShape;
use crate::colour::parse;
use crate::colour::srgb::linear_to_srgb_channel;
use crate::colour::srgb::srgb_channel_to_linear;

/// The HSL (hue–saturation–lightness) colour space derived from sRGB.
#[non_exhaustive]
pub struct Hsl;

/// Convert sRGB to HSL; hue is normalised to 0–1.
#[expect(
    clippy::float_cmp,
    reason = "exact equality on min/max is intentional for hue sector selection"
)]
fn srgb_to_hsl(rgb: [f32; 3]) -> [f32; 3] {
    let [r, g, b] = rgb;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let l = (max + min) / 2.0;

    let s = if delta < 1e-6 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    let h = if delta < 1e-6 {
        0.0
    } else if max == r {
        ((g - b) / delta).rem_euclid(6.0) / 6.0
    } else if max == g {
        ((b - r) / delta + 2.0) / 6.0
    } else {
        ((r - g) / delta + 4.0) / 6.0
    };

    [h, s, l]
}

/// Convert HSL back to sRGB; hue is expected to be normalised 0–1.
fn hsl_to_srgb(hsl: [f32; 3]) -> [f32; 3] {
    let [h, s, l] = hsl;
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
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
    let m = l - c / 2.0;
    [r1 + m, g1 + m, b1 + m]
}

impl ColourSpace for Hsl {
    fn name(&self) -> &'static str {
        "HSL"
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
                name: "L",
                range: (0.0, 1.0),
                circular: false,
            },
        ]
    }

    fn shape(&self) -> SpaceShape {
        SpaceShape::Bicone
    }

    fn to_linear_rgb(&self, coord: [f32; 3]) -> Option<[f32; 3]> {
        let srgb = hsl_to_srgb(coord);
        // HSL always maps to a valid sRGB value — clamp any floating point drift
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
        srgb_to_hsl(srgb)
    }

    fn parse_css(&self, input: &str) -> Option<[f32; 3]> {
        parse::hex(input)
            .map(srgb_to_hsl)
            .or_else(|| parse::css_function(input, "hsl", &self.axes()))
            .or_else(|| parse::bare_values(input, &self.axes()))
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::colour::ColourSpace;

    #[test]
    fn red_round_trips() {
        let space = Hsl;
        // Red in sRGB is (1,0,0); in HSL it's (0°, 100%, 50%) → normalised (0.0, 1.0, 0.5)
        let linear_red = [1.0_f32, 0.0, 0.0];
        let coord = space.from_linear_rgb(linear_red);
        assert_abs_diff_eq!(coord[0], 0.0, epsilon = 1e-4); // H = 0°
        assert_abs_diff_eq!(coord[1], 1.0, epsilon = 1e-4); // S = 1.0
        assert_abs_diff_eq!(coord[2], 0.5, epsilon = 1e-4); // L = 0.5
        let back = space.to_linear_rgb(coord).unwrap();
        assert_abs_diff_eq!(back[0], linear_red[0], epsilon = 1e-4);
    }

    #[test]
    fn white_round_trips() {
        let space = Hsl;
        let coord = space.from_linear_rgb([1.0, 1.0, 1.0]);
        assert_abs_diff_eq!(coord[2], 1.0, epsilon = 1e-4); // L = 1.0
        let back = space.to_linear_rgb(coord).unwrap();
        assert_abs_diff_eq!(back[0], 1.0, epsilon = 1e-4);
        assert_abs_diff_eq!(back[1], 1.0, epsilon = 1e-4);
        assert_abs_diff_eq!(back[2], 1.0, epsilon = 1e-4);
    }
}
