use std::{collections::LinkedList, f64::consts::PI};

use glam::DVec3;

use crate::{
    config::CONFIG,
    photons::{wavelength::WaveLength, Photon},
};

#[derive(Debug, Clone, Copy)]
pub struct Star {
    pub position: DVec3,
    pub speed: DVec3,
    pub mass: f64,
    pub photons_wavelength: WaveLength,

    /// how much photons are emitted per iteration
    pub luminosity: f64,
}

impl Star {
    pub fn spawn_photons(&self, photon_threads: &mut Vec<LinkedList<Photon>>) {
        let spawn_count = (self.luminosity * (CONFIG.photons_spawn_rate as f64)
            / (photon_threads.len() as f64)) as usize;

        for photon_thread in photon_threads {
            for _ in 0..spawn_count {
                let theta = (rand::random::<f64>() - 0.5) * 2.0 * PI;
                let phi = (rand::random::<f64>() - 0.5) * 2.0 * PI;

                let direction =
                    DVec3::new(theta.cos() * phi.cos(), phi.sin(), theta.sin() * phi.cos());

                let photon = Photon::new(self.photons_wavelength, self.position, direction);

                photon_thread.push_back(photon);
            }
        }
    }
}

impl Default for Star {
    fn default() -> Self {
        Self {
            position: DVec3::ZERO,
            speed: DVec3::ZERO,
            mass: 1.0,
            photons_wavelength: WaveLength::default(),
            luminosity: 1.0,
        }
    }
}
