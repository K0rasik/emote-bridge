// src/main.rs

mod webp_processor;
mod avif_processor;
mod avif_decoder;

use std::env;
use std::fs;
use webp_processor::WebpProcessor;
use avif_processor::AvifProcessor;
use avif_decoder::decode_avif;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <input_path> <output_path> <format: webp|avif>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];
    let format = &args[3];

    match format.as_str() {
        "webp" => {
            match WebpProcessor::decode(input_path) {
                Ok(mut processor) => {
                    processor.upscale();
                    println!("Собрали и заапскейлили");

                    match processor.encode() {
                        Ok(webp_data) => {
                            if let Err(e) = fs::write(output_path, webp_data) {
                                eprintln!("Failed to save file: {}", e);
                            } else {
                                println!("Изображение успешно апскейлено и сохранено в WEBP!");
                            }
                        }
                        Err(e) => eprintln!("Error during WEBP encoding: {}", e),
                    }
                }
                Err(e) => eprintln!("Error during WEBP decoding: {}", e),
            }
        }
        "avif" => {
            match decode_avif(input_path) {
                Ok((frames, (width, height))) => {
                    let mut avif_processor = AvifProcessor::new(frames, width, height);
                    avif_processor.upscale();
                    println!("Собрали и заапскейлили");

                    match avif_processor.encode() {
                        Ok(avif_data) => {
                            if let Err(e) = fs::write(output_path, avif_data) {
                                eprintln!("Failed to save file: {}", e);
                            } else {
                                println!("Изображение успешно апскейлено и сохранено в AVIF!");
                            }
                        }
                        Err(e) => eprintln!("Error during AVIF encoding: {}", e),
                    }
                }
                Err(e) => eprintln!("Error during AVIF decoding: {}", e),
            }
        }
        _ => eprintln!("Unsupported format: {}", format),
    }
}