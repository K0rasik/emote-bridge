// src/avif_decoder.rs

use image::{DynamicImage, GenericImageView};
use std::error::Error;
use std::fs;

/// Декодирует AVIF файл и возвращает кадры с их временными метками.
/// Примечание: AVIF не поддерживает анимацию в библиотеке `image`, поэтому мы работаем с одним кадром.
pub fn decode_avif(input_path: &str) -> Result<(Vec<(DynamicImage, i32)>, (u32, u32)), Box<dyn Error>> {
    // Читаем файл
    let file_data = fs::read(input_path)?;

    // Декодируем AVIF с помощью библиотеки `image`
    let img = image::load_from_memory(&file_data)?;
    let dimensions = img.dimensions();

    // Создаем один кадр с временной меткой 0
    let frames = vec![(img, 0)];

    Ok((frames, dimensions))
}