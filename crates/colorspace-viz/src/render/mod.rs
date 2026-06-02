//! three-d (WebGL2/glow) render pipeline for the colour space visualiser.

pub mod geometry;

#[cfg(target_arch = "wasm32")]
mod gizmo;

#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use three_d::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;
#[cfg(target_arch = "wasm32")]
use web_sys::PointerEvent;
#[cfg(target_arch = "wasm32")]
use web_sys::WheelEvent;

#[cfg(target_arch = "wasm32")]
/// Shared mutable state driven by UI events and consumed by the render loop.
pub struct RenderState {
    /// Index into [`crate::colour::SPACES`].
    pub active_space_idx: usize,
    /// Colour-line endpoints in the active colour space's coordinates, or `None` when unset.
    pub line_endpoints: Option<([f32; 3], [f32; 3])>,
    /// Whether the colour-line overlay is currently visible.
    pub line_active: bool,
    /// Set to `true` when the gamut mesh needs to be rebuilt.
    pub mesh_dirty: bool,
    /// Set to `true` when the colour-line instances need to be rebuilt.
    pub line_dirty: bool,
    /// Whether to render fallback spheres and connectors for out-of-gamut line points.
    pub line_fallback: bool,
}

#[cfg(target_arch = "wasm32")]
impl RenderState {
    /// Create default render state with space 0 selected and no line active.
    pub fn new() -> Self {
        Self {
            active_space_idx: 0,
            line_endpoints: None,
            line_active: false,
            mesh_dirty: false,
            line_dirty: false,
            line_fallback: false,
        }
    }
}

