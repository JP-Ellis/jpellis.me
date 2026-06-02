//! Mesh tessellation for all colour space shapes.

use three_d::*;

use crate::colour::Axis;
use crate::colour::ColourSpace;
use crate::colour::SPACES;
use crate::colour::SpaceShape;

/// A single intermediate vertex with position and colour.
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    /// XYZ position in render space (each axis normalised to roughly 0–1).
    pub position: [f32; 3],
    /// Linear sRGB colour at this vertex.
    pub color: [f32; 3],
}

/// A triangle mesh in intermediate form.
pub struct Mesh {
    /// Interleaved vertex data.
    pub vertices: Vec<Vertex>,
    /// Triangle indices into `vertices`, three per triangle.
    pub indices: Vec<u32>,
}

/// Encode a linear sRGB component as a gamma-corrected `u8` (sRGB transfer function).
fn linear_to_srgb_u8(c: f32) -> u8 {
    let c = c.clamp(0.0, 1.0);
    let srgb = if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    };
    (srgb * 255.0).round() as u8
}

/// Build a [`CpuMesh`] for the colour space at `space_idx`.
///
/// Positions are centred at the origin (shifted by -0.5 on each axis) and
/// vertex colours are gamma-encoded. Normals are computed automatically.
pub fn build_cpu_mesh(space_idx: usize) -> CpuMesh {
    let space = SPACES[space_idx];
    let mesh = build_mesh(space);

    let positions: Vec<Vec3> = mesh
        .vertices
        .iter()
        .map(|v| {
            vec3(
                v.position[0] - 0.5,
                v.position[1] - 0.5,
                v.position[2] - 0.5,
            )
        })
        .collect();

    let colors: Vec<Srgba> = mesh
        .vertices
        .iter()
        .map(|v| {
            Srgba::new(
                linear_to_srgb_u8(v.color[0]),
                linear_to_srgb_u8(v.color[1]),
                linear_to_srgb_u8(v.color[2]),
                255,
            )
        })
        .collect();

    let mut cpu = CpuMesh {
        positions: Positions::F32(positions),
        indices: Indices::U32(mesh.indices),
        colors: Some(colors),
        ..Default::default()
    };
    cpu.compute_normals();
    cpu
}

/// Instanced spheres and connectors for a colour-space line overlay.
#[non_exhaustive]
pub struct LineInstances {
    /// One sphere per sample point. In-gamut: true colour, full size. Out-of-gamut:
    /// grey, reduced to 1/3 size when `fallback` is active.
    pub primary: Instances,
    /// Gamut-mapped coloured spheres for OOG samples (non-empty only when `fallback=true`).
    pub fallback_spheres: Instances,
    /// Cylinder transforms connecting each OOG sphere to its fallback position.
    pub connectors: Instances,
}

