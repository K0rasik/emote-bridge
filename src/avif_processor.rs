// src/avif_processor.rs

use image::DynamicImage;
use ravif::{Encoder, Img, RGBA8};
use std::error::Error;

pub struct AvifProcessor {
    frames: Vec<(DynamicImage, i32)>,
    width: u32,
    height: u32,
}

impl AvifProcessor {
    /// Создает новый экземпляр AvifProcessor из декодированных данных.
    pub fn new(frames: Vec<(DynamicImage, i32)>, width: u32, height: u32) -> Self {
        AvifProcessor {
            frames,
            width,
            height,
        }
    }

    /// Апскейлит кадры до нового размера.
    pub fn upscale(&mut self) {
        let (new_width, new_height) = self.calculate_new_dimensions();

        self.frames = self
            .frames
            .iter()
            .map(|(img, timestamp)| {
                let upscaled = img.resize(new_width, new_height, image::imageops::FilterType::CatmullRom);
                (upscaled, *timestamp)
            })
            .collect();

        self.width = new_width;
        self.height = new_height;
    }

    /// Кодирует апскейленные кадры в AVIF.
    pub fn encode(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut result = Vec::new();

        for (frame, _) in &self.frames {
            // Преобразуем изображение в RGBA8
            let rgba = frame.to_rgba8();
            let pixels: Vec<RGBA8> = rgba
                .pixels()
                .map(|p| RGBA8::new(p[0], p[1], p[2], p[3]))
                .collect();

            // Создаем Img<&[RGBA8]> для ravif
            let img = Img::new(pixels.as_slice(), self.width.try_into().unwrap(), self.height.try_into().unwrap());
            //let img = RgbaImage::new(self.width.try_into().unwrap(), self.height.try_into().unwrap());
            // Настройки кодирования
            let encoder = Encoder::new()
                .with_quality(80.0)
                .with_alpha_quality(80.0)
                .with_speed(5);

            // Кодируем в AVIF
            let encoded = encoder.encode_rgba(img)?;
            result.extend_from_slice(&encoded.avif_file);
        }

        Ok(result)
}

    /// Вспомогательный метод для расчета новых размеров.
    fn calculate_new_dimensions(&self) -> (u32, u32) {
        if self.width > self.height {
            let scale = 512.0 / self.width as f32;
            (512, (self.height as f32 * scale) as u32)
        } else {
            let scale = 512.0 / self.height as f32;
            ((self.width as f32 * scale) as u32, 512)
        }
    }
}