#[cfg(target_arch = "wasm32")]
/// Start the `requestAnimationFrame` render loop using three-d (glow/WebGL2).
pub fn start_loop(canvas: HtmlCanvasElement, state: Rc<RefCell<RenderState>>) {
    let width = canvas.width();
    let height = canvas.height();

    // ── WebGL2 context ────────────────────────────────────────────────────────
    let webgl2 = canvas
        .get_context("webgl2")
        .expect("getContext webgl2")
        .expect("WebGL2 not supported")
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .expect("dyn_into WebGl2RenderingContext");

    let glow_ctx = three_d::context::Context::from_webgl2_context(webgl2);
    let context = Context::from_gl_context(Arc::new(glow_ctx)).expect("three-d Context");

    // ── Camera ────────────────────────────────────────────────────────────────
    let orbit_target = vec3(0.0f32, 0.0, 0.0);
    let camera_rc: Rc<RefCell<Camera>> = Rc::new(RefCell::new(Camera::new_perspective(
        Viewport::new_at_origo(width, height),
        vec3(0.0, 0.0, 2.0),
        orbit_target,
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        100.0,
    )));

    // ── Lights ────────────────────────────────────────────────────────────────
    let ambient = AmbientLight::new(&context, 0.3, Srgba::WHITE);
    let directional1 = DirectionalLight::new(&context, 1.5, Srgba::WHITE, vec3(-1.0, -1.0, -1.0));
    let directional2 = DirectionalLight::new(&context, 0.5, Srgba::WHITE, vec3(1.0, 1.0, 0.5));

    // ── Initial gamut mesh ────────────────────────────────────────────────────
    let initial_cpu = geometry::build_cpu_mesh(0);
    let solid_rc: Rc<RefCell<Gm<Mesh, PhysicalMaterial>>> = Rc::new(RefCell::new(Gm::new(
        Mesh::new(&context, &initial_cpu),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::WHITE,
                roughness: 0.7,
                metallic: 0.0,
                ..Default::default()
            },
        ),
    )));

    // ── Line overlay (instanced unit spheres) ─────────────────────────────────
    let sphere_cpu = CpuMesh::sphere(6);
    let line_rc: Rc<RefCell<Gm<InstancedMesh, ColorMaterial>>> = Rc::new(RefCell::new(Gm::new(
        InstancedMesh::new(&context, &Instances::default(), &sphere_cpu),
        ColorMaterial {
            color: Srgba::WHITE,
            render_states: RenderStates {
                depth_test: DepthTest::Always,
                ..Default::default()
            },
            ..Default::default()
        },
    )));

    let fallback_rc: Rc<RefCell<Gm<InstancedMesh, ColorMaterial>>> =
        Rc::new(RefCell::new(Gm::new(
            InstancedMesh::new(&context, &Instances::default(), &sphere_cpu),
            ColorMaterial {
                color: Srgba::WHITE,
                render_states: RenderStates {
                    depth_test: DepthTest::Always,
                    ..Default::default()
                },
                ..Default::default()
            },
        )));

    let cylinder_cpu = CpuMesh::cylinder(4);
    let connectors_rc: Rc<RefCell<Gm<InstancedMesh, ColorMaterial>>> =
        Rc::new(RefCell::new(Gm::new(
            InstancedMesh::new(&context, &Instances::default(), &cylinder_cpu),
            ColorMaterial {
                color: Srgba::new(128, 128, 128, 84),
                is_transparent: true,
                render_states: RenderStates {
                    depth_test: DepthTest::Always,
                    blend: Blend::STANDARD_TRANSPARENCY,
                    write_mask: WriteMask::COLOR,
                    ..Default::default()
                },
                ..Default::default()
            },
        )));

    // ── Corner gizmo ──────────────────────────────────────────────────────────
    let initial_shape = crate::colour::SPACES[0].shape();
    let gizmo_rc: Rc<RefCell<gizmo::Gizmo>> =
        Rc::new(RefCell::new(gizmo::Gizmo::new(&context, initial_shape)));

    // ── Orbit event wiring ────────────────────────────────────────────────────
    wire_orbit(&canvas, Rc::clone(&camera_rc), orbit_target);

    // ── RAF loop ──────────────────────────────────────────────────────────────
    // Ping-pong Rc<RefCell<Option<Closure>>> so the closure stays alive.
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);

    let context_c = context.clone();
    let state_c = Rc::clone(&state);
    let camera_c = Rc::clone(&camera_rc);
    let solid_c = Rc::clone(&solid_rc);
    let line_c = Rc::clone(&line_rc);
    let fallback_c = Rc::clone(&fallback_rc);
    let connectors_c = Rc::clone(&connectors_rc);
    let gizmo_c = Rc::clone(&gizmo_rc);
    let mut was_line_active = false;

    *g.borrow_mut() = Some(Closure::new(move || {
        // ── Dirty-flag processing (mutable phase) ─────────────────────────────
        {
            let mut s = state_c.borrow_mut();
            if s.mesh_dirty {
                let space = crate::colour::SPACES[s.active_space_idx];
                let cpu = geometry::build_cpu_mesh(s.active_space_idx);
                solid_c.borrow_mut().geometry = Mesh::new(&context_c, &cpu);
                *gizmo_c.borrow_mut() = gizmo::Gizmo::new(&context_c, space.shape());
                s.mesh_dirty = false;
            }
            if s.line_dirty {
                let sphere = CpuMesh::sphere(6);
                let cylinder = CpuMesh::cylinder(4);
                if let Some((a, b)) = s.line_endpoints {
                    let instances =
                        geometry::build_line_instances(s.active_space_idx, a, b, s.line_fallback);
                    line_c.borrow_mut().geometry =
                        InstancedMesh::new(&context_c, &instances.primary, &sphere);
                    fallback_c.borrow_mut().geometry =
                        InstancedMesh::new(&context_c, &instances.fallback_spheres, &sphere);
                    connectors_c.borrow_mut().geometry =
                        InstancedMesh::new(&context_c, &instances.connectors, &cylinder);
                } else {
                    let empty = Instances::default();
                    line_c.borrow_mut().geometry = InstancedMesh::new(&context_c, &empty, &sphere);
                    fallback_c.borrow_mut().geometry =
                        InstancedMesh::new(&context_c, &empty, &sphere);
                    connectors_c.borrow_mut().geometry =
                        InstancedMesh::new(&context_c, &empty, &cylinder);
                }
                s.line_dirty = false;
            }
        }

        // ── Make the gamut solid semi-transparent when the line overlay is active
        // Use STANDARD_TRANSPARENCY (dst_alpha = src_alpha) so the canvas stores
        // correct premultiplied alpha for compositing over the page background.
        // TRANSPARENCY has source_alpha_multiplier=Zero which leaves dst_alpha=0,
        // causing the browser to add full page_bg on top (overflows to white).
        let line_active = state_c.borrow().line_active;
        if line_active != was_line_active {
            let mat = &mut solid_c.borrow_mut().material;
            if line_active {
                mat.albedo = Srgba::new(255, 255, 255, 80);
                mat.is_transparent = true;
                mat.render_states.blend = Blend::STANDARD_TRANSPARENCY;
                // Don't write depth so the sphere pass (below) can composite
                // correctly over the solid regardless of geometric depth.
                mat.render_states.write_mask = WriteMask::COLOR;
            } else {
                mat.albedo = Srgba::WHITE;
                mat.is_transparent = false;
                mat.render_states.blend = Blend::Disabled;
                mat.render_states.write_mask = WriteMask::default();
            }
            was_line_active = line_active;
        }

        // ── Render ────────────────────────────────────────────────────────────
        let camera = camera_c.borrow();
        let solid = solid_c.borrow();
        let line = line_c.borrow();
        let lights: &[&dyn Light] = &[&ambient, &directional1, &directional2];

        let screen = RenderTarget::screen(&context_c, width, height);
        screen.clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 0.0, 1.0));

        if line_active {
            let line_fallback = state_c.borrow().line_fallback;
            screen.render(&*camera, [&*solid as &dyn Object], lights);
            screen.render(&*camera, [&*line as &dyn Object], lights);
            if line_fallback {
                let fallback = fallback_c.borrow();
                let connectors = connectors_c.borrow();
                screen.render(&*camera, [&*fallback as &dyn Object], lights);
                screen.render(&*camera, [&*connectors as &dyn Object], lights);
            }
        } else {
            screen.render(&*camera, [&*solid as &dyn Object], lights);
        }

        // ── Corner gizmo ──────────────────────────────────────────────────────
        gizmo::render_corner(&gizmo_c.borrow(), &screen, &camera, height);

        web_sys::window()
            .expect("no window")
            .request_animation_frame(
                f.borrow()
                    .as_ref()
                    .expect("closure")
                    .as_ref()
                    .unchecked_ref(),
            )
            .expect("requestAnimationFrame");
    }));

    web_sys::window()
        .expect("no window")
        .request_animation_frame(
            g.borrow()
                .as_ref()
                .expect("closure")
                .as_ref()
                .unchecked_ref(),
        )
        .expect("requestAnimationFrame");
}

