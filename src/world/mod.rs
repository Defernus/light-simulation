use std::collections::LinkedList;

use crate::{
    camera::Camera,
    canvas::Canvas,
    photons::{wavelength::WaveLength, Photon},
    star::Star,
};

pub struct World {
    photons: LinkedList<Photon>,
    stars: Vec<Star>,
}

impl World {
    pub fn new() -> World {
        World {
            photons: LinkedList::new(),
            stars: vec![
                Star {
                    position: glam::DVec3::new(0.0, 1.0, -10.0),
                    mass: 1.0,
                    luminosity: 1.0,
                    photons_wavelength: WaveLength::WHITE,
                },
                Star {
                    position: glam::DVec3::new(1.0, -3.0, -7.0),
                    mass: 1.0,
                    luminosity: 1.0,
                    photons_wavelength: WaveLength::WHITE,
                },
            ],
        }
    }

    pub fn process(&mut self, camera: Camera, canvas: &mut Canvas) {
        self.stars.iter().for_each(|star| {
            star.spawn_photons(&mut self.photons);
        });

        let mut new_photons = LinkedList::new();
        while let Some(mut photon) = self.photons.pop_back() {
            if let Some(uv) = camera.get_intersection(photon) {
                canvas.set_pixel_by_uv(uv, photon.into());
                continue;
            }
            if !photon.process() {
                new_photons.push_front(photon);
            }
        }

        self.photons = new_photons;
    }
}
