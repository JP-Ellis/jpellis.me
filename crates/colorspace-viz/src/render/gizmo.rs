//! Corner reference gizmo — shows the orientation and axis labels of the active colour space.

use std::f32::consts::FRAC_PI_2;
use std::f32::consts::TAU;

use three_d::*;

use crate::colour::SpaceShape;

// MARK: Public API

/// Corner reference axes gizmo for the 3-D render.
///
/// Renders as a small overlay in the bottom-left corner:
/// - **Cartesian** spaces (sRGB, LAB, …): coloured X/Y/Z arrows.
/// - **Cylindrical** spaces (HSL, HSV, LCh, …): a white vertical arrow for the
///   lightness/value axis plus a rainbow arc in the horizontal plane showing the
///   direction of increasing hue.
pub(crate) enum Gizmo {
    Cartesian {
        axes: Axes,
    },
    Cylindrical {
        vert: Gm<Mesh, ColorMaterial>,
        hue: Gm<Mesh, ColorMaterial>,
    },
}

impl Gizmo {
    pub(crate) fn new(context: &Context, shape: SpaceShape) -> Self {
        match shape {
            SpaceShape::Cube | SpaceShape::GamutClipped => {
                let mut axes = Axes::new(context, 0.025, 0.4);
                axes.material.render_states.depth_test = DepthTest::Always;
                Self::Cartesian { axes }
            }
            SpaceShape::Bicone | SpaceShape::Cone => Self::Cylindrical {
                vert: build_vert_arrow(context),
                hue: build_hue_arc(context),
            },
        }
    }
}

// MARK: Rendering helper (used by render/mod.rs)

/// Render `gizmo` into the bottom-left corner of `screen`, oriented to match `camera`.
///
/// This clears only the gizmo's scissor-box depth region so the gizmo always
/// appears on top of the main scene.
pub(crate) fn render_corner(
    gizmo: &Gizmo,
    screen: &RenderTarget<'_>,
    camera: &Camera,
    height: u32,
) {
    const SIZE: u32 = 80;
    const MARGIN: u32 = 10;

    // Bottom-left corner; in WebGL, y=0 is the bottom of the framebuffer.
    let _ = height; // kept for potential future use (e.g., top-left placement)
    let scissor = ScissorBox {
        x: MARGIN as i32,
        y: MARGIN as i32,
        width: SIZE,
        height: SIZE,
    };

    // The gizmo camera has the same orientation as the main camera but is always
    // at a fixed distance from the origin.  The viewport must match the scissor
    // box so that NDC (0,0) maps to the centre of that corner region.
    let dir = camera.position().normalize();
    let gizmo_cam = Camera::new_perspective(
        Viewport {
            x: scissor.x,
            y: scissor.y,
            width: scissor.width,
            height: scissor.height,
        },
        dir * 2.5,
        vec3(0.0, 0.0, 0.0),
        camera.up(),
        degrees(30.0),
        0.1,
        100.0,
    );

    // Clear depth so the gizmo is never occluded by the gamut mesh.
    screen.clear_partially(scissor, ClearState::depth(1.0));

    let no_lights: &[&dyn Light] = &[];

    match gizmo {
        Gizmo::Cartesian { axes } => {
            screen.render_partially(scissor, &gizmo_cam, [axes as &dyn Object], no_lights);
        }
        Gizmo::Cylindrical { vert, hue } => {
            screen.render_partially(
                scissor,
                &gizmo_cam,
                [vert as &dyn Object, hue as &dyn Object],
                no_lights,
            );
        }
    }
}

// MARK: Cylindrical gizmo geometry

