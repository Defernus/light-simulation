use std::f32::consts::PI;

use glam::Vec3;

use crate::{photons::wavelength::WaveLength, star::Star};

pub fn spawn_galaxy(
    stars: &mut Vec<Star>,
    center: Vec3,
    top: Vec3,
    radius: f32,
    thickness: f32,
    size: usize,
    mass_range: (f32, f32),
) {
    let z = top.normalize();
    let x = top.cross(top + Vec3::ONE).normalize();
    let y = top.cross(x).normalize();

    for i in 0..size {
        let angle = i as f32 * 2.0 * PI / size as f32;

        let r: f32 = rand::random();
        // let r = 1.0 - r * r;
        let r = r * radius;

        let position = center
            + x * r * angle.cos()
            + y * r * angle.sin()
            + z * rand::random::<f32>() * thickness;

        let star = Star {
            pos: center + position,
            mass: rand::random::<f32>() * (mass_range.1 - mass_range.0) + mass_range.0,
            photons_wavelength: WaveLength(rand::random::<f32>() * 370.0 + 380.0),
            ..Default::default()
        };
        stars.push(star);
    }
}
