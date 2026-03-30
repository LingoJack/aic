use std::io::Write;
use std::path::Path;

use core_graphics::display::CGDisplay;

use crate::error::AicError;

/// Take a screenshot of the main display.
pub fn take_screenshot(output: Option<&str>, as_base64: bool) -> Result<(), AicError> {
    let display = CGDisplay::main();
    let cg_image = CGDisplay::image(&display)
        .ok_or_else(|| AicError::ScreenshotFailed("CGDisplayCreateImage returned null — check Screen Recording permission".into()))?;

    let width = cg_image.width();
    let height = cg_image.height();
    let bytes_per_row = cg_image.bytes_per_row();
    let data = cg_image.data();
    let raw_bytes = data.bytes();

    // Convert BGRA to RGBA
    let mut rgba = Vec::with_capacity(width * height * 4);
    for row in 0..height {
        let row_start = row * bytes_per_row;
        for col in 0..width {
            let offset = row_start + col * 4;
            let b = raw_bytes[offset];
            let g = raw_bytes[offset + 1];
            let r = raw_bytes[offset + 2];
            let a = raw_bytes[offset + 3];
            rgba.push(r);
            rgba.push(g);
            rgba.push(b);
            rgba.push(a);
        }
    }

    let img = image::RgbaImage::from_raw(width as u32, height as u32, rgba)
        .ok_or_else(|| AicError::ImageEncodingFailed("failed to create image buffer".into()))?;

    if let Some(path) = output {
        img.save(Path::new(path))
            .map_err(|e| AicError::ImageEncodingFailed(e.to_string()))?;
        eprintln!("Screenshot saved to {path}");
    } else {
        // Encode to PNG in memory
        let mut png_bytes = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png_bytes);
        img.write_to(&mut cursor, image::ImageFormat::Png)
            .map_err(|e| AicError::ImageEncodingFailed(e.to_string()))?;

        if as_base64 {
            use base64::Engine;
            let encoded = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
            print!("{encoded}");
        } else {
            std::io::stdout()
                .write_all(&png_bytes)
                .map_err(AicError::IoError)?;
        }
    }
    Ok(())
}
