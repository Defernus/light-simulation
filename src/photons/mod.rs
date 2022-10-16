use self::wavelength::WaveLength;
use bytemuck::{Pod, Zeroable};
use glam::Vec3;

pub mod wavelength;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Photon {
    pub pos: [f32; 4],
    pub dir: [f32; 4],
    pub wavelength: WaveLength,
    pub _pad: [f32; 3],
}

impl Photon {
    pub fn new(wavelength: wavelength::WaveLength, pos: Vec3, dir: Vec3) -> Photon {
        Photon {
            wavelength,
            pos: [pos.x, pos.y, pos.z, 0.0],
            dir: [dir.x, dir.y, dir.z, 0.0],
            _pad: [0.0, 0.0, 0.0],
        }
    }

    pub fn get_wavelength(&self) -> wavelength::WaveLength {
        self.wavelength
    }

    pub fn get_position(&self) -> Vec3 {
        Vec3::new(self.pos[0], self.pos[1], self.pos[2])
    }

    pub fn get_direction(&self) -> Vec3 {
        Vec3::new(self.dir[0], self.dir[1], self.dir[2])
    }

    pub fn set_wavelength(&mut self, wavelength: wavelength::WaveLength) {
        self.wavelength = wavelength;
    }

    pub fn process(&self) -> Self {
        Photon {
            pos: [
                self.pos[0] + self.dir[0],
                self.pos[1] + self.dir[1],
                self.pos[2] + self.dir[2],
                0.0,
            ],
            dir: self.dir,
            wavelength: self.wavelength,
            _pad: [0.0, 0.0, 0.0],
        }
    }
}