/// Vertical Y-axis arrow (white) for cylindrical colour spaces.
fn build_vert_arrow(context: &Context) -> Gm<Mesh, ColorMaterial> {
    let mut cpu = CpuMesh::arrow(0.9, 0.5, 12);
    // Scale, rotate X→Y (90° around Z), then translate to centre on origin.
    cpu.transform(
        Mat4::from_translation(vec3(0.0, -0.3, 0.0))
            * Mat4::from_angle_z(radians(FRAC_PI_2))
            * Mat4::from_nonuniform_scale(0.6, 0.04, 0.04),
    )
    .unwrap();
    Gm::new(
        Mesh::new(context, &cpu),
        ColorMaterial {
            color: Srgba::new(210, 210, 210, 255),
            render_states: RenderStates {
                depth_test: DepthTest::Always,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}

/// Rainbow-coloured hue arc (XZ plane) with arrowhead, for cylindrical colour spaces.
///
/// The arc starts at +X (hue = 0) and sweeps 300° counterclockwise, ending with a
/// cone arrowhead that shows the direction of increasing hue.
fn build_hue_arc(context: &Context) -> Gm<Mesh, ColorMaterial> {
    const ARC_R: f32 = 0.28;
    const TUBE_R: f32 = 0.022;
    const ARC_STEPS: u32 = 36;
    const TUBE_RING: u32 = 8;
    // 300° arc, leaving a 60° gap so the open end makes the direction obvious.
    let arc_extent = 300.0_f32.to_radians();

    let tube_cpu = build_arc_tube(ARC_R, TUBE_R, ARC_STEPS, TUBE_RING, arc_extent);
    let head_cpu = build_arc_arrowhead(ARC_R, arc_extent, 0.06, 0.05);

    let merged = merge_cpu_meshes(tube_cpu, head_cpu);
    Gm::new(
        Mesh::new(context, &merged),
        ColorMaterial {
            render_states: RenderStates {
                depth_test: DepthTest::Always,
                ..Default::default()
            },
            ..Default::default()
        },
    )
}

// MARK: Geometry helpers

/// Build a tube mesh following a circular arc in the XZ plane at Y = 0.
///
/// The arc centre is the world origin. `arc_extent` is the sweep in radians.
fn build_arc_tube(
    arc_r: f32,
    tube_r: f32,
    arc_steps: u32,
    tube_ring: u32,
    arc_extent: f32,
) -> CpuMesh {
    let n_rings = (arc_steps + 1) as usize;
    let ring_verts = tube_ring as usize;

    let mut positions = Vec::with_capacity(n_rings * ring_verts);
    let mut indices: Vec<u32> = Vec::new();
    let mut colors = Vec::with_capacity(n_rings * ring_verts);

    for i in 0..=arc_steps {
        let t = i as f32 / arc_steps as f32;
        let theta = t * arc_extent;

        // Spine position in XZ plane.
        let cx = arc_r * theta.cos();
        let cz = arc_r * theta.sin();

        // Outward-radial direction (from arc centre through spine point).
        let rx = theta.cos();
        let rz = theta.sin();

        let color = hue_to_srgba(theta / TAU);

        for j in 0..tube_ring {
            let phi = j as f32 / tube_ring as f32 * TAU;
            let nr = phi.cos(); // weight along radial direction
            let ny = phi.sin(); // weight along Y

            positions.push(vec3(
                cx + tube_r * nr * rx,
                tube_r * ny,
                cz + tube_r * nr * rz,
            ));
            colors.push(color);
        }
    }

    for i in 0..arc_steps {
        for j in 0..tube_ring {
            let a = i * tube_ring + j;
            let b = i * tube_ring + (j + 1) % tube_ring;
            let c = (i + 1) * tube_ring + j;
            let d = (i + 1) * tube_ring + (j + 1) % tube_ring;
            // Two CCW triangles per quad (both faces drawn since Cull::None is default).
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }

    let mut cpu = CpuMesh {
        positions: Positions::F32(positions),
        indices: Indices::U32(indices),
        colors: Some(colors),
        ..Default::default()
    };
    cpu.compute_normals();
    cpu
}

/// Build a cone arrowhead at the end of the hue arc, pointing tangentially.
fn build_arc_arrowhead(arc_r: f32, arc_extent: f32, cone_length: f32, cone_radius: f32) -> CpuMesh {
    // End-of-arc position in XZ plane.
    let end_x = arc_r * arc_extent.cos();
    let end_z = arc_r * arc_extent.sin();

    // Tangent direction at arc end (CCW, so derivative of (cos θ, sin θ) w.r.t. θ).
    // The cone's default axis is +X; rotate around Y to align with the tangent.
    // Rotation angle satisfies: Ry(a) * X_hat = tangent = (-sin θ, 0, cos θ).
    let rot_a = f32::atan2(-arc_extent.cos(), -arc_extent.sin());

    let mut cone = CpuMesh::cone(12);
    cone.transform(
        Mat4::from_translation(vec3(end_x, 0.0, end_z))
            * Mat4::from_angle_y(radians(rot_a))
            * Mat4::from_nonuniform_scale(cone_length, cone_radius, cone_radius),
    )
    .unwrap();

    // Colour the arrowhead with the hue at the end of the arc.
    let n = match &cone.positions {
        Positions::F32(v) => v.len(),
        _ => 0,
    };
    cone.colors = Some(vec![hue_to_srgba(arc_extent / TAU); n]);
    cone
}

/// Merge two `CpuMesh` instances into one (appends positions, indices, colors, normals).
fn merge_cpu_meshes(mut a: CpuMesh, b: CpuMesh) -> CpuMesh {
    let offset = match &a.positions {
        Positions::F32(v) => v.len() as u32,
        _ => 0,
    };

    if let (Positions::F32(pa), Positions::F32(pb)) = (&mut a.positions, &b.positions) {
        pa.extend_from_slice(pb);
    }

    match (&mut a.indices, &b.indices) {
        (Indices::U32(ia), Indices::U32(ib)) => {
            ia.extend(ib.iter().map(|&i| i + offset));
        }
        _ => {}
    }

    if let (Some(ca), Some(cb)) = (&mut a.colors, &b.colors) {
        ca.extend_from_slice(cb);
    }

    if let (Some(na), Some(nb)) = (&mut a.normals, &b.normals) {
        na.extend_from_slice(nb);
    }

    a
}

// MARK: Colour helpers

/// Convert a hue value (0–1) at full saturation and mid lightness to `Srgba`.
fn hue_to_srgba(hue: f32) -> Srgba {
    let h6 = hue.rem_euclid(1.0) * 6.0;
    let sector = h6 as u32;
    let f = h6 - sector as f32;
    let (r, g, b): (f32, f32, f32) = match sector {
        0 => (1.0, f, 0.0),
        1 => (1.0 - f, 1.0, 0.0),
        2 => (0.0, 1.0, f),
        3 => (0.0, 1.0 - f, 1.0),
        4 => (f, 0.0, 1.0),
        _ => (1.0, 0.0, 1.0 - f),
    };
    Srgba::new((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, 255)
}