/// Build [`LineInstances`] for the colour-line overlay (spheres along the line from `a` to `b`).
///
/// # Arguments
///
/// * `space_idx` - Index into [`SPACES`] selecting the colour space.
/// * `a` - Start coordinate in the selected colour space.
/// * `b` - End coordinate in the selected colour space.
/// * `fallback` - When `true`, compute gamut-mapped fallback spheres, connectors,
///   and render OOG primary spheres at 1/3 size.
///
/// # Returns
///
/// A [`LineInstances`] with per-sample geometry for the line overlay.
#[must_use]
pub fn build_line_instances(
    space_idx: usize,
    a: [f32; 3],
    b: [f32; 3],
    fallback: bool,
) -> LineInstances {
    let space = SPACES[space_idx];
    let axes = space.axes();
    let n = 128usize;

    let mut primary_transforms = Vec::with_capacity(n);
    let mut primary_colors = Vec::with_capacity(n);
    let mut fallback_transforms = Vec::new();
    let mut fallback_colors = Vec::new();
    let mut connector_transforms = Vec::new();

    for i in 0..n {
        let t = i as f32 / (n - 1) as f32;
        let coord = [
            a[0] + t * (b[0] - a[0]),
            a[1] + t * (b[1] - a[1]),
            a[2] + t * (b[2] - a[2]),
        ];

        let pos = coord_to_render_pos(coord, &axes, space.shape());
        let center = vec3(pos[0] - 0.5, pos[1] - 0.5, pos[2] - 0.5);

        if let Some(rgb) = space.to_linear_rgb(coord) {
            primary_transforms.push(Mat4::from_translation(center) * Mat4::from_scale(0.012));
            primary_colors.push(Srgba::new(
                (rgb[0].clamp(0.0, 1.0) * 255.0).round() as u8,
                (rgb[1].clamp(0.0, 1.0) * 255.0).round() as u8,
                (rgb[2].clamp(0.0, 1.0) * 255.0).round() as u8,
                255,
            ));
        } else {
            let scale = if fallback { 0.004 } else { 0.012 };
            primary_transforms.push(Mat4::from_translation(center) * Mat4::from_scale(scale));
            primary_colors.push(Srgba::new(128, 128, 128, 255));

            if fallback {
                let raw_linear = oog_coord_to_linear_by_idx(coord, space_idx);
                if let Some(mapped) = map_to_gamut(raw_linear) {
                    let fb_coord = space.from_linear_rgb(mapped);
                    let fb_pos = coord_to_render_pos(fb_coord, &axes, space.shape());
                    let fb_center = vec3(fb_pos[0] - 0.5, fb_pos[1] - 0.5, fb_pos[2] - 0.5);

                    fallback_transforms
                        .push(Mat4::from_translation(fb_center) * Mat4::from_scale(0.012));
                    fallback_colors.push(Srgba::new(
                        (mapped[0].clamp(0.0, 1.0) * 255.0).round() as u8,
                        (mapped[1].clamp(0.0, 1.0) * 255.0).round() as u8,
                        (mapped[2].clamp(0.0, 1.0) * 255.0).round() as u8,
                        255,
                    ));
                    connector_transforms.push(cylinder_transform(center, fb_center));
                }
            }
        }
    }

    LineInstances {
        primary: Instances {
            transformations: primary_transforms,
            colors: Some(primary_colors),
            ..Default::default()
        },
        fallback_spheres: Instances {
            transformations: fallback_transforms,
            colors: fallback.then_some(fallback_colors),
            ..Default::default()
        },
        connectors: Instances {
            transformations: connector_transforms,
            colors: None,
            ..Default::default()
        },
    }
}

/// Convert an out-of-gamut coordinate to raw (unclamped) linear sRGB without a gamut check.
pub fn oog_coord_to_linear_by_idx(coord: [f32; 3], space_idx: usize) -> [f32; 3] {
    use crate::colour::cie_lab::cie_lab_to_linear_rgb;
    use crate::colour::oklab::oklab_to_linear_rgb;
    use crate::colour::oklch::oklch_to_oklab;
    use crate::colour::srgb::srgb_channel_to_linear;

    match space_idx {
        0 => [
            srgb_channel_to_linear(coord[0]),
            srgb_channel_to_linear(coord[1]),
            srgb_channel_to_linear(coord[2]),
        ],
        1 => coord,
        2 => SPACES[2].to_linear_rgb(coord).unwrap_or(coord),
        3 => SPACES[3].to_linear_rgb(coord).unwrap_or(coord),
        4 => oklab_to_linear_rgb(coord),
        5 => oklab_to_linear_rgb(oklch_to_oklab(coord)),
        6 => cie_lab_to_linear_rgb(coord),
        7 => {
            let [l, c, h] = coord;
            let h_rad = h * 360.0_f32.to_radians();
            let lab = [l, c * h_rad.cos(), c * h_rad.sin()];
            cie_lab_to_linear_rgb(lab)
        }
        _ => coord,
    }
}

