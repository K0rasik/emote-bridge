// avif_decoder.rs
extern "C" {
    fn decode_avif(avif_data: *const u8, avif_size: usize, frames: *mut *mut AvifFrame, frame_count: *mut usize) -> i32;
    fn free_avif_result(frames: *mut AvifFrame, frame_count: usize);
}

use std::slice;
use std::ptr;
use std::alloc::{alloc, Layout};

#[repr(C)]
struct AvifFrame {
    data: *mut u8,
    width: i32,
    height: i32,
    duration_ms: i32,
}

impl Clone for AvifFrame {
    fn clone(&self) -> Self {
        let size = (self.width * self.height * 4) as usize;
        let layout = Layout::array::<u8>(size).unwrap();
        
        // Выделяем новую область памяти для данных
        let cloned_data = unsafe {
            let ptr = alloc(layout);
            if ptr.is_null() {
                panic!("Failed to allocate memory for AvifFrame clone");
            }
            ptr::copy_nonoverlapping(self.data, ptr, size);
            ptr
        };

        AvifFrame {
            data: cloned_data,
            width: self.width,
            height: self.height,
            duration_ms: self.duration_ms,
        }
    }
}

pub struct AvifDecoder {
    frames: Vec<Vec<u8>>,
}

impl AvifDecoder {
    pub fn new(filename: &str) -> Result<Self, String> {
        let mut frames_ptr: *mut AvifFrame = ptr::null_mut();
        let mut frame_count: usize = 0;

        // Читаем файл в байты
        let data = std::fs::read(filename).map_err(|_| "Failed to read file")?;
        let result = unsafe { 
            decode_avif(data.as_ptr(), data.len(), &mut frames_ptr as *mut _, &mut frame_count as *mut _)
        };
        
        if result != 0 || frames_ptr.is_null() {
            return Err("Failed to decode AVIF".to_string());
        }

        let mut frames = Vec::new();

        unsafe {
            for i in 0..frame_count {
                let frame = (*frames_ptr.add(i)).clone();
                if !frame.data.is_null() {
                    let size = (frame.width * frame.height * 4) as usize;
                    let slice = slice::from_raw_parts(frame.data, size);
                    let frame_data = slice.to_vec(); // Копируем данные в новый вектор
                    frames.push(frame_data); // Пушим новый вектор в вектор кадров
                }
            }
            free_avif_result(frames_ptr, frame_count);
        }

        Ok(AvifDecoder { frames })
    }

    pub fn get_frames(&self) -> &Vec<Vec<u8>> {
        &self.frames
    }
}