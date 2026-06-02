//! Browser event wiring for the colour space visualiser.
//!
//! All closures are intentionally leaked via [`Closure::forget`] so they
//! remain registered for the lifetime of the page.

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::Document;
use web_sys::Element;
use web_sys::Event;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;
use web_sys::MouseEvent;

use crate::colour::SPACES;
use crate::colour::parse;
use crate::render::RenderState;
use crate::ui::App;
use crate::ui::format_axis_label;
use crate::ui::line_picker::clear_gradient_bar;
use crate::ui::line_picker::update_gradient_bar;
use crate::ui::slices::render_slice;
use crate::ui::slices::sync_slices_to_point;

/// Attach all event listeners.
pub fn wire_all(_document: &Document, app: &App, state: Rc<RefCell<RenderState>>) {
    // `editing` tracks which endpoint is being edited: Some(true)=A, Some(false)=B.
    let editing: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(None));
    // `css_focused` suppresses input re-population while the user is typing.
    let css_focused: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));

    wire_space_select(app, Rc::clone(&state), Rc::clone(&editing));
    wire_sliders(app, Rc::clone(&state), Rc::clone(&editing));
    wire_line_toggle(app, Rc::clone(&state), Rc::clone(&editing));
    wire_edit_buttons(
        app,
        Rc::clone(&state),
        Rc::clone(&editing),
        Rc::clone(&css_focused),
    );
    wire_css_input(
        app,
        Rc::clone(&state),
        Rc::clone(&editing),
        Rc::clone(&css_focused),
    );
    wire_slice_clicks(
        app,
        Rc::clone(&state),
        Rc::clone(&editing),
        Rc::clone(&css_focused),
    );
    wire_fallback_toggle(app, Rc::clone(&state));
}

// MARK: Space Select

fn wire_space_select(
    app: &App,
    state: Rc<RefCell<RenderState>>,
    editing: Rc<RefCell<Option<bool>>>,
) {
    let select = app.select.clone();
    let slices = clone_slices(app);
    let sliders = clone_sliders(app);
    let labels = clone_labels(app);
    let editing_row = app.editing_row.clone();
    let edit_a_btn = app.edit_a_btn.clone();
    let edit_b_btn = app.edit_b_btn.clone();
    let gradient_bar = app.gradient_bar.clone();

    let cb = Closure::<dyn Fn(Event)>::new(move |_e: Event| {
        let new_idx = select.selected_index() as usize;
        let new_space = SPACES[new_idx];

        // Convert existing endpoints into the new space so the same two
        // colours are preserved but interpolated through the new space.
        let old_idx = state.borrow().active_space_idx;
        let old_eps = state.borrow().line_endpoints;
        let new_eps = old_eps.and_then(|(a, b)| {
            let a_lin = SPACES[old_idx].to_linear_rgb(a)?;
            let b_lin = SPACES[old_idx].to_linear_rgb(b)?;
            Some((
                new_space.from_linear_rgb(a_lin),
                new_space.from_linear_rgb(b_lin),
            ))
        });

        {
            let mut s = state.borrow_mut();
            s.active_space_idx = new_idx;
            s.mesh_dirty = true;
            s.line_endpoints = new_eps;
            s.line_dirty = true;
        }

        // Exit editing mode on space change.
        *editing.borrow_mut() = None;
        set_editing_row_active(&editing_row, false);
        set_btn_active(&edit_a_btn, false);
        set_btn_active(&edit_b_btn, false);

        if let Some((a, b)) = new_eps {
            update_gradient_bar(&gradient_bar, a, b, new_idx, state.borrow().line_fallback);
        } else {
            clear_gradient_bar(&gradient_bar);
        }

        let axes = new_space.axes();
        for (i, ((canvas, slider), label)) in slices
            .iter()
            .zip(sliders.iter())
            .zip(labels.iter())
            .enumerate()
        {
            let val = slider.value().parse::<f32>().unwrap_or(50.0) / 100.0;
            render_slice(canvas, new_space, i, val, None);
            label.set_inner_text(&format_axis_label(&axes, i, val));
        }
    });

    app.select
        .add_event_listener_with_callback("change", cb.as_ref().unchecked_ref())
        .expect("add_event_listener change");
    cb.forget();
}

