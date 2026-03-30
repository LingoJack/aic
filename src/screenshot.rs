use std::io::Write;
use std::path::Path;

use core_graphics::display::CGDisplay;
use image::RgbaImage;

use crate::error::AicError;

/// Capture the main display and return the image with the Retina scale factor.
/// Scale factor = pixel width / point width (1.0 on non-Retina, 2.0 on Retina).
pub fn capture_screen() -> Result<(RgbaImage, f64), AicError> {
    let display = CGDisplay::main();
    let bounds = display.bounds();

    let cg_image = CGDisplay::image(&display).ok_or_else(|| {
        AicError::ScreenshotFailed(
            "CGDisplayCreateImage returned null — check Screen Recording permission".into(),
        )
    })?;

    let width = cg_image.width();
    let height = cg_image.height();
    let bytes_per_row = cg_image.bytes_per_row();
    let data = cg_image.data();
    let raw_bytes = data.bytes();

    let scale = width as f64 / bounds.size.width;

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

    let img = RgbaImage::from_raw(width as u32, height as u32, rgba)
        .ok_or_else(|| AicError::ImageEncodingFailed("failed to create image buffer".into()))?;

    Ok((img, scale))
}

/// Output an RgbaImage to file, base64, or raw stdout.
pub fn output_image(img: &RgbaImage, output: Option<&str>, as_base64: bool) -> Result<(), AicError> {
    if let Some(path) = output {
        img.save(Path::new(path))
            .map_err(|e| AicError::ImageEncodingFailed(e.to_string()))?;
        eprintln!("Saved to {path}");
    } else {
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

/// Take a screenshot of the main display.
pub fn take_screenshot(output: Option<&str>, as_base64: bool) -> Result<(), AicError> {
    let (img, _) = capture_screen()?;
    output_image(&img, output, as_base64)
}
