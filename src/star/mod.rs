use crate::{
    config::CONFIG,
    photons::{wavelength::WaveLength, Photon},
};
use glam::DVec3;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Star {
    pub pos: DVec3,
    pub vel: DVec3,
    pub mass: f64,
    pub photons_wavelength: WaveLength,

    /// how much photons are emitted per iteration
    pub luminosity: f64,
}

impl Star {
    pub fn get_photons_per_frame(&self) -> usize {
        (self.luminosity * (CONFIG.photons_spawn_rate as f64)) as usize
    }
    pub fn spawn_photons(&self, photons: &mut Vec<Photon>) {
        let spawn_count = self.get_photons_per_frame();

        for _ in 0..spawn_count {
            let theta = (rand::random::<f64>() - 0.5) * 2.0 * PI;
            let phi = (rand::random::<f64>() - 0.5) * 2.0 * PI;

            let direction = DVec3::new(theta.cos() * phi.cos(), phi.sin(), theta.sin() * phi.cos());

            let photon = Photon::new(self.photons_wavelength, self.pos, direction);

            photons.push(photon);
        }
    }
}

impl Default for Star {
    fn default() -> Self {
        Self {
            pos: DVec3::ZERO,
            vel: DVec3::ZERO,
            mass: 1.0,
            photons_wavelength: WaveLength::default(),
            luminosity: 1.0,
        }
    }
}
