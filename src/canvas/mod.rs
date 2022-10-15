use glam::DVec2;
use image::RgbImage;

use crate::config::CONFIG;

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

    pub fn update_fading(&mut self) {
        for x in 0..self.img.width() {
            for y in 0..self.img.height() {
                let mut color = self.get_pixel(x, y);
                color[0] = (color[0] as f64 * CONFIG.fade_out_speed) as u8;
                color[1] = (color[1] as f64 * CONFIG.fade_out_speed) as u8;
                color[2] = (color[2] as f64 * CONFIG.fade_out_speed) as u8;
                self.set_pixel(x, y, color);
            }
        }
    }
}
