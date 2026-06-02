//! CIE XYZ intermediate colour space (D65 illuminant).
//!
//! Used as a stepping stone when converting between sRGB and perceptual spaces.

/// Convert linear sRGB to CIE XYZ colour space (D65 illuminant).
///
/// Uses IEC 61966-2-1 transformation matrices.
///
/// # Arguments
///
/// * `rgb` - Linear sRGB values as [r, g, b] in range [0, 1].
///
/// # Returns
///
/// CIE XYZ coordinates where Y is luminance.
pub fn linear_rgb_to_xyz(rgb: [f32; 3]) -> [f32; 3] {
    let [r, g, b] = rgb;
    [
        r * 0.4124564 + g * 0.3575761 + b * 0.1804375,
        r * 0.2126729 + g * 0.7151522 + b * 0.0721750,
        r * 0.0193339 + g * 0.1191920 + b * 0.9503041,
    ]
}

/// Convert CIE XYZ colour space (D65 illuminant) to linear sRGB.
///
/// Uses IEC 61966-2-1 transformation matrices.
///
/// # Arguments
///
/// * `xyz` - CIE XYZ coordinates where Y is luminance.
///
/// # Returns
///
/// Linear sRGB values as [r, g, b] in range [0, 1].
pub fn xyz_to_linear_rgb(xyz: [f32; 3]) -> [f32; 3] {
    let [x, y, z] = xyz;
    [
        x * 3.2404542 + y * (-1.5371385) + z * (-0.4985314),
        x * (-0.9692660) + y * 1.8760108 + z * 0.0415560,
        x * 0.0556434 + y * (-0.2040259) + z * 1.0572252,
    ]
}
