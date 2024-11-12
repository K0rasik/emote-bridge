use std::fs;

use image::imageops::FilterType;
use image::{DynamicImage, RgbaImage};
use webp_animation::{Decoder, Encoder, EncodingConfig};
use rayon::prelude::*;
fn upscale_webp(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read(input_path)?;

    let decoder = Decoder::new(&file)?;
    let mut final_timestamp: i32 = 0;
    let width;
    let height;
    (width, height) = decoder.dimensions();
    // Вычисляем новый размер (512 по большей стороне, сохраняя пропорции)
    let (new_width, new_height) = if width > height {
        let scale = 512.0 / width as f32;
        (512, (height as f32 * scale) as u32)
    } else {
        let scale = 512.0 / height as f32;
        ((width as f32 * scale) as u32, 512)
    };
    let config = EncodingConfig::new_lossy(30.0);
    let mut encoder = Encoder::new((new_width, new_height)).unwrap();

    for frame in decoder {
        final_timestamp = frame.timestamp();
        let rgba = RgbaImage::from_raw(width, height, frame.data().to_vec()).unwrap();
        let img = DynamicImage::ImageRgba8(rgba);
        let upscaled = img.resize(new_width, new_height, FilterType::Triangle);
        encoder.add_frame_with_config(upscaled.as_bytes(), frame.timestamp(), &config)?;
    }
    let webp_data = encoder.finalize(final_timestamp)?;
    fs::write(output_path, webp_data)?;
    Ok(())
}

fn main() {
    let input_path = "input.webp";
    let output_path = "3upscaled_output.webp";

    match upscale_webp(input_path, output_path) {
        Ok(_) => println!("Изображение успешно апскейлено!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
