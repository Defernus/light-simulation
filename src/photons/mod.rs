use self::wavelength::WaveLength;
use glam::DVec3;

pub mod wavelength;

#[derive(Debug, Clone, Copy)]
pub struct Photon {
    wavelength: WaveLength,
    position: DVec3,
    direction: DVec3,
}

impl Photon {
    pub fn new(wavelength: wavelength::WaveLength, position: DVec3, direction: DVec3) -> Photon {
        Photon {
            wavelength,
            position,
            direction: direction.normalize(),
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

    pub fn process(&mut self) {
        self.position += self.direction;
    }
}
