//! Colour-space implementations used by the visualiser.
//!
//! Each colour space is a zero-sized marker type that implements [`ColourSpace`].
//! All spaces are listed in [`SPACES`].

pub mod cie_lab;
pub mod cie_lch;
pub mod hsl;
pub mod hsv;
pub mod linear_srgb;
pub mod oklab;
pub mod oklch;
pub(crate) mod parse;
pub mod srgb;
pub(crate) mod xyz;

pub use cie_lab::CieLab;
pub use cie_lch::CieLch;
pub use hsl::Hsl;
pub use hsv::Hsv;
pub use linear_srgb::LinearSRgb;
pub use oklab::OkLab;
pub use oklch::OkLch;
pub use srgb::SRgb;

/// A colour space that supports converting to and from linear sRGB.
pub trait ColourSpace: Send + Sync {
    /// Returns the name of this colour space.
    fn name(&self) -> &'static str;

    /// Returns the three axes of this colour space.
    fn axes(&self) -> [Axis; 3];

    /// Returns the shape classification of this colour space.
    fn shape(&self) -> SpaceShape;

    /// Convert coordinates in this space to linear sRGB.
    /// Returns None if the coordinate is outside the sRGB gamut.
    fn to_linear_rgb(&self, coord: [f32; 3]) -> Option<[f32; 3]>;

    /// Convert linear sRGB to coordinates in this space.
    #[expect(
        clippy::wrong_self_convention,
        reason = "`from_*` takes `&self` here to dispatch on the space type, not to convert Self"
    )]
    fn from_linear_rgb(&self, linear_rgb: [f32; 3]) -> [f32; 3];

    /// Parse a CSS colour string into coordinates in this space.
    /// Accepts hex (#rrggbb), CSS functional notation, or bare values.
    fn parse_css(&self, input: &str) -> Option<[f32; 3]>;
}

/// Describes a single axis of a colour space.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Axis {
    /// Human-readable name of the axis (e.g., "Hue", "Saturation").
    pub name: &'static str,
    /// The valid range for this axis as (min, max).
    pub range: (f32, f32),
    /// Whether this axis is circular (e.g., hue wraps 0–360°).
    pub circular: bool,
}

/// Describes the shape of a colour space's gamut.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpaceShape {
    /// Rectangular gamut (e.g., sRGB).
    Cube,
    /// Double-cone gamut (e.g., HSL, HSV).
    Bicone,
    /// Single-cone gamut (e.g., LCh).
    Cone,
    /// Gamut with clipping or irregular boundary (e.g., LAB).
    GamutClipped,
}

/// All registered colour spaces, in display order.
pub static SPACES: &[&dyn ColourSpace] = &[
    &SRgb,
    &LinearSRgb,
    &Hsl,
    &Hsv,
    &OkLab,
    &OkLch,
    &CieLab,
    &CieLch,
];

#[cfg(test)]
mod integration_tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    /// Every space must correctly round-trip a sample of in-gamut colours.
    #[test]
    fn all_spaces_round_trip_orange() {
        // Linear sRGB for #d43400
        let linear = [0.6274_f32, 0.0399, 0.0];
        for space in SPACES {
            let coord = space.from_linear_rgb(linear);
            let back = space
                .to_linear_rgb(coord)
                .unwrap_or_else(|| panic!("{} returned None for in-gamut orange", space.name()));
            assert_abs_diff_eq!(back[0], linear[0], epsilon = 1e-3);
            assert_abs_diff_eq!(back[1], linear[1], epsilon = 1e-3);
            assert_abs_diff_eq!(back[2], linear[2], epsilon = 1e-3);
        }
    }

    #[test]
    fn all_spaces_have_three_axes() {
        for space in SPACES {
            assert_eq!(space.axes().len(), 3, "{} must have 3 axes", space.name());
        }
    }

    #[test]
    fn gamut_clipped_spaces_reject_out_of_gamut() {
        // Extremely saturated green that's outside sRGB in perceptual spaces
        let out_of_gamut_linear = [0.0_f32, 2.0, 0.0];
        for space in SPACES {
            if space.shape() == SpaceShape::GamutClipped {
                let coord = space.from_linear_rgb(out_of_gamut_linear);
                let result = space.to_linear_rgb(coord);
                assert!(
                    result.is_none(),
                    "{} should return None for out-of-gamut input, got {:?}",
                    space.name(),
                    result
                );
            }
        }
    }

    #[test]
    fn parse_css_hex_works_for_all_spaces() {
        for space in SPACES {
            let result = space.parse_css("#d43400");
            assert!(
                result.is_some(),
                "{} parse_css failed for hex #d43400",
                space.name()
            );
        }
    }

    #[test]
    fn spaces_list_has_eight_entries() {
        assert_eq!(SPACES.len(), 8);
    }
}
