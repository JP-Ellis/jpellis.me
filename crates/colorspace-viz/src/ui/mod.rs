pub mod dom;
pub mod events;
pub mod line_picker;
pub mod slices;

use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;
use web_sys::HtmlSelectElement;

use self::dom::append;
use self::dom::append_text;
use self::dom::as_html;
use self::dom::el;
use self::dom::el_attr;
use self::dom::inject_styles;
use crate::colour::SPACES;

/// Handle to an individual slice panel (canvas + slider + label).
pub struct SlicePanel {
    /// Canvas element on which the 2-D slice is painted.
    pub canvas: HtmlCanvasElement,
    /// Range slider controlling the fixed-axis value for this slice.
    pub slider: HtmlInputElement,
    /// Label showing the current axis name and value (e.g. `B = 0.50`).
    pub label: HtmlElement,
}

/// Handle to all interactive elements in the mounted UI.
pub struct App {
    /// The main 3-D WebGL canvas.
    pub canvas: HtmlCanvasElement,
    /// Colour-space selector drop-down.
    pub select: HtmlSelectElement,
    /// Button that toggles the colour-line overlay.
    pub line_toggle: HtmlElement,
    /// Container div shown when the line overlay is active.
    pub line_picker: HtmlElement,
    /// Three 2-D cross-section panels (one per axis).
    pub slices: [SlicePanel; 3],
    // ── Colour line picker ────────────────────────────────────────────────
    /// Button to start editing endpoint A (left end of gradient bar).
    pub edit_a_btn: HtmlElement,
    /// Button to start editing endpoint B (right end of gradient bar).
    pub edit_b_btn: HtmlElement,
    /// Horizontal gradient bar showing the colour path from A to B.
    pub gradient_bar: HtmlElement,
    /// Container row shown only while actively editing a point.
    pub editing_row: HtmlElement,
    /// CSS colour text input (shown during editing).
    pub css_input: HtmlInputElement,
    /// Button that toggles the gamut-fallback overlay.
    pub fallback_toggle: HtmlElement,
}

/// Mount the visualiser UI into `#colorspace-viz` and return element handles.
pub fn mount(document: &Document) -> App {
    inject_styles(document, CSS);

    let root = document
        .get_element_by_id("colorspace-viz")
        .expect("#colorspace-viz not found");

    // ── 3D canvas ──────────────────────────────────────────────────────────
    let canvas_el = el_attr(document, "canvas", "id", "csv-canvas");
    canvas_el.set_attribute("width", "600").expect("set width");
    canvas_el
        .set_attribute("height", "480")
        .expect("set height");
    append(&root, &canvas_el);

    // ── Space selector ─────────────────────────────────────────────────────
    let select_el = el_attr(document, "select", "id", "csv-space");
    for space in SPACES {
        let opt = el(document, "option");
        append_text(&opt, space.name());
        append(&select_el, &opt);
    }
    append(&root, &select_el);

    // ── Slice panels ───────────────────────────────────────────────────────
    let slices_header = el_attr(document, "p", "class", "csv-slices-header");
    append_text(
        &slices_header,
        "2D cross-sections — each panel holds one axis constant",
    );
    append(&root, &slices_header);

    let slices_div = el_attr(document, "div", "class", "csv-slices");
    let mut slice_panels: Vec<SlicePanel> = Vec::new();
    let initial_axes = SPACES[0].axes();
    for i in 0..3usize {
        let panel_div = el_attr(document, "div", "class", "csv-slice-panel");

        let slice_canvas = el_attr(document, "canvas", "class", "csv-slice-canvas");
        slice_canvas
            .set_attribute("width", "256")
            .expect("set width");
        slice_canvas
            .set_attribute("height", "256")
            .expect("set height");
        slice_canvas
            .set_attribute("data-axis", &i.to_string())
            .expect("set data-axis");
        append(&panel_div, &slice_canvas);

        let slider = el_attr(document, "input", "type", "range");
        slider.set_attribute("min", "0").expect("set min");
        slider.set_attribute("max", "100").expect("set max");
        slider.set_attribute("value", "50").expect("set value");
        slider
            .set_attribute("data-axis", &i.to_string())
            .expect("set data-axis");
        append(&panel_div, &slider);

        let label_el = el_attr(document, "span", "class", "csv-axis-label");
        append_text(&label_el, &format_axis_label(&initial_axes, i, 0.5));
        append(&panel_div, &label_el);

        append(&slices_div, &panel_div);

        let sc = slice_canvas
            .dyn_into::<HtmlCanvasElement>()
            .expect("HtmlCanvasElement");
        let sl = slider
            .dyn_into::<HtmlInputElement>()
            .expect("HtmlInputElement");
        let lbl = as_html(&label_el).clone();
        slice_panels.push(SlicePanel {
            canvas: sc,
            slider: sl,
            label: lbl,
        });
    }
    append(&root, &slices_div);

    // ── Line toggle button ─────────────────────────────────────────────────
    let line_toggle_el = el_attr(document, "button", "id", "csv-line-toggle");
    append_text(&line_toggle_el, "Show colour line");
    append(&root, &line_toggle_el);

    // ── Line picker panel ──────────────────────────────────────────────────
    let line_picker_el = el_attr(document, "div", "id", "csv-line-picker");

    // Point A / B row — gradient bar flanked by edit buttons
    let points_row = el_attr(document, "div", "class", "csv-points-row");

    let edit_a_el = el_attr(document, "button", "class", "csv-edit-btn");
    edit_a_el.set_attribute("id", "csv-edit-a").expect("set id");
    append_text(&edit_a_el, "Edit A");

    let gradient_bar_el = el_attr(document, "div", "id", "csv-gradient-bar");

    let edit_b_el = el_attr(document, "button", "class", "csv-edit-btn");
    edit_b_el.set_attribute("id", "csv-edit-b").expect("set id");
    append_text(&edit_b_el, "Edit B");

    append(&points_row, &edit_a_el);
    append(&points_row, &gradient_bar_el);
    append(&points_row, &edit_b_el);
    append(&line_picker_el, &points_row);

    // Editing row (hidden until Edit A/B is clicked)
    let editing_row_el = el_attr(document, "div", "class", "csv-editing-row");

    let css_input_el = el_attr(document, "input", "type", "text");
    css_input_el
        .set_attribute("id", "csv-css-input")
        .expect("set id");
    css_input_el
        .set_attribute("placeholder", "Any CSS colour…")
        .expect("set placeholder");
    css_input_el
        .set_attribute("class", "csv-color-input")
        .expect("set class");

    append(&editing_row_el, &css_input_el);
    append(&line_picker_el, &editing_row_el);

    let fallback_toggle_el = el_attr(document, "button", "id", "csv-fallback-toggle");
    append_text(&fallback_toggle_el, "Show CSS fallback");
    append(&line_picker_el, &fallback_toggle_el);

    append(&root, &line_picker_el);

    // ── Extract typed handles ──────────────────────────────────────────────
    let canvas = canvas_el
        .dyn_into::<HtmlCanvasElement>()
        .expect("HtmlCanvasElement for canvas");
    let select = select_el
        .dyn_into::<HtmlSelectElement>()
        .expect("HtmlSelectElement");

    let line_toggle = as_html(&line_toggle_el).clone();
    let line_picker = as_html(&line_picker_el).clone();
    let edit_a_btn = as_html(&edit_a_el).clone();
    let edit_b_btn = as_html(&edit_b_el).clone();
    let gradient_bar = as_html(&gradient_bar_el).clone();
    let editing_row = as_html(&editing_row_el).clone();
    let fallback_toggle = as_html(&fallback_toggle_el).clone();
    let css_input = css_input_el
        .dyn_into::<HtmlInputElement>()
        .expect("HtmlInputElement css_input");

    let Ok(slices) = <[SlicePanel; 3]>::try_from(slice_panels) else {
        panic!("expected exactly 3 slice panels");
    };

    App {
        canvas,
        select,
        line_toggle,
        line_picker,
        slices,
        edit_a_btn,
        edit_b_btn,
        gradient_bar,
        editing_row,
        css_input,
        fallback_toggle,
    }
}

