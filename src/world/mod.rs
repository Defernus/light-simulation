use crate::{
    camera::Camera, canvas::Canvas, config::CONFIG, object::Object,
    physics_constants::get_gravity_acceleration,
};
use futures::executor::block_on;
use glam::Vec3;
use rayon::prelude::*;
use std::{
    collections::LinkedList,
    sync::{Arc, Mutex},
};

use self::light_processing::LightProcessor;

mod light_processing;
mod spawn_galaxy;

pub struct World {
    /// Represent all photons for each frame
    light_groups: LinkedList<LightProcessor>,
    stars: Vec<Object>,
}

impl World {
    pub fn new() -> World {
        let mut stars = vec![];

        // spawn_galaxy::spawn_galaxy(
        //     &mut stars,
        //     Vec3::new(0., 0., -4.),
        //     // Vec3::new(1.0, 3.0, 2.0),
        //     Vec3::Z,
        //     1.0,
        //     0.01,
        //     100,
        //     (0.8, 2000.0),
        // );

        stars.push(Object {
            pos: Vec3::new(0., 0., -4.),
            ..Default::default()
        });

        World {
            light_groups: LinkedList::new(),
            stars,
        }
    }

    pub fn update_light(&mut self, camera: Camera, canvas: &mut Canvas) {
        let total_photons_count = self
            .stars
            .iter()
            .map(|star| star.get_photons_per_frame())
            .sum::<usize>();

        let mut frame_photons = Vec::with_capacity(total_photons_count);

        self.stars.iter().for_each(|star| {
            star.spawn_photons(&mut frame_photons);
        });

        self.light_groups
            .push_back(block_on(async { LightProcessor::new(frame_photons).await }));

        let canvas = Arc::new(Mutex::new(canvas));

        self.light_groups
            .par_iter_mut()
            .for_each(|light_processor| {
                block_on(async {
                    let canvas = canvas.clone();
                    light_processor
                        .process_light_for_group(camera, canvas)
                        .await;
                });
            });

        if self.light_groups.len() > CONFIG.photons_ttl {
            self.light_groups.pop_front();
        }
    }

    pub fn update_movement(&mut self) {
        self.stars = self
            .stars
            .iter()
            .enumerate()
            .map(|(i, star)| {
                let mut a = Vec3::ZERO;

                for (j, other_star) in self.stars.iter().enumerate() {
                    if i == j {
                        continue;
                    }

                    let delta = other_star.pos - star.pos;

                    a += delta.normalize()
                        * get_gravity_acceleration(other_star.mass, delta.length_squared());
                }

                Object {
                    pos: star.pos + star.vel,
                    vel: star.vel + a,
                    ..*star
                }
            })
            .collect();
    }
}