// MARK: Orbit wiring

#[cfg(target_arch = "wasm32")]
fn wire_orbit(canvas: &HtmlCanvasElement, camera: Rc<RefCell<Camera>>, target: Vec3) {
    let dragging: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
    let last_x: Rc<RefCell<f32>> = Rc::new(RefCell::new(0.0));
    let last_y: Rc<RefCell<f32>> = Rc::new(RefCell::new(0.0));

    // pointerdown
    {
        let dragging = Rc::clone(&dragging);
        let last_x = Rc::clone(&last_x);
        let last_y = Rc::clone(&last_y);
        let canvas_clone = canvas.clone();
        let cb = Closure::<dyn Fn(PointerEvent)>::new(move |e: PointerEvent| {
            *dragging.borrow_mut() = true;
            *last_x.borrow_mut() = e.client_x() as f32;
            *last_y.borrow_mut() = e.client_y() as f32;
            canvas_clone
                .set_pointer_capture(e.pointer_id())
                .expect("setPointerCapture");
        });
        canvas
            .add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref())
            .expect("add pointerdown");
        cb.forget();
    }

    // pointermove
    {
        let dragging = Rc::clone(&dragging);
        let last_x = Rc::clone(&last_x);
        let last_y = Rc::clone(&last_y);
        let camera = Rc::clone(&camera);
        let cb = Closure::<dyn Fn(PointerEvent)>::new(move |e: PointerEvent| {
            if !*dragging.borrow() {
                return;
            }
            let x = e.client_x() as f32;
            let y = e.client_y() as f32;
            let dx = x - *last_x.borrow();
            let dy = y - *last_y.borrow();
            *last_x.borrow_mut() = x;
            *last_y.borrow_mut() = y;
            camera
                .borrow_mut()
                .rotate_around_with_fixed_up(target, dx * 0.01, dy * 0.01);
        });
        canvas
            .add_event_listener_with_callback("pointermove", cb.as_ref().unchecked_ref())
            .expect("add pointermove");
        cb.forget();
    }

    // pointerup
    {
        let dragging = Rc::clone(&dragging);
        let cb = Closure::<dyn Fn(PointerEvent)>::new(move |_e: PointerEvent| {
            *dragging.borrow_mut() = false;
        });
        canvas
            .add_event_listener_with_callback("pointerup", cb.as_ref().unchecked_ref())
            .expect("add pointerup");
        cb.forget();
    }

    // wheel
    {
        let cb = Closure::<dyn Fn(WheelEvent)>::new(move |e: WheelEvent| {
            e.prevent_default();
            let dist = target.distance(camera.borrow().position());
            let delta = dist * (1.0 - (-e.delta_y() as f32 * 0.001).exp());
            camera.borrow_mut().zoom_towards(target, delta, 0.5, 10.0);
        });
        canvas
            .add_event_listener_with_callback("wheel", cb.as_ref().unchecked_ref())
            .expect("add wheel");
        cb.forget();
    }
}