// MARK: Helpers

/// Format a slice axis label showing the axis name and current value.
pub fn format_axis_label(axes: &[crate::colour::Axis; 3], axis_idx: usize, raw_val: f32) -> String {
    let axis = &axes[axis_idx];
    let actual = axis.range.0 + raw_val * (axis.range.1 - axis.range.0);
    let range_span = axis.range.1 - axis.range.0;
    if range_span >= 100.0 {
        format!("{} = {:.0}", axis.name, actual)
    } else if range_span >= 1.0 {
        format!("{} = {:.2}", axis.name, actual)
    } else {
        format!("{} = {:.3}", axis.name, actual)
    }
}

// MARK: CSS

static CSS: &str = r#"
#colorspace-viz {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    font-family: system-ui, sans-serif;
}

#csv-canvas {
    width: 100%;
    max-width: 600px;
    height: auto;
    aspect-ratio: 5 / 4;
    border-radius: 8px;
    cursor: grab;
}

#csv-canvas:active {
    cursor: grabbing;
}

#csv-space {
    font-size: 1rem;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
}

.csv-slices {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    justify-content: center;
}

.csv-slice-panel {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
}

.csv-slice-canvas {
    width: 160px;
    height: 160px;
    border-radius: 4px;
    border: 1px solid currentColor;
    opacity: 0.9;
    cursor: crosshair;
}

.csv-slice-panel input[type="range"] {
    width: 160px;
}

.csv-slices-header {
    font-size: 0.85rem;
    opacity: 0.7;
    margin: 0;
    text-align: center;
}

.csv-axis-label {
    font-size: 0.8rem;
    font-family: monospace;
    opacity: 0.8;
}

#csv-line-toggle {
    padding: 0.4rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
}

#csv-line-picker {
    display: none;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
    max-width: 500px;
}

#csv-line-picker.active {
    display: flex;
}

.csv-points-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
}

.csv-edit-btn {
    padding: 0.3rem 0.8rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
    opacity: 0.8;
    white-space: nowrap;
}

.csv-edit-btn.active {
    opacity: 1;
    font-weight: bold;
    outline: 2px solid currentColor;
}

#csv-gradient-bar {
    flex: 1;
    height: 2rem;
    border-radius: 4px;
    border: 1px solid currentColor;
    background-color: #555;
}

.csv-editing-row {
    display: none;
    width: 100%;
}

.csv-editing-row.active {
    display: flex;
}

.csv-color-input {
    font-size: 0.9rem;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-family: monospace;
    width: 100%;
}

#csv-fallback-toggle {
    padding: 0.3rem 0.8rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
    opacity: 0.7;
}

#csv-fallback-toggle.active {
    opacity: 1;
    font-weight: bold;
    outline: 2px solid currentColor;
}
"#;
