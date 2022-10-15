use std::path::Path;

use glam::DVec2;
use image::{ImageBuffer, LumaA, RgbImage};
use show_image::{create_window, error::SetImageError, ImageInfo, ImageView, WindowProxy};

use crate::{config::CONFIG, photons::wavelength::WaveLength};

pub struct Canvas {
    img: ImageBuffer<LumaA<f64>, Vec<f64>>,
    window: WindowProxy,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        let window = create_window("image", Default::default()).expect("Window created");

        Canvas {
            img: ImageBuffer::new(width, height),
            window,
        }
    }

    pub fn update_pixel(&mut self, x: u32, y: u32, wave_length: WaveLength, luminosity: f64) {
        let val = &mut self.img.get_pixel_mut(x, y).0;
        val[0] = (val[0] * val[1] + wave_length.0 * luminosity) / (val[1] + luminosity);
        val[1] += luminosity;
    }

    pub fn update_pixel_by_uv(&mut self, uv: DVec2, wave_length: WaveLength, luminosity: f64) {
        let x = (uv.x * self.img.width() as f64) as u32;
        let y = (uv.y * self.img.height() as f64) as u32;
        self.update_pixel(x, y, wave_length, luminosity);
    }

    pub fn generate_rgb(&self) -> RgbImage {
        let mut rgb_img = RgbImage::new(self.img.width(), self.img.height());

        for (x, y, pixel) in self.img.enumerate_pixels() {
            // !TODO add color from wave length
            let color = (pixel.0[1] * 255.0) as u8;
            rgb_img.put_pixel(x, y, image::Rgb([color, color, color]));
        }

        rgb_img
    }

    pub fn save<T>(&self, path: T)
    where
        T: AsRef<Path>,
    {
        self.generate_rgb().save(path).expect("Image saved");
    }

    pub fn update_fading(&mut self) {
        for x in 0..self.img.width() {
            for y in 0..self.img.height() {
                let mut color = self.img.get_pixel_mut(x, y);
                color.0[1] = color[1] as f64 * CONFIG.fade_out_speed;
            }
        }
    }

    pub fn show(&mut self) -> Result<(), SetImageError> {
        let rgb_data = &self.generate_rgb();
        let rgb = ImageView::new(
            ImageInfo::rgb8(self.img.width(), self.img.height()),
            rgb_data,
        );
        self.window.set_image("frame-001", rgb)?;

        Ok(())
    }
}
