use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;

use crate::colour::ColourSpace;
use crate::colour::srgb::linear_to_srgb_channel;
use crate::ui::format_axis_label;

/// Rasterise a 256×256 cross-section of `space` onto `canvas`.
///
/// `axis_idx` is the axis held constant; `value` is its normalised `[0,1]` position.
/// The two varying axes are mapped to pixel `(x, y)` respectively.
///
/// `crosshair` is an optional `(norm_x, norm_y)` in `[0,1]` where a contrasting
/// crosshair marker should be drawn over the rendered slice.
pub fn render_slice(
    canvas: &HtmlCanvasElement,
    space: &dyn ColourSpace,
    axis_idx: usize,
    value: f32,
    crosshair: Option<(f32, f32)>,
) {
    let axes = space.axes();
    let size: usize = 256;
    let mut data = vec![0u8; size * size * 4];

    let ax = (axis_idx + 1) % 3;
    let ay = (axis_idx + 2) % 3;

    for py in 0..size {
        for px in 0..size {
            let u = px as f32 / (size - 1) as f32;
            let v = py as f32 / (size - 1) as f32;

            let mut coord = [0.0f32; 3];
            coord[axis_idx] =
                axes[axis_idx].range.0 + value * (axes[axis_idx].range.1 - axes[axis_idx].range.0);
            coord[ax] = axes[ax].range.0 + u * (axes[ax].range.1 - axes[ax].range.0);
            coord[ay] = axes[ay].range.0 + v * (axes[ay].range.1 - axes[ay].range.0);

            let offset = (py * size + px) * 4;
            if let Some(linear) = space.to_linear_rgb(coord) {
                data[offset] = (linear_to_srgb_channel(linear[0]).clamp(0.0, 1.0) * 255.0) as u8;
                data[offset + 1] =
                    (linear_to_srgb_channel(linear[1]).clamp(0.0, 1.0) * 255.0) as u8;
                data[offset + 2] =
                    (linear_to_srgb_channel(linear[2]).clamp(0.0, 1.0) * 255.0) as u8;
                data[offset + 3] = 255;
            }
            // else: leave as 0,0,0,0 (transparent)
        }
    }

    let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&data),
        size as u32,
        size as u32,
    )
    .expect("ImageData::new failed");

    let ctx = canvas
        .get_context("2d")
        .expect("get_context failed")
        .expect("get_context returned None")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("dyn_into CanvasRenderingContext2d");

    ctx.put_image_data(&image_data, 0.0, 0.0)
        .expect("put_image_data failed");

    if let Some((cx, cy)) = crosshair {
        draw_crosshair(&ctx, cx, cy, size as f64);
    }
}

#[allow(deprecated)]
/// Draw a two-tone crosshair at normalised position `(cx, cy)` on `ctx`.
fn draw_crosshair(ctx: &web_sys::CanvasRenderingContext2d, cx: f32, cy: f32, size: f64) {
    let px = cx as f64 * (size - 1.0);
    let py = cy as f64 * (size - 1.0);
    const GAP: f64 = 6.0;
    const ARM: f64 = 14.0;

    // Dark outer lines
    ctx.set_stroke_style(&JsValue::from_str("rgba(0,0,0,0.8)"));
    ctx.set_line_width(3.0);
    stroke_crosshair(ctx, px, py, GAP, ARM, size);

    // Bright inner lines
    ctx.set_stroke_style(&JsValue::from_str("rgba(255,255,255,0.95)"));
    ctx.set_line_width(1.5);
    stroke_crosshair(ctx, px, py, GAP, ARM, size);
}

fn stroke_crosshair(
    ctx: &web_sys::CanvasRenderingContext2d,
    px: f64,
    py: f64,
    gap: f64,
    arm: f64,
    size: f64,
) {
    // Horizontal left arm
    if px - gap > 0.0 {
        ctx.begin_path();
        ctx.move_to((px - gap).max(0.0), py);
        ctx.line_to((px - gap - arm).max(0.0), py);
        ctx.stroke();
    }
    // Horizontal right arm
    if px + gap < size {
        ctx.begin_path();
        ctx.move_to((px + gap).min(size), py);
        ctx.line_to((px + gap + arm).min(size), py);
        ctx.stroke();
    }
    // Vertical top arm
    if py - gap > 0.0 {
        ctx.begin_path();
        ctx.move_to(px, (py - gap).max(0.0));
        ctx.line_to(px, (py - gap - arm).max(0.0));
        ctx.stroke();
    }
    // Vertical bottom arm
    if py + gap < size {
        ctx.begin_path();
        ctx.move_to(px, (py + gap).min(size));
        ctx.line_to(px, (py + gap + arm).min(size));
        ctx.stroke();
    }
}

/// Update all three slice canvases, sliders, and labels to show cross-sections
/// through `coord` (in native colour-space units), with crosshairs.
///
/// This is the main sync call used during interactive editing of a colour endpoint.
pub(crate) fn sync_slices_to_point(
    coord: [f32; 3],
    space: &dyn ColourSpace,
    canvases: &[HtmlCanvasElement; 3],
    sliders: &[HtmlInputElement; 3],
    labels: &[HtmlElement; 3],
) {
    let axes = space.axes();

    for i in 0..3 {
        let range = axes[i].range;
        let norm = ((coord[i] - range.0) / (range.1 - range.0)).clamp(0.0, 1.0);

        let slider_val = (norm * 100.0).round().clamp(0.0, 100.0) as u32;
        sliders[i].set_value(&slider_val.to_string());
        labels[i].set_inner_text(&format_axis_label(&axes, i, norm));

        // Crosshair for the two varying axes of this slice.
        let ax = (i + 1) % 3;
        let ay = (i + 2) % 3;
        let cx = ((coord[ax] - axes[ax].range.0) / (axes[ax].range.1 - axes[ax].range.0))
            .clamp(0.0, 1.0);
        let cy = ((coord[ay] - axes[ay].range.0) / (axes[ay].range.1 - axes[ay].range.0))
            .clamp(0.0, 1.0);

        render_slice(&canvases[i], space, i, norm, Some((cx, cy)));
    }
}
