use crate::{camera::Camera, canvas::Canvas, photons::Photon, star::Star};
use rayon::prelude::*;
use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
};

const THREADS_COUNT: usize = 8;

pub struct World {
    photon_threads: Vec<LinkedList<Photon>>,
    stars: Vec<Star>,
}

impl World {
    pub fn new() -> World {
        World {
            photon_threads: vec![LinkedList::new(); THREADS_COUNT],
            stars: vec![
                Star {
                    position: glam::DVec3::new(3.0, 2.0, -20.0),
                    ..Default::default()
                },
                Star {
                    position: glam::DVec3::new(-5.0, -1.0, -10.0),
                    ..Default::default()
                },
            ],
        }
    }

    pub fn process(&mut self, camera: Camera, canvas: &mut Canvas) {
        self.stars.iter().for_each(|star| {
            star.spawn_photons(&mut self.photon_threads);
        });

        let canvas = Arc::new(Mutex::new(canvas));

        self.photon_threads.par_iter_mut().for_each(|photons| {
            let mut new_photons = LinkedList::new();

            let canvas = canvas.clone();
            let mut canvas = canvas.lock().unwrap();

            while let Some(mut photon) = photons.pop_back() {
                if let Some(uv) = camera.get_intersection(photon) {
                    canvas.set_pixel_by_uv(uv, photon.into());
                    continue;
                }
                if photon.process() {
                    new_photons.push_front(photon);
                }
            }

            *photons = new_photons
        });
    }
}
