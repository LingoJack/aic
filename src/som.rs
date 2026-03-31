use image::{Rgba, RgbaImage};
use serde::Serialize;

use crate::ax::{self, Frame};
use crate::error::AicError;
use crate::preview::{blend_pixel, draw_filled_rect, draw_rect};
use crate::screenshot::{capture_screen, output_image};

// High-contrast color palette for SoM bounding boxes (8 colors, cycled)
const SOM_COLORS: [Rgba<u8>; 8] = [
    Rgba([255, 0, 0, 200]),     // red
    Rgba([0, 180, 0, 200]),     // green
    Rgba([0, 100, 255, 200]),   // blue
    Rgba([255, 165, 0, 200]),   // orange
    Rgba([180, 0, 255, 200]),   // purple
    Rgba([0, 200, 200, 200]),   // cyan
    Rgba([255, 80, 180, 200]),  // pink
    Rgba([128, 128, 0, 200]),   // olive
];

// --- Embedded 5x7 bitmap font for digits 0-9 ---
// Each digit is a 5-wide x 7-tall bitmap, stored as 7 bytes (each byte = 5 bits, MSB = left)
const FONT_WIDTH: u32 = 5;
const FONT_HEIGHT: u32 = 7;

#[rustfmt::skip]
const DIGIT_BITMAPS: [[u8; 7]; 10] = [
    // 0
    [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110],
    // 1
    [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
    // 2
    [0b01110, 0b10001, 0b00001, 0b00110, 0b01000, 0b10000, 0b11111],
    // 3
    [0b01110, 0b10001, 0b00001, 0b00110, 0b00001, 0b10001, 0b01110],
    // 4
    [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
    // 5
    [0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110],
    // 6
    [0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
    // 7
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
    // 8
    [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
    // 9
    [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100],
];

/// Draw a single digit at (x, y) with given scale and color
fn draw_digit(img: &mut RgbaImage, digit: u8, x: i32, y: i32, scale: u32, color: Rgba<u8>) {
    if digit > 9 {
        return;
    }
    let bitmap = &DIGIT_BITMAPS[digit as usize];
    for row in 0..FONT_HEIGHT {
        let bits = bitmap[row as usize];
        for col in 0..FONT_WIDTH {
            if (bits >> (FONT_WIDTH - 1 - col)) & 1 == 1 {
                // Fill a scale x scale block for this pixel
                for sy in 0..scale {
                    for sx in 0..scale {
                        let px = x + (col * scale + sx) as i32;
                        let py = y + (row * scale + sy) as i32;
                        blend_pixel(img, px, py, color);
                    }
                }
            }
        }
    }
}

/// Draw a number string at (x, y)
fn draw_number(img: &mut RgbaImage, num: usize, x: i32, y: i32, scale: u32, color: Rgba<u8>) {
    let s = num.to_string();
    let mut cx = x;
    for ch in s.chars() {
        if let Some(d) = ch.to_digit(10) {
            draw_digit(img, d as u8, cx, y, scale, color);
            cx += (FONT_WIDTH * scale + scale) as i32; // 1-pixel-scaled gap between digits
        }
    }
}

/// Calculate pixel width of a number string at given scale
fn number_width(num: usize, scale: u32) -> u32 {
    let digits = num.to_string().len() as u32;
    digits * FONT_WIDTH * scale + (digits - 1) * scale // digits * char_width + gaps
}

// --- SoM index entry for JSON output ---

#[derive(Serialize)]
struct SomEntry {
    index: usize,
    role: String,
    title: Option<String>,
    frame: Option<Frame>,
    center_x: f64,
    center_y: f64,
}

// --- Public entry ---

pub fn capture_som(
    app: Option<&str>,
    output: Option<&str>,
    as_base64: bool,
) -> Result<(), AicError> {
    // 1. Get interactive elements
    let elements = ax::collect_interactive_elements(app)?;

    // 2. Capture screenshot
    let (mut img, scale) = capture_screen()?;

    // Font scale: 2 on non-Retina, 3 on Retina (or bigger for readability)
    let font_scale = if scale > 1.5 { 3u32 } else { 2u32 };
    let box_thickness = if scale > 1.5 { 3.0 } else { 2.0 };
    let label_pad: i32 = 2; // padding inside label background

    // 3. Draw annotations
    let mut som_entries: Vec<SomEntry> = Vec::with_capacity(elements.len());

    for (i, elem) in elements.iter().enumerate() {
        let index = i + 1;
        let color = SOM_COLORS[i % SOM_COLORS.len()];

        if let Some(ref frame) = elem.frame {
            // Convert logical points to pixels
            let px = frame.x * scale;
            let py = frame.y * scale;
            let pw = frame.w * scale;
            let ph = frame.h * scale;

            // Draw bounding box
            draw_rect(&mut img, px, py, pw, ph, box_thickness, color);

            // Draw label: black background + white number
            let num_w = number_width(index, font_scale) as i32;
            let num_h = (FONT_HEIGHT * font_scale) as i32;
            let label_w = num_w + label_pad * 2;
            let label_h = num_h + label_pad * 2;

            // Position label at top-left of bounding box, offset slightly inside
            let label_x = px as i32;
            let label_y = (py as i32 - label_h).max(0); // above the box, clamped

            // Black background
            let bg_color = Rgba([0, 0, 0, 220]);
            draw_filled_rect(&mut img, label_x, label_y, label_w, label_h, bg_color);

            // White number
            draw_number(
                &mut img,
                index,
                label_x + label_pad,
                label_y + label_pad,
                font_scale,
                Rgba([255, 255, 255, 255]),
            );
        }

        som_entries.push(SomEntry {
            index,
            role: elem.role.clone(),
            title: elem.title.clone(),
            frame: elem.frame.clone(),
            center_x: elem.center_x,
            center_y: elem.center_y,
        });
    }

    // 4. Output JSON index to stderr
    if let Ok(json) = serde_json::to_string(&som_entries) {
        eprintln!("{json}");
    }

    // 5. Output image
    let effective_base64 = if output.is_none() && !as_base64 {
        true // default to base64 when no output file
    } else {
        as_base64
    };
    output_image(&img, output, effective_base64)
}
