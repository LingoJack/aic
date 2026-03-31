use image::{Rgba, RgbaImage};

use crate::cli::PreviewAction;
use crate::error::AicError;
use crate::screenshot::{capture_screen, output_image};

// Orange matching the live indicator
const COLOR: Rgba<u8> = Rgba([255, 140, 0, 220]);
const COLOR_DIM: Rgba<u8> = Rgba([255, 140, 0, 120]);
const COLOR_END: Rgba<u8> = Rgba([50, 180, 255, 220]); // blue for drag end / scroll direction

// --- drawing primitives ---

pub(crate) fn blend_pixel(img: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>) {
    if x < 0 || y < 0 || x >= img.width() as i32 || y >= img.height() as i32 {
        return;
    }
    let bg = *img.get_pixel(x as u32, y as u32);
    let a = color.0[3] as f64 / 255.0;
    let inv = 1.0 - a;
    let r = (color.0[0] as f64 * a + bg.0[0] as f64 * inv) as u8;
    let g = (color.0[1] as f64 * a + bg.0[1] as f64 * inv) as u8;
    let b = (color.0[2] as f64 * a + bg.0[2] as f64 * inv) as u8;
    img.put_pixel(x as u32, y as u32, Rgba([r, g, b, 255]));
}

fn draw_circle(img: &mut RgbaImage, cx: f64, cy: f64, radius: f64, thickness: f64, color: Rgba<u8>) {
    let r_min = radius - thickness / 2.0;
    let r_max = radius + thickness / 2.0;
    let x0 = (cx - r_max - 1.0).max(0.0) as i32;
    let x1 = ((cx + r_max + 1.0) as i32).min(img.width() as i32 - 1);
    let y0 = (cy - r_max - 1.0).max(0.0) as i32;
    let y1 = ((cy + r_max + 1.0) as i32).min(img.height() as i32 - 1);

    for py in y0..=y1 {
        for px in x0..=x1 {
            let dx = px as f64 - cx;
            let dy = py as f64 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist >= r_min && dist <= r_max {
                blend_pixel(img, px, py, color);
            }
        }
    }
}

fn draw_filled_circle(img: &mut RgbaImage, cx: f64, cy: f64, radius: f64, color: Rgba<u8>) {
    let r2 = radius * radius;
    let x0 = (cx - radius - 1.0).max(0.0) as i32;
    let x1 = ((cx + radius + 1.0) as i32).min(img.width() as i32 - 1);
    let y0 = (cy - radius - 1.0).max(0.0) as i32;
    let y1 = ((cy + radius + 1.0) as i32).min(img.height() as i32 - 1);

    for py in y0..=y1 {
        for px in x0..=x1 {
            let dx = px as f64 - cx;
            let dy = py as f64 - cy;
            if dx * dx + dy * dy <= r2 {
                blend_pixel(img, px, py, color);
            }
        }
    }
}

pub(crate) fn draw_line(img: &mut RgbaImage, x1: f64, y1: f64, x2: f64, y2: f64, thickness: f64, color: Rgba<u8>) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.5 {
        return;
    }
    let steps = (len * 2.0) as usize;
    let half_t = thickness / 2.0;
    // Normal vector (perpendicular)
    let nx = -dy / len;
    let ny = dx / len;

    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let cx = x1 + dx * t;
        let cy = y1 + dy * t;
        let spread = (half_t.ceil()) as i32;
        for j in -spread..=spread {
            let fj = j as f64;
            if fj.abs() > half_t {
                continue;
            }
            let px = (cx + nx * fj) as i32;
            let py = (cy + ny * fj) as i32;
            blend_pixel(img, px, py, color);
        }
    }
}

fn draw_dashed_line(
    img: &mut RgbaImage,
    x1: f64, y1: f64, x2: f64, y2: f64,
    thickness: f64, dash: f64, gap: f64,
    color: Rgba<u8>,
) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.5 {
        return;
    }
    let ux = dx / len;
    let uy = dy / len;

    let cycle = dash + gap;
    let mut pos = 0.0;
    while pos < len {
        let seg_end = (pos + dash).min(len);
        let sx = x1 + ux * pos;
        let sy = y1 + uy * pos;
        let ex = x1 + ux * seg_end;
        let ey = y1 + uy * seg_end;
        draw_line(img, sx, sy, ex, ey, thickness, color);
        pos += cycle;
    }
}

fn draw_cross(img: &mut RgbaImage, cx: f64, cy: f64, size: f64, thickness: f64, color: Rgba<u8>) {
    draw_line(img, cx - size, cy, cx + size, cy, thickness, color);
    draw_line(img, cx, cy - size, cx, cy + size, thickness, color);
}

pub(crate) fn draw_rect(img: &mut RgbaImage, x: f64, y: f64, w: f64, h: f64, thickness: f64, color: Rgba<u8>) {
    // Top
    draw_line(img, x, y, x + w, y, thickness, color);
    // Bottom
    draw_line(img, x, y + h, x + w, y + h, thickness, color);
    // Left
    draw_line(img, x, y, x, y + h, thickness, color);
    // Right
    draw_line(img, x + w, y, x + w, y + h, thickness, color);
}