// MARK: Sliders

fn wire_sliders(app: &App, state: Rc<RefCell<RenderState>>, editing: Rc<RefCell<Option<bool>>>) {
    let all_canvases = clone_slices(app);
    let all_sliders = clone_sliders(app);
    let all_labels = clone_labels(app);
    let gradient_bar = app.gradient_bar.clone();

    for i in 0..3 {
        let all_canvases = all_canvases.clone();
        let all_sliders = all_sliders.clone();
        let all_labels = all_labels.clone();
        let gradient_bar = gradient_bar.clone();
        let editing = Rc::clone(&editing);
        let state = Rc::clone(&state);

        let cb = Closure::<dyn Fn(Event)>::new(move |_e: Event| {
            let space_idx = state.borrow().active_space_idx;
            let space = SPACES[space_idx];
            let axes = space.axes();
            let raw_val = all_sliders[i].value().parse::<f32>().unwrap_or(50.0) / 100.0;

            if let Some(is_a) = *editing.borrow() {
                // Editing mode: update the editing point's i-th coordinate.
                let actual = axes[i].range.0 + raw_val * (axes[i].range.1 - axes[i].range.0);
                let (coord, a, b) = {
                    let mut s = state.borrow_mut();
                    let ep = s.line_endpoints.get_or_insert(([0.5; 3], [0.5; 3]));
                    if is_a {
                        ep.0[i] = actual;
                    } else {
                        ep.1[i] = actual;
                    }
                    let (a, b) = *ep;
                    let coord = if is_a { a } else { b };
                    s.line_dirty = true;
                    (coord, a, b)
                };
                sync_slices_to_point(coord, space, &all_canvases, &all_sliders, &all_labels);
                update_gradient_bar(&gradient_bar, a, b, space_idx, state.borrow().line_fallback);
            } else {
                // Normal mode: re-render only this slice.
                render_slice(&all_canvases[i], space, i, raw_val, None);
                all_labels[i].set_inner_text(&format_axis_label(&axes, i, raw_val));
            }
        });

        app.slices[i]
            .slider
            .add_event_listener_with_callback("input", cb.as_ref().unchecked_ref())
            .expect("add_event_listener input");
        cb.forget();
    }
}

// MARK: Line Toggle

fn wire_line_toggle(
    app: &App,
    state: Rc<RefCell<RenderState>>,
    editing: Rc<RefCell<Option<bool>>>,
) {
    let line_picker = app.line_picker.clone();
    let edit_a_btn = app.edit_a_btn.clone();
    let edit_b_btn = app.edit_b_btn.clone();
    let editing_row = app.editing_row.clone();
    let gradient_bar = app.gradient_bar.clone();
    let all_canvases = clone_slices(app);
    let all_sliders = clone_sliders(app);
    let all_labels = clone_labels(app);

    let cb = Closure::<dyn Fn(Event)>::new(move |_e: Event| {
        let active = line_picker
            .unchecked_ref::<Element>()
            .class_list()
            .toggle("active")
            .expect("toggle active");

        state.borrow_mut().line_active = active;

        if active {
            let has_endpoints = state.borrow().line_endpoints.is_some();
            if !has_endpoints {
                // Auto-start editing A when line mode first activated with no points set.
                begin_editing(
                    true,
                    &state,
                    &editing,
                    &edit_a_btn,
                    &edit_b_btn,
                    &editing_row,
                    &gradient_bar,
                    &all_canvases,
                    &all_sliders,
                    &all_labels,
                );
            }
        } else {
            // Exiting line mode: clear editing state.
            *editing.borrow_mut() = None;
            set_editing_row_active(&editing_row, false);
            set_btn_active(&edit_a_btn, false);
            set_btn_active(&edit_b_btn, false);
        }
    });

    app.line_toggle
        .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
        .expect("add click line_toggle");
    cb.forget();
}

