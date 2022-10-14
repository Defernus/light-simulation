use glam::DVec2;
use image::RgbImage;

const PIXEL_SIZE: usize = 3;

type Color = (u8, u8, u8);

pub struct Canvas {
    img: RgbImage,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            img: RgbImage::new(width, height),
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 3]) {
        self.img.get_pixel_mut(x, y).0 = color;
    }

    pub fn set_pixel_by_uv(&mut self, uv: DVec2, color: [u8; 3]) {
        let x = (uv.x * self.img.width() as f64) as u32;
        let y = (uv.y * self.img.height() as f64) as u32;
        self.set_pixel(x, y, color);
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 3] {
        self.img.get_pixel(x, y).0
    }

    pub fn save(&self, path: &str) {
        self.img.save(path).unwrap();
    }
}
