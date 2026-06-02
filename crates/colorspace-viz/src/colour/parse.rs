//! CSS colour string parsing utilities shared by all colour spaces.

// MARK: CSS colour string parsing

use crate::colour::Axis;

/// Parse a CSS hex colour (#rrggbb or #rgb) into sRGB [f32; 3] (0–1).
///
/// # Arguments
///
/// * `input` - A hex colour string like "#ff8000" or "#f80"
///
/// # Returns
///
/// An array of three floats (0.0–1.0) representing sRGB, or None if parsing fails.
///
/// # Example
///
/// ```ignore
/// let result = hex("#ff8000").unwrap();
/// assert!((result[0] - 1.0).abs() < 1e-3);
/// assert!((result[1] - 0.502).abs() < 1e-3);
/// assert!((result[2] - 0.0).abs() < 1e-3);
/// ```
pub(crate) fn hex(input: &str) -> Option<[f32; 3]> {
    let s = input.trim().strip_prefix('#')?;
    match s.len() {
        6 => {
            let r = u8::from_str_radix(s.get(0..2)?, 16).ok()?;
            let g = u8::from_str_radix(s.get(2..4)?, 16).ok()?;
            let b = u8::from_str_radix(s.get(4..6)?, 16).ok()?;
            Some([
                f32::from(r) / 255.0,
                f32::from(g) / 255.0,
                f32::from(b) / 255.0,
            ])
        }
        3 => {
            let r = u8::from_str_radix(s.get(0..1)?, 16).ok()?;
            let g = u8::from_str_radix(s.get(1..2)?, 16).ok()?;
            let b = u8::from_str_radix(s.get(2..3)?, 16).ok()?;
            // Single hex digit expanded to full byte: 0xF → 0xFF (multiply by 17 = 0x11).
            // r ≤ 15, so r * 17 ≤ 255 — no overflow.
            Some([
                f32::from(r) * (17.0 / 255.0),
                f32::from(g) * (17.0 / 255.0),
                f32::from(b) * (17.0 / 255.0),
            ])
        }
        _ => None,
    }
}

/// Parse a CSS functional notation string like `oklch(0.55 0.15 30)` or `hsl(120, 80%, 50%)`.
///
/// Hue axes (circular=true) treat the value as degrees (0–360) and normalise to 0–1.
/// Percentage-suffixed values have the `%` stripped and are divided by 100.
///
/// # Arguments
///
/// * `input` - The functional notation string (e.g., "oklch(0.55 0.15 30)")
/// * `name` - The expected function name (e.g., "oklch", "hsl")
/// * `axes` - The three axes describing the colour space
///
/// # Returns
///
/// An array of three normalised floats, or None if parsing fails.
///
/// # Example
///
/// ```ignore
/// let axes = [
///     Axis { name: "L", range: (0.0, 1.0), circular: false },
///     Axis { name: "C", range: (0.0, 0.4), circular: false },
///     Axis { name: "h", range: (0.0, 1.0), circular: true },
/// ];
/// let result = css_function("oklch(0.55 0.15 30)", "oklch", &axes).unwrap();
/// assert!((result[2] - 30.0/360.0).abs() < 1e-4);
/// ```
pub(crate) fn css_function(input: &str, name: &str, axes: &[Axis; 3]) -> Option<[f32; 3]> {
    let input = input.trim();
    let body = input
        .strip_prefix(name)?
        .trim()
        .strip_prefix('(')?
        .strip_suffix(')')?
        .trim();

    let raw: Vec<&str> = if body.contains(',') {
        body.split(',').map(str::trim).collect()
    } else {
        body.split_whitespace().collect()
    };
    if raw.len() != 3 {
        return None;
    }

    let mut values = [0.0_f32; 3];
    for (i, (token, axis)) in raw.iter().zip(axes.iter()).enumerate() {
        let token = token.trim();
        let (value_str, is_percent) = if let Some(p) = token.strip_suffix('%') {
            (p, true)
        } else {
            (token, false)
        };
        let mut v: f32 = value_str.parse().ok()?;
        if is_percent {
            v /= 100.0;
        }
        // Hue axes: degrees input, normalise to 0–1
        if axis.circular {
            v = v.rem_euclid(360.0) / 360.0;
        }
        values[i] = v;
    }
    Some(values)
}