// MARK: Edit A / Edit B Buttons

fn wire_edit_buttons(
    app: &App,
    state: Rc<RefCell<RenderState>>,
    editing: Rc<RefCell<Option<bool>>>,
    css_focused: Rc<RefCell<bool>>,
) {
    for is_a in [true, false] {
        let edit_a_btn = app.edit_a_btn.clone();
        let edit_b_btn = app.edit_b_btn.clone();
        let editing_row = app.editing_row.clone();
        let gradient_bar = app.gradient_bar.clone();
        let css_input = app.css_input.clone();
        let all_canvases = clone_slices(app);
        let all_sliders = clone_sliders(app);
        let all_labels = clone_labels(app);
        let state = Rc::clone(&state);
        let editing = Rc::clone(&editing);
        let css_focused = Rc::clone(&css_focused);

        let btn = if is_a {
            &app.edit_a_btn
        } else {
            &app.edit_b_btn
        };

        let cb = Closure::<dyn Fn(Event)>::new(move |_e: Event| {
            let already = *editing.borrow() == Some(is_a);
            if already {
                // Clicking the same button again cancels editing (acts as lock-in).
                *editing.borrow_mut() = None;
                set_editing_row_active(&editing_row, false);
                set_btn_active(&edit_a_btn, false);
                set_btn_active(&edit_b_btn, false);
                return;
            }

            begin_editing(
                is_a,
                &state,
                &editing,
                &edit_a_btn,
                &edit_b_btn,
                &editing_row,
                &gradient_bar,
                &all_canvases,
                &all_sliders,
                &all_labels,
            );

            // Populate CSS input with the current point's value (if not typing).
            if !*css_focused.borrow() {
                let space_idx = state.borrow().active_space_idx;
                let space = SPACES[space_idx];
                let coord = current_coord(is_a, &state);
                css_input.set_value(&parse::format_coord_css(coord, space));
            }
        });

        btn.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
            .expect("add click edit btn");
        cb.forget();
    }
}

// MARK: CSS Input

fn wire_css_input(
    app: &App,
    state: Rc<RefCell<RenderState>>,
    editing: Rc<RefCell<Option<bool>>>,
    css_focused: Rc<RefCell<bool>>,
) {
    let input = app.css_input.clone();
    let gradient_bar = app.gradient_bar.clone();
    let all_canvases = clone_slices(app);
    let all_sliders = clone_sliders(app);
    let all_labels = clone_labels(app);

    // Update on input (live parsing)
    {
        let input_cb = input.clone();
        let state = Rc::clone(&state);
        let editing = Rc::clone(&editing);
        let gradient_bar = gradient_bar.clone();
        let all_canvases = all_canvases.clone();
        let all_sliders = all_sliders.clone();
        let all_labels = all_labels.clone();

        let cb = Closure::<dyn Fn(Event)>::new(move |_e: Event| {
            let is_a = match *editing.borrow() {
                Some(v) => v,
                None => return,
            };
            let text = input_cb.value();
            let space_idx = state.borrow().active_space_idx;
            let space = SPACES[space_idx];

            if let Some(coord) = parse::parse_any_css_to_space(&text, space) {
                let (a, b) = {
                    let mut s = state.borrow_mut();
                    let ep = s.line_endpoints.get_or_insert(([0.5; 3], [0.5; 3]));
                    if is_a {
                        ep.0 = coord;
                    } else {
                        ep.1 = coord;
                    }
                    let copy = *ep;
                    s.line_dirty = true;
                    copy
                };
                sync_slices_to_point(coord, space, &all_canvases, &all_sliders, &all_labels);
                update_gradient_bar(&gradient_bar, a, b, space_idx, state.borrow().line_fallback);
            }
        });

        input
            .add_event_listener_with_callback("input", cb.as_ref().unchecked_ref())
            .expect("add input css_input");
        cb.forget();
    }

    // Track focus to avoid overwriting while the user is typing.
    {
        let css_focused = Rc::clone(&css_focused);
        let cb = Closure::<dyn Fn(Event)>::new(move |_e: Event| {
            *css_focused.borrow_mut() = true;
        });
        input
            .add_event_listener_with_callback("focus", cb.as_ref().unchecked_ref())
            .expect("add focus css_input");
        cb.forget();
    }
    {
        let cb = Closure::<dyn Fn(Event)>::new(move |_e: Event| {
            *css_focused.borrow_mut() = false;
        });
        app.css_input
            .add_event_listener_with_callback("blur", cb.as_ref().unchecked_ref())
            .expect("add blur css_input");
        cb.forget();
    }
}

