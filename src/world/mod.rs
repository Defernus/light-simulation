use crate::{camera::Camera, canvas::Canvas, config::CONFIG, photons::Photon, star::Star};
use glam::DVec3;
use rayon::prelude::*;
use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
};

pub mod spawn_galaxy;

pub struct World {
    /// Represent all photons for each frame
    photon_groups: LinkedList<Vec<Photon>>,
    stars: Vec<Star>,
}

impl World {
    pub fn new() -> World {
        let mut stars = vec![];

        // spawn_galaxy::spawn_galaxy(&mut stars, DVec3::new(0., 0., -4.), DVec3::Y, 4.0, 1.0, 10);
        stars.push(Star {
            position: DVec3::new(3., 0., -4.),
            ..Default::default()
        });

        World {
            photon_groups: LinkedList::new(),
            stars,
        }
    }

    pub fn process(&mut self, camera: Camera, canvas: &mut Canvas) {
        let total_photons_count = self
            .stars
            .iter()
            .map(|star| star.get_photons_per_frame())
            .sum::<usize>();

        let mut frame_photons = Vec::with_capacity(total_photons_count);

        self.stars.iter().for_each(|star| {
            star.spawn_photons(&mut frame_photons);
        });

        self.photon_groups.push_back(frame_photons);

        let canvas = Arc::new(Mutex::new(canvas));

        self.photon_groups.par_iter_mut().for_each(|photons| {
            let canvas = canvas.clone();

            for photon in photons.iter_mut() {
                if let Some((uv, factor)) = camera.get_intersection(*photon) {
                    let mut canvas = canvas.lock().unwrap();
                    canvas.update_pixel_by_uv(1. - uv, photon.get_wavelength(), 1.0 - factor);
                    continue;
                }

                photon.process();
            }
        });

        if self.photon_groups.len() > CONFIG.photons_ttl {
            self.photon_groups.pop_front();
        }
    }
}
