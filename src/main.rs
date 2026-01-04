use image::io::Reader as ImageReader;
use minifb::{Key, Scale, Window, WindowOptions};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args()
        .nth(1)
        .expect("Использование: imgview <путь_к_изображению>");

    let img = ImageReader::open(path)?.decode()?.to_rgba8();

    let (width, height) = img.dimensions();

    // RGBA → u32 (0x00RRGGBB)
    let buffer: Vec<u32> = img
        .chunks_exact(4)
        .map(|p| ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | p[2] as u32)
        .collect();

    let mut window = Window::new(
        "Image Viewer",
        width as usize,
        height as usize,
        WindowOptions {
            scale: Scale::X2, // 🔹 СКЕЙЛИНГ ОКНА
            resize: true,
            ..WindowOptions::default()
        },
    )?;

    // Ограничение FPS
    window.limit_update_rate(Some(std::time::Duration::from_micros(16_600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width as usize, height as usize)?;
    }

    Ok(())
}