// MARK: Slice Click

fn wire_slice_clicks(
    app: &App,
    state: Rc<RefCell<RenderState>>,
    editing: Rc<RefCell<Option<bool>>>,
    css_focused: Rc<RefCell<bool>>,
) {
    for i in 0..3 {
        let canvas = app.slices[i].canvas.clone();
        let all_canvases = clone_slices(app);
        let all_sliders = clone_sliders(app);
        let all_labels = clone_labels(app);
        let gradient_bar = app.gradient_bar.clone();
        let css_input = app.css_input.clone();
        let state = Rc::clone(&state);
        let editing = Rc::clone(&editing);
        let css_focused = Rc::clone(&css_focused);

        let cb = Closure::<dyn Fn(MouseEvent)>::new(move |e: MouseEvent| {
            let is_a = match *editing.borrow() {
                Some(v) => v,
                None => return,
            };

            // Convert click position to normalised [0,1] within the canvas.
            let css_w = canvas.client_width() as f64;
            let css_h = canvas.client_height() as f64;
            if css_w == 0.0 || css_h == 0.0 {
                return;
            }
            let norm_u = (e.offset_x() as f64 / css_w).clamp(0.0, 1.0) as f32;
            let norm_v = (e.offset_y() as f64 / css_h).clamp(0.0, 1.0) as f32;

            let space_idx = state.borrow().active_space_idx;
            let space = SPACES[space_idx];
            let axes = space.axes();

            // For slice `i` (axis i fixed), the two varying axes are ax and ay.
            let ax = (i + 1) % 3;
            let ay = (i + 2) % 3;

            let (coord, a, b) = {
                let mut s = state.borrow_mut();
                let ep = s.line_endpoints.get_or_insert(([0.5; 3], [0.5; 3]));
                if is_a {
                    ep.0[ax] = axes[ax].range.0 + norm_u * (axes[ax].range.1 - axes[ax].range.0);
                    ep.0[ay] = axes[ay].range.0 + norm_v * (axes[ay].range.1 - axes[ay].range.0);
                } else {
                    ep.1[ax] = axes[ax].range.0 + norm_u * (axes[ax].range.1 - axes[ax].range.0);
                    ep.1[ay] = axes[ay].range.0 + norm_v * (axes[ay].range.1 - axes[ay].range.0);
                }
                let (a, b) = *ep;
                let coord = if is_a { a } else { b };
                s.line_dirty = true;
                (coord, a, b)
            };

            sync_slices_to_point(coord, space, &all_canvases, &all_sliders, &all_labels);
            update_gradient_bar(&gradient_bar, a, b, space_idx, state.borrow().line_fallback);

            // Update CSS input if not focused.
            if !*css_focused.borrow() {
                css_input.set_value(&parse::format_coord_css(coord, space));
            }
        });

        app.slices[i]
            .canvas
            .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
            .expect("add click slice");
        cb.forget();
    }
}

// MARK: Fallback Toggle

