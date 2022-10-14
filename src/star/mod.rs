use std::{collections::LinkedList, f64::consts::PI};

use glam::DVec3;

use crate::{
    config::CONFIG,
    photons::{wavelength::WaveLength, Photon},
};

pub struct Star {
    pub position: DVec3,
    pub mass: f64,
    pub photons_wavelength: WaveLength,

    /// how much photons are emitted per iteration
    pub luminosity: f64,
}

impl Star {
    pub fn spawn_photons(&self, photons: &mut LinkedList<Photon>) {
        let spawn_count = (self.luminosity * CONFIG.photons_spawn_rate as f64) as usize;

        for _ in 0..spawn_count {
            let theta = (rand::random::<f64>() - 0.5) * 2.0 * PI;
            let phi = (rand::random::<f64>() - 0.5) * 2.0 * PI;

            let direction = DVec3::new(theta.cos() * phi.cos(), phi.sin(), theta.sin() * phi.cos());

            let photon = Photon::new(self.photons_wavelength, self.position, direction);

            photons.push_back(photon);
        }
    }
}