/// Parse bare space- or comma-separated float values.
///
/// Values must be within the axis range (±10% tolerance for float drift).
///
/// # Arguments
///
/// * `input` - Space- or comma-separated numbers (e.g., "0.5 0.2 0.8")
/// * `axes` - The three axes describing the colour space
///
/// # Returns
///
/// An array of three floats, or None if parsing fails or values are out of range.
///
/// # Example
///
/// ```ignore
/// let axes = [
///     Axis { name: "a", range: (0.0, 1.0), circular: false },
///     Axis { name: "b", range: (0.0, 1.0), circular: false },
///     Axis { name: "c", range: (0.0, 1.0), circular: false },
/// ];
/// let result = bare_values("0.5 0.2 0.8", &axes).unwrap();
/// assert_eq!(result[0], 0.5);
/// ```
pub(crate) fn bare_values(input: &str, axes: &[Axis; 3]) -> Option<[f32; 3]> {
    let tokens: Vec<&str> = if input.contains(',') {
        input.split(',').map(str::trim).collect()
    } else {
        input.split_whitespace().collect()
    };
    if tokens.len() != 3 {
        return None;
    }
    let mut values = [0.0_f32; 3];
    for (i, (token, axis)) in tokens.iter().zip(axes.iter()).enumerate() {
        let v: f32 = token.parse().ok()?;
        let slack = (axis.range.1 - axis.range.0) * 0.1;
        if v < axis.range.0 - slack || v > axis.range.1 + slack {
            return None;
        }
        values[i] = v;
    }
    Some(values)
}

// MARK: Generic CSS parsing

/// Parse any common CSS colour string into linear sRGB `[0,1]³`.
///
/// Tries, in order: `#rrggbb` hex, `rgb(…)`, and `hsl(…)`.
/// Returns `None` if the input cannot be recognised.
pub(crate) fn parse_any_css_to_linear_rgb(input: &str) -> Option<[f32; 3]> {
    use crate::colour::srgb::srgb_channel_to_linear;
    let gamma_to_linear = |g: [f32; 3]| {
        [
            srgb_channel_to_linear(g[0]),
            srgb_channel_to_linear(g[1]),
            srgb_channel_to_linear(g[2]),
        ]
    };

    if let Some(srgb) = hex(input) {
        return Some(gamma_to_linear(srgb));
    }
    if let Some(srgb) = parse_rgb_css_to_gamma(input) {
        return Some(gamma_to_linear(srgb));
    }
    if let Some(srgb) = parse_hsl_css_to_gamma(input) {
        return Some(gamma_to_linear(srgb));
    }
    None
}

/// Parse any CSS colour into coordinates in `space`.
///
/// First tries the space's own `parse_css`, then falls back to interpreting the
/// input through linear sRGB via [`parse_any_css_to_linear_rgb`].
pub fn parse_any_css_to_space(
    input: &str,
    space: &dyn crate::colour::ColourSpace,
) -> Option<[f32; 3]> {
    if let Some(coord) = space.parse_css(input) {
        return Some(coord);
    }
    parse_any_css_to_linear_rgb(input).map(|lin| space.from_linear_rgb(lin))
}

