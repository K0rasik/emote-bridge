use image::imageops::FilterType;
use image::{DynamicImage, RgbaImage};
use webp_animation::{Decoder, Encoder, EncodingConfig};
use rayon::prelude::*;

pub struct WebpProcessor {
    frames: Vec<(DynamicImage, i32)>,
    width: u32,
    height: u32,
}

impl WebpProcessor {
    /// Декодирует WEBP файл и возвращает экземпляр WebpProcessor.
    pub fn decode(input_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::read(input_path)?;
        let decoder = Decoder::new(&file)?;
        let dimensions = decoder.dimensions();

        let frames: Vec<_> = decoder
            .into_iter()
            .map(|frame| {
                let timestamp = frame.timestamp();
                let data = frame.data().to_vec();
                let rgba = RgbaImage::from_raw(dimensions.0, dimensions.1, data).unwrap();
                let img = DynamicImage::ImageRgba8(rgba);
                (img, timestamp)
            })
            .collect();

        Ok(WebpProcessor {
            frames,
            width: dimensions.0,
            height: dimensions.1,
        })
    }

    /// Апскейлит кадры до нового размера.
    /// Новые размеры рассчитываются автоматически, сохраняя пропорции и ограничивая большую сторону до 512 пикселей.
    pub fn upscale(&mut self) {
        let (new_width, new_height) = if self.width > self.height {
            let scale = 512.0 / self.width as f32;
            (512, (self.height as f32 * scale) as u32)
        } else {
            let scale = 512.0 / self.height as f32;
            ((self.width as f32 * scale) as u32, 512)
        };

        self.frames = self
            .frames
            .par_iter()
            .map(|(img, timestamp)| {
                let upscaled = img.resize(new_width, new_height, FilterType::CatmullRom);
                (upscaled, *timestamp)
            })
            .collect();

        self.width = new_width;
        self.height = new_height;
    }

    /// Кодирует апскейленные кадры в WEBP.
    pub fn encode(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let config = EncodingConfig::new_lossy(50.0);
        let mut encoder = Encoder::new((self.width, self.height)).unwrap();

        let mut sorted_frames = self.frames.clone();
        sorted_frames.sort_by_key(|&(_, timestamp)| timestamp);
        let final_timestamp = sorted_frames.last().map(|(_, ts)| *ts).unwrap_or(0);

        for (frame_data, timestamp) in sorted_frames {
            encoder.add_frame_with_config(&frame_data.as_bytes(), timestamp, &config)?;
        }

        let webp_data = encoder.finalize(final_timestamp)?;
        Ok(webp_data.to_vec())
    }
}