pub(crate) fn draw_filled_rect(img: &mut RgbaImage, x: i32, y: i32, w: i32, h: i32, color: Rgba<u8>) {
    for py in y..(y + h) {
        for px in x..(x + w) {
            blend_pixel(img, px, py, color);
        }
    }
}

fn draw_arrowhead(img: &mut RgbaImage, tip_x: f64, tip_y: f64, from_x: f64, from_y: f64, arrow_len: f64, thickness: f64, color: Rgba<u8>) {
    let dx = tip_x - from_x;
    let dy = tip_y - from_y;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.5 {
        return;
    }
    let ux = dx / len;
    let uy = dy / len;

    let angle: f64 = 28.0_f64.to_radians();
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    // Left wing
    let lx = tip_x - arrow_len * (ux * cos_a + uy * sin_a);
    let ly = tip_y - arrow_len * (-ux * sin_a + uy * cos_a);
    // Right wing
    let rx = tip_x - arrow_len * (ux * cos_a - uy * sin_a);
    let ry = tip_y - arrow_len * (ux * sin_a + uy * cos_a);

    draw_line(img, tip_x, tip_y, lx, ly, thickness, color);
    draw_line(img, tip_x, tip_y, rx, ry, thickness, color);
}

// --- preview drawing for each action ---

fn draw_click_marker(img: &mut RgbaImage, px: f64, py: f64, scale: f64) {
    let r = 20.0 * scale;
    let thick = 3.0 * scale;
    let cross_size = 10.0 * scale;
    let dot_r = 4.0 * scale;

    draw_circle(img, px, py, r, thick, COLOR);
    draw_cross(img, px, py, cross_size, thick * 0.8, COLOR_DIM);
    draw_filled_circle(img, px, py, dot_r, COLOR);
}

fn draw_drag_marker(
    img: &mut RgbaImage,
    sx: f64, sy: f64, ex: f64, ey: f64,
    scale: f64,
) {
    let thick = 3.0 * scale;
    let r = 16.0 * scale;
    let dot_r = 5.0 * scale;
    let arrow_len = 16.0 * scale;

    // Start: filled circle
    draw_filled_circle(img, sx, sy, dot_r, COLOR);
    draw_circle(img, sx, sy, r, thick, COLOR);

    // Dashed line
    draw_dashed_line(img, sx, sy, ex, ey, thick * 0.8, 12.0 * scale, 8.0 * scale, COLOR_DIM);

    // End: hollow circle + arrowhead
    draw_circle(img, ex, ey, r, thick, COLOR_END);
    draw_arrowhead(img, ex, ey, sx, sy, arrow_len, thick, COLOR_END);
}

fn draw_scroll_marker(
    img: &mut RgbaImage,
    px: f64, py: f64, dx: i32, dy: i32,
    scale: f64,
) {
    let r = 18.0 * scale;
    let thick = 3.0 * scale;
    let arrow_len = 14.0 * scale;
    let shaft = 36.0 * scale;

    // Circle at position
    draw_circle(img, px, py, r, thick, COLOR);
    draw_filled_circle(img, px, py, 4.0 * scale, COLOR);

    // Vertical arrow (dy: positive = up, negative = down)
    if dy != 0 {
        let dir = if dy > 0 { -1.0 } else { 1.0 }; // screen coords: up is negative
        let tip_y = py + dir * (r + shaft);
        let base_y = py + dir * r;
        draw_line(img, px, base_y, px, tip_y, thick, COLOR_END);
        draw_arrowhead(img, px, tip_y, px, base_y, arrow_len, thick, COLOR_END);
    }

    // Horizontal arrow (dx: positive = right, negative = left)
    if dx != 0 {
        let dir = if dx > 0 { 1.0 } else { -1.0 };
        let tip_x = px + dir * (r + shaft);
        let base_x = px + dir * r;
        draw_line(img, base_x, py, tip_x, py, thick, COLOR_END);
        draw_arrowhead(img, tip_x, py, base_x, py, arrow_len, thick, COLOR_END);
    }
}

// --- public entry ---

pub fn preview_mouse_action(action: &PreviewAction, output: Option<&str>) -> Result<(), AicError> {
    let (mut img, scale) = capture_screen()?;

    match action {
        PreviewAction::Click { x, y, .. }
        | PreviewAction::Doubleclick { x, y, .. }
        | PreviewAction::Rightclick { x, y, .. }
        | PreviewAction::Move { x, y, .. }
        | PreviewAction::Longpress { x, y, .. } => {
            draw_click_marker(&mut img, x * scale, y * scale, scale);
        }
        PreviewAction::Drag { x1, y1, x2, y2, .. } => {
            draw_drag_marker(
                &mut img,
                x1 * scale, y1 * scale,
                x2 * scale, y2 * scale,
                scale,
            );
        }
        PreviewAction::Scroll { dx, dy, x, y, .. } => {
            let px = x.unwrap_or(img.width() as f64 / scale / 2.0);
            let py = y.unwrap_or(img.height() as f64 / scale / 2.0);
            draw_scroll_marker(&mut img, px * scale, py * scale, *dx, *dy, scale);
        }
    }

    // Default to base64 output (most useful for LLM consumption)
    let as_base64 = output.is_none();
    output_image(&img, output, as_base64)
}