/// Format a colour coordinate as a CSS string in the given space's notation.
///
/// Produces human-readable strings like `#d43400`, `hsl(12, 90%, 42%)`,
/// or `oklch(0.550 0.180 25.0)` depending on the space.
pub fn format_coord_css(coord: [f32; 3], space: &dyn crate::colour::ColourSpace) -> String {
    let axes = space.axes();
    match space.name() {
        "sRGB" => {
            let r = (coord[0].clamp(0.0, 1.0) * 255.0).round() as u8;
            let g = (coord[1].clamp(0.0, 1.0) * 255.0).round() as u8;
            let b = (coord[2].clamp(0.0, 1.0) * 255.0).round() as u8;
            format!("#{r:02x}{g:02x}{b:02x}")
        }
        "HSL" => {
            let h = (coord[0] * 360.0).round() as u32;
            let s = (coord[1] * 100.0).round() as u32;
            let l = (coord[2] * 100.0).round() as u32;
            format!("hsl({h}, {s}%, {l}%)")
        }
        "HSV" => {
            let h = (coord[0] * 360.0).round() as u32;
            let s = (coord[1] * 100.0).round() as u32;
            let v = (coord[2] * 100.0).round() as u32;
            format!("hsv({h}, {s}%, {v}%)")
        }
        "OkLab" => format!("oklab({:.3} {:.3} {:.3})", coord[0], coord[1], coord[2]),
        "OkLCh" => {
            let h = coord[2] * 360.0;
            format!("oklch({:.3} {:.3} {:.1})", coord[0], coord[1], h)
        }
        "CIE Lab" => format!("lab({:.1} {:.1} {:.1})", coord[0], coord[1], coord[2]),
        "CIE LCh" => {
            let h = coord[2] * 360.0;
            format!("lch({:.1} {:.1} {:.1})", coord[0], coord[1], h)
        }
        "Linear sRGB" => format!("{:.4} {:.4} {:.4}", coord[0], coord[1], coord[2]),
        _ => {
            let fmt = |v: f32, axis: &crate::colour::Axis| {
                if axis.circular {
                    format!("{:.1}", v * 360.0)
                } else {
                    format!("{:.3}", v)
                }
            };
            format!(
                "{}({} {} {})",
                space.name(),
                fmt(coord[0], &axes[0]),
                fmt(coord[1], &axes[1]),
                fmt(coord[2], &axes[2]),
            )
        }
    }
}

// MARK: CSS sub-parsers

/// Parse `rgb(R G B)` / `rgb(R, G, B)` where each component is 0–255 or 0–100%.
/// Returns sRGB gamma `[0,1]`, or `None` on failure.
fn parse_rgb_css_to_gamma(input: &str) -> Option<[f32; 3]> {
    let input = input.trim();
    let body = input
        .strip_prefix("rgb(")
        .or_else(|| input.strip_prefix("rgba("))?
        .strip_suffix(')')?;

    let tokens: Vec<&str> = if body.contains(',') {
        body.split(',').map(str::trim).collect()
    } else {
        body.split_whitespace().collect()
    };
    // Accept 3 or 4 tokens (ignore alpha).
    if tokens.len() < 3 {
        return None;
    }
    let mut values = [0.0_f32; 3];
    for (i, token) in tokens[..3].iter().enumerate() {
        let token = token.trim();
        if let Some(p) = token.strip_suffix('%') {
            values[i] = p.parse::<f32>().ok()? / 100.0;
        } else {
            values[i] = token.parse::<f32>().ok()? / 255.0;
        }
    }
    Some(values)
}

/// Parse `hsl(H S% L%)` / `hsl(H, S%, L%)` and convert to sRGB gamma `[0,1]`.
fn parse_hsl_css_to_gamma(input: &str) -> Option<[f32; 3]> {
    let input = input.trim();
    let body = input
        .strip_prefix("hsl(")
        .or_else(|| input.strip_prefix("hsla("))?
        .strip_suffix(')')?;

    let tokens: Vec<&str> = if body.contains(',') {
        body.split(',').map(str::trim).collect()
    } else {
        body.split_whitespace().collect()
    };
    if tokens.len() < 3 {
        return None;
    }
    let h_str = tokens[0].trim().trim_end_matches("deg");
    let h = h_str.parse::<f32>().ok()?.rem_euclid(360.0) / 360.0;
    let s_str = tokens[1].trim().trim_end_matches('%');
    let s = s_str.parse::<f32>().ok()? / 100.0;
    let l_str = tokens[2].trim().trim_end_matches('%');
    let l = l_str.parse::<f32>().ok()? / 100.0;
    Some(hsl_to_srgb_gamma([h, s, l]))
}