/// Build a cylinder transform from `from` to `to` along the X-axis primitive.
///
/// [`CpuMesh::cylinder`] extends along the X-axis from (0,0,0) to (1,0,0). This
/// function rotates and scales it to run between the two given positions.
fn cylinder_transform(from: Vec3, to: Vec3) -> Mat4 {
    let diff = to - from;
    let length = (diff.x * diff.x + diff.y * diff.y + diff.z * diff.z).sqrt();
    if length < 1e-6 {
        return Mat4::from_scale(0.0);
    }
    let q = Quat::from_arc(vec3(1.0f32, 0.0, 0.0), diff / length, None);
    Mat4::from_translation(from)
        * Mat4::from(q)
        * Mat4::from_nonuniform_scale(length, 0.0015, 0.0015)
}

/// Map an out-of-gamut linear sRGB colour to the sRGB gamut via OKLCH chroma reduction.
///
/// Returns `None` if already in gamut, `Some([r,g,b])` with the mapped colour otherwise.
#[must_use]
pub fn map_to_gamut(linear_rgb: [f32; 3]) -> Option<[f32; 3]> {
    if linear_rgb.iter().all(|&c| (-0.001..=1.001).contains(&c)) {
        return None;
    }

    use crate::colour::ColourSpace;
    use crate::colour::OkLch;

    let [l, c, h] = OkLch.from_linear_rgb(linear_rgb);
    if l <= 0.0 {
        return Some([0.0, 0.0, 0.0]);
    }
    if l >= 1.0 {
        return Some([1.0, 1.0, 1.0]);
    }

    let mut lo = 0.0f32;
    let mut hi = c;
    for _ in 0..16 {
        let mid = (lo + hi) * 0.5;
        if OkLch.to_linear_rgb([l, mid, h]).is_some() {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    let mapped = OkLch.to_linear_rgb([l, lo, h]).map_or_else(
        || [l.clamp(0.0, 1.0); 3],
        |rgb| {
            [
                rgb[0].clamp(0.0, 1.0),
                rgb[1].clamp(0.0, 1.0),
                rgb[2].clamp(0.0, 1.0),
            ]
        },
    );
    Some(mapped)
}

/// Normalise a value `v` from range `(min, max)` to `[0, 1]`.
fn normalize(v: f32, range: (f32, f32)) -> f32 {
    (v - range.0) / (range.1 - range.0)
}

/// Convert colour-space coordinates to a render-space position in `[0, 1]^3`.
///
/// Cylindrical spaces (HSL, HSV, OkLCh, CIE LCh) are converted to Cartesian
/// so they render as the expected cone/bicone shapes.
///
/// For `Cone` (HSV) the horizontal radius scales as `S × V` so the shape
/// tapers to a black point at V=0.  For `Bicone` (HSL) it scales as
/// `S × (1 − |2L − 1|)` so both poles (L=0 black, L=1 white) taper to points.
#[must_use]
pub fn coord_to_render_pos(coord: [f32; 3], axes: &[Axis; 3], shape: SpaceShape) -> [f32; 3] {
    if axes[0].circular {
        // HSL / HSV: H=axis0 (°), S=axis1, L/V=axis2
        let h = normalize(coord[0], axes[0].range) * std::f32::consts::TAU;
        let s = normalize(coord[1], axes[1].range);
        let lv = normalize(coord[2], axes[2].range);
        let r = match shape {
            SpaceShape::Bicone => s * (1.0 - (2.0 * lv - 1.0).abs()),
            _ => s * lv,
        };
        [0.5 + 0.5 * r * h.cos(), lv, 0.5 + 0.5 * r * h.sin()]
    } else if axes[2].circular {
        // OkLCh / CIE LCh: L=axis0, C=axis1, h=axis2 (°)
        let l = normalize(coord[0], axes[0].range);
        let c = normalize(coord[1], axes[1].range);
        let h = normalize(coord[2], axes[2].range) * std::f32::consts::TAU;
        [0.5 + 0.5 * c * h.cos(), l, 0.5 + 0.5 * c * h.sin()]
    } else {
        // Cartesian (sRGB, OkLab, CIE LAB, …): normalise each axis to [0, 1].
        [
            normalize(coord[0], axes[0].range),
            normalize(coord[1], axes[1].range),
            normalize(coord[2], axes[2].range),
        ]
    }
}

/// Build a mesh for the given colour space.
pub fn build_mesh(space: &dyn ColourSpace) -> Mesh {
    parametric_gamut_mesh(space, 64)
}

// MARK: Parametric Gamut Mesh

/// Tessellate the sRGB unit cube surface and map each vertex into `space`.
///
/// The sRGB gamut boundary in any colour space is exactly the image of the
/// unit RGB cube's 6 faces under [`ColourSpace::from_linear_rgb`].  By
/// tessellating those faces directly we obtain an exact, smooth surface with
/// no staircase artefacts — the boundary is correct by construction rather
/// than approximated by isosurface extraction.
///
/// `n` is the number of subdivisions per face edge.  `n = 64` produces
/// 6 × 64² × 2 ≈ 49 k triangles, which is accurate and fast for WebGL.
fn parametric_gamut_mesh(space: &dyn ColourSpace, n: usize) -> Mesh {
    let axes = space.axes();
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // Each entry: (fixed_channel, fixed_value, [u_channel, v_channel], flip_winding).
    //
    // `flip_winding` corrects the triangle orientation so every face normal
    // points outward (away from the interior of the unit cube).  The rule
    // follows from the cross-product of the u- and v-axis unit vectors:
    //
    //   R=0: v×u = B×G = −R  (outward for R=0) → no flip
    //   R=1: same face normal = −R  (inward for R=1)  → flip
    //   G=0: v×u = B×R = +G  (inward for G=0)        → flip
    //   G=1: same face normal = +G  (outward for G=1) → no flip
    //   B=0: v×u = G×R = −B  (outward for B=0)       → no flip
    //   B=1: same face normal = −B  (inward for B=1)  → flip
    const FACES: [(usize, f32, [usize; 2], bool); 6] = [
        (0, 0.0, [1, 2], false), // R=0
        (0, 1.0, [1, 2], true),  // R=1
        (1, 0.0, [0, 2], true),  // G=0
        (1, 1.0, [0, 2], false), // G=1
        (2, 0.0, [0, 1], false), // B=0
        (2, 1.0, [0, 1], true),  // B=1
    ];

    for (fixed_ch, fixed_val, [u_ch, v_ch], flip) in FACES {
        let base = vertices.len() as u32;

        for vi in 0..=(n as u32) {
            for ui in 0..=(n as u32) {
                let u = ui as f32 / n as f32;
                let v = vi as f32 / n as f32;

                let mut rgb = [0.0f32; 3];
                rgb[fixed_ch] = fixed_val;
                rgb[u_ch] = u;
                rgb[v_ch] = v;

                let coord = space.from_linear_rgb(rgb);
                let position = coord_to_render_pos(coord, &axes, space.shape());
                vertices.push(Vertex {
                    position,
                    color: rgb,
                });
            }
        }

        let stride = (n + 1) as u32;
        for vi in 0..n as u32 {
            for ui in 0..n as u32 {
                let v00 = base + vi * stride + ui;
                let v10 = v00 + 1;
                let v01 = base + (vi + 1) * stride + ui;
                let v11 = v01 + 1;

                if flip {
                    indices.extend_from_slice(&[v00, v10, v01, v01, v10, v11]);
                } else {
                    indices.extend_from_slice(&[v00, v01, v10, v10, v01, v11]);
                }
            }
        }
    }

    Mesh { vertices, indices }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colour::Hsl;
    use crate::colour::Hsv;
    use crate::colour::OkLab;
    use crate::colour::OkLch;
    use crate::colour::SRgb;

    #[test]
    fn coord_to_render_pos_cartesian_normalises() {
        let axes = SRgb.axes();
        let pos = coord_to_render_pos([1.0, 0.0, 0.5], &axes, SRgb.shape());
        // sRGB axes are all [0,1], so normalization is identity
        assert!((pos[0] - 1.0).abs() < 1e-5);
        assert!((pos[1] - 0.0).abs() < 1e-5);
        assert!((pos[2] - 0.5).abs() < 1e-5);
    }

    #[test]
    fn coord_to_render_pos_hsl_zero_saturation_is_axis() {
        let axes = Hsl.axes();
        // Hue=0, S=0 → should be at (0.5, ?, 0.5) regardless of hue
        let pos = coord_to_render_pos([0.0, 0.0, 0.5], &axes, Hsl.shape());
        assert!(
            (pos[0] - 0.5).abs() < 1e-5,
            "x should be 0.5 at S=0, got {}",
            pos[0]
        );
        assert!(
            (pos[2] - 0.5).abs() < 1e-5,
            "z should be 0.5 at S=0, got {}",
            pos[2]
        );
    }

    #[test]
    fn coord_to_render_pos_oklch_centre_is_axis() {
        let axes = OkLch.axes();
        // C=0 → should be at (0.5, L_norm, 0.5)
        let pos = coord_to_render_pos([0.5, 0.0, 0.0], &axes, OkLch.shape());
        assert!(
            (pos[0] - 0.5).abs() < 1e-5,
            "x should be 0.5 at C=0, got {}",
            pos[0]
        );
        assert!(
            (pos[2] - 0.5).abs() < 1e-5,
            "z should be 0.5 at C=0, got {}",
            pos[2]
        );
    }

    #[test]
    fn build_mesh_srgb_produces_vertices() {
        let mesh = build_mesh(&SRgb);
        assert!(!mesh.vertices.is_empty(), "sRGB should produce vertices");
        assert!(
            mesh.vertices.len() > 100,
            "sRGB should have many vertices, got {}",
            mesh.vertices.len()
        );
    }

    #[test]
    fn build_mesh_hsl_produces_bicone() {
        let mesh = build_mesh(&Hsl);
        assert!(!mesh.vertices.is_empty(), "HSL should produce vertices");
        assert!(!mesh.indices.is_empty(), "HSL should have indices");
    }

    #[test]
    fn build_mesh_hsv_produces_cone() {
        let mesh = build_mesh(&Hsv);
        assert!(!mesh.vertices.is_empty(), "HSV should produce vertices");
    }

    #[test]
    fn build_mesh_oklab_produces_vertices() {
        let mesh = build_mesh(&OkLab);
        assert!(!mesh.vertices.is_empty(), "OkLab should produce vertices");
    }

    #[test]
    fn build_line_instances_in_gamut_line_has_128_spheres() {
        let instances = build_line_instances(0, [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], false);
        assert_eq!(instances.primary.transformations.len(), 128);
        assert_eq!(instances.fallback_spheres.transformations.len(), 0);
        assert_eq!(instances.connectors.transformations.len(), 0);
    }

    #[test]
    fn build_line_instances_oog_spheres_smaller_when_fallback() {
        let with_fb = build_line_instances(0, [1.5, 0.5, 0.5], [0.5, 0.5, 0.5], true);
        let without_fb = build_line_instances(0, [1.5, 0.5, 0.5], [0.5, 0.5, 0.5], false);
        assert_ne!(
            with_fb.primary.transformations[0], without_fb.primary.transformations[0],
            "OOG sphere should be smaller with fallback active"
        );
    }

    #[test]
    fn build_line_instances_fallback_true_oog_line_has_connectors() {
        let instances = build_line_instances(0, [1.5, 0.5, 0.5], [0.0, 0.5, 0.5], true);
        assert!(instances.fallback_spheres.transformations.len() > 0);
        assert_eq!(
            instances.fallback_spheres.transformations.len(),
            instances.connectors.transformations.len(),
        );
    }

    #[test]
    fn map_to_gamut_in_gamut_returns_none() {
        assert!(map_to_gamut([1.0, 0.0, 0.0]).is_none());
    }

    #[test]
    fn map_to_gamut_out_of_gamut_returns_some_in_gamut() {
        let mapped = map_to_gamut([1.5, 0.2, -0.1]).expect("expected mapped result");
        for (i, &c) in mapped.iter().enumerate() {
            assert!(
                (-0.001..=1.001).contains(&c),
                "channel {i} = {c} out of gamut"
            );
        }
    }
}
