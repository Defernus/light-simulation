use glam::DVec3;

use crate::config::CONFIG;

use self::wavelength::WaveLength;

pub mod wavelength;

#[derive(Debug, Clone, Copy)]
pub struct Photon {
    wavelength: WaveLength,
    position: DVec3,
    direction: DVec3,
    ttl: usize,
}

impl Into<[u8; 3]> for Photon {
    fn into(self) -> [u8; 3] {
        self.wavelength.into()
    }
}

impl Into<DVec3> for Photon {
    fn into(self) -> DVec3 {
        self.wavelength.into()
    }
}

impl Into<WaveLength> for Photon {
    fn into(self) -> WaveLength {
        self.wavelength
    }
}

impl Photon {
    pub fn new(wavelength: wavelength::WaveLength, position: DVec3, direction: DVec3) -> Photon {
        Photon {
            wavelength,
            position,
            direction,
            ttl: CONFIG.photons_ttl,
        }
    }

    pub fn get_wavelength(&self) -> wavelength::WaveLength {
        self.wavelength
    }

    pub fn get_position(&self) -> DVec3 {
        self.position
    }

    pub fn get_direction(&self) -> DVec3 {
        self.direction
    }

    pub fn set_wavelength(&mut self, wavelength: wavelength::WaveLength) {
        self.wavelength = wavelength;
    }

    /// Process photon's movement and return if it should be kept
    pub fn process(&mut self) -> bool {
        self.position += self.direction;
        self.ttl -= 1;

        self.ttl > 0
    }
}