fn wire_fallback_toggle(app: &App, state: Rc<RefCell<RenderState>>) {
    let fallback_toggle = app.fallback_toggle.clone();
    let gradient_bar = app.gradient_bar.clone();

    let cb = Closure::<dyn Fn(Event)>::new(move |_e: Event| {
        let (new_fallback, space_idx, endpoints) = {
            let mut s = state.borrow_mut();
            s.line_fallback = !s.line_fallback;
            s.line_dirty = true;
            (s.line_fallback, s.active_space_idx, s.line_endpoints)
        };

        let cl = fallback_toggle.unchecked_ref::<Element>().class_list();
        if new_fallback {
            cl.add_1("active").expect("add active");
            fallback_toggle.set_inner_text("Hide CSS fallback");
        } else {
            cl.remove_1("active").expect("remove active");
            fallback_toggle.set_inner_text("Show CSS fallback");
        }

        if let Some((a, b)) = endpoints {
            update_gradient_bar(&gradient_bar, a, b, space_idx, new_fallback);
        }
    });

    app.fallback_toggle
        .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
        .expect("add click fallback_toggle");
    cb.forget();
}

// MARK: Shared helpers

/// Enter editing mode for the given point, snapping slices to the current coord.
fn begin_editing(
    is_a: bool,
    state: &Rc<RefCell<RenderState>>,
    editing: &Rc<RefCell<Option<bool>>>,
    edit_a_btn: &HtmlElement,
    edit_b_btn: &HtmlElement,
    editing_row: &HtmlElement,
    gradient_bar: &HtmlElement,
    all_canvases: &[HtmlCanvasElement; 3],
    all_sliders: &[HtmlInputElement; 3],
    all_labels: &[HtmlElement; 3],
) {
    *editing.borrow_mut() = Some(is_a);
    set_btn_active(edit_a_btn, is_a);
    set_btn_active(edit_b_btn, !is_a);
    set_editing_row_active(editing_row, true);

    let (a, b) = state
        .borrow()
        .line_endpoints
        .unwrap_or(([0.5; 3], [0.5; 3]));
    let coord = if is_a { a } else { b };
    let space_idx = state.borrow().active_space_idx;
    let space = SPACES[space_idx];

    sync_slices_to_point(coord, space, all_canvases, all_sliders, all_labels);
    update_gradient_bar(gradient_bar, a, b, space_idx, state.borrow().line_fallback);
}

/// Get the current coordinate of the given endpoint (or a default midpoint).
fn current_coord(is_a: bool, state: &Rc<RefCell<RenderState>>) -> [f32; 3] {
    state
        .borrow()
        .line_endpoints
        .map(|(a, b)| if is_a { a } else { b })
        .unwrap_or([0.5; 3])
}

fn set_btn_active(btn: &HtmlElement, active: bool) {
    let cl = btn.unchecked_ref::<Element>().class_list();
    if active {
        cl.add_1("active").expect("add active");
    } else {
        cl.remove_1("active").expect("remove active");
    }
}

fn set_editing_row_active(row: &HtmlElement, active: bool) {
    let cl = row.unchecked_ref::<Element>().class_list();
    if active {
        cl.add_1("active").expect("add active");
    } else {
        cl.remove_1("active").expect("remove active");
    }
}

// MARK: Clone helpers

fn clone_slices(app: &App) -> [HtmlCanvasElement; 3] {
    [
        app.slices[0].canvas.clone(),
        app.slices[1].canvas.clone(),
        app.slices[2].canvas.clone(),
    ]
}

fn clone_sliders(app: &App) -> [HtmlInputElement; 3] {
    [
        app.slices[0].slider.clone(),
        app.slices[1].slider.clone(),
        app.slices[2].slider.clone(),
    ]
}

fn clone_labels(app: &App) -> [HtmlElement; 3] {
    [
        app.slices[0].label.clone(),
        app.slices[1].label.clone(),
        app.slices[2].label.clone(),
    ]
}
