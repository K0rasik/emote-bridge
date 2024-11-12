use std::fs;

use image::imageops::FilterType;
use image::{DynamicImage, RgbaImage};
use webp_animation::{Decoder, Encoder, EncodingConfig};
use rayon::prelude::*;
fn upscale_webp(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read(input_path)?;

    let decoder = Decoder::new(&file)?;
    //let mut final_timestamp: i32 = 0;
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
    let config = EncodingConfig::new_lossy(50.0);
    let mut encoder = Encoder::new((new_width, new_height)).unwrap();

    // Собираем кадры с таймштампами в вектор
    let frames: Vec<_> = decoder.into_iter().map(|frame| {
        let timestamp = frame.timestamp();
        let data = frame.data().to_vec();
        (data, timestamp)
    }).collect();
    // Параллельно апскейлим кадры и сохраняем вектор апскейленных изображений с таймштампами
    let upscaled_frames: Vec<_> = frames
    .into_par_iter()
    .map(|(data, timestamp)| {
        let rgba = RgbaImage::from_raw(width, height, data).unwrap();
        let img = DynamicImage::ImageRgba8(rgba);
        let upscaled = img.resize(new_width, new_height, FilterType::CatmullRom);
        (upscaled, timestamp) 
    })
    .collect();

    let mut upscaled_frames = upscaled_frames;
    upscaled_frames.sort_by_key(|&(_, timestamp)| timestamp);
    let final_timestamp = upscaled_frames.last().map(|(_, ts)| *ts).unwrap_or(0);

    for (frame_data, timestamp) in upscaled_frames {
        encoder.add_frame_with_config(&frame_data.as_bytes(), timestamp, &config)?;
    }

    
    let webp_data = encoder.finalize(final_timestamp)?;

    fs::write(output_path, webp_data)?;
    Ok(())
}

fn main() {
    let input_path = "noshot.webp";
    let output_path = "5upscaled_output.webp";

    match upscale_webp(input_path, output_path) {
        Ok(_) => println!("Изображение успешно апскейлено!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
