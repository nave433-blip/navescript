use image::{DynamicImage, imageops};
use std::io::Cursor;

pub struct ImageHost {
    img: DynamicImage,
}

impl ImageHost {
    pub fn new(data: &[u8]) -> Result<Self, String> {
        let img = image::load_from_memory(data).map_err(|e| e.to_string())?;
        Ok(ImageHost { img })
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), String> {
        self.img = self.img.resize_exact(width, height, imageops::FilterType::Lanczos3);
        Ok(())
    }

    pub fn rotate(&mut self, degrees: f32) -> Result<(), String> {
        // Simplified rotation logic
        self.img = match degrees as i32 {
            90 => self.img.rotate90(),
            180 => self.img.rotate180(),
            270 => self.img.rotate270(),
            _ => self.img.clone(),
        };
        Ok(())
    }

    pub fn get_data(&self) -> Vec<u8> {
        let mut buffer = Cursor::new(Vec::new());
        self.img.write_to(&mut buffer, image::ImageFormat::Png).unwrap();
        buffer.into_inner()
    }
}