/// Convert normalised HSL `[0–1, 0–1, 0–1]` to sRGB gamma `[0–1]`.
fn hsl_to_srgb_gamma(hsl: [f32; 3]) -> [f32; 3] {
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

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    fn axis(range: (f32, f32)) -> crate::colour::Axis {
        crate::colour::Axis {
            name: "",
            range,
            circular: false,
        }
    }

    fn circular_axis(range: (f32, f32)) -> crate::colour::Axis {
        crate::colour::Axis {
            name: "",
            range,
            circular: true,
        }
    }

    #[test]
    fn hex_6digit() {
        let result = hex("#ff8000").unwrap();
        assert_abs_diff_eq!(result[0], 1.0, epsilon = 1e-3);
        assert_abs_diff_eq!(result[1], 0.502, epsilon = 1e-3);
        assert_abs_diff_eq!(result[2], 0.0, epsilon = 1e-3);
    }

    #[test]
    fn hex_3digit() {
        let result = hex("#f80").unwrap();
        assert_abs_diff_eq!(result[0], 1.0, epsilon = 1e-3);
        assert_abs_diff_eq!(result[1], 0.533, epsilon = 1e-3);
        assert_abs_diff_eq!(result[2], 0.0, epsilon = 1e-3);
    }

    #[test]
    fn hex_without_hash() {
        assert!(hex("ff8000").is_none()); // must start with #
    }

    #[test]
    fn css_function_oklch() {
        let axes = [
            axis((0.0, 1.0)),          // L - not circular
            axis((0.0, 0.4)),          // C - not circular
            circular_axis((0.0, 1.0)), // h - circular
        ];
        let result = css_function("oklch(0.55 0.15 30)", "oklch", &axes).unwrap();
        assert_abs_diff_eq!(result[0], 0.55, epsilon = 1e-4);
        assert_abs_diff_eq!(result[1], 0.15, epsilon = 1e-4);
        // h=30 degrees → normalised to 30/360
        assert_abs_diff_eq!(result[2], 30.0 / 360.0, epsilon = 1e-4);
    }

    #[test]
    fn css_function_hsl_with_percent() {
        let axes = [
            circular_axis((0.0, 1.0)), // H - circular
            axis((0.0, 1.0)),          // S - not circular
            axis((0.0, 1.0)),          // L - not circular
        ];
        let result = css_function("hsl(120 80% 50%)", "hsl", &axes).unwrap();
        assert_abs_diff_eq!(result[0], 120.0 / 360.0, epsilon = 1e-4); // H normalised
        assert_abs_diff_eq!(result[1], 0.8, epsilon = 1e-4);
        assert_abs_diff_eq!(result[2], 0.5, epsilon = 1e-4);
    }

    #[test]
    fn bare_values_space_separated() {
        let axes = [axis((0.0, 1.0)), axis((0.0, 1.0)), axis((0.0, 1.0))];
        let result = bare_values("0.5 0.2 0.8", &axes).unwrap();
        assert_abs_diff_eq!(result[0], 0.5, epsilon = 1e-4);
        assert_abs_diff_eq!(result[1], 0.2, epsilon = 1e-4);
        assert_abs_diff_eq!(result[2], 0.8, epsilon = 1e-4);
    }

    #[test]
    fn bare_values_comma_separated() {
        let axes = [
            axis((0.0, 100.0)),
            axis((-128.0, 128.0)),
            axis((-128.0, 128.0)),
        ];
        let result = bare_values("55, -20, 30", &axes).unwrap();
        assert_abs_diff_eq!(result[0], 55.0, epsilon = 1e-4);
        assert_abs_diff_eq!(result[1], -20.0, epsilon = 1e-4);
        assert_abs_diff_eq!(result[2], 30.0, epsilon = 1e-4);
    }
}
