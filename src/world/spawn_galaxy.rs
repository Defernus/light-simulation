use glam::DVec3;

use crate::{photons::wavelength::WaveLength, star::Star};

pub fn spawn_galaxy(
    stars: &mut Vec<Star>,
    center: DVec3,
    top: DVec3,
    radius: f64,
    thickness: f64,
    size: usize,
    mass_range: (f64, f64),
) {
    let z = top.normalize();
    let x = top.cross(top + DVec3::ONE).normalize();
    let y = top.cross(x).normalize();

    for i in 0..size {
        let angle = i as f64 * 2.0 * std::f64::consts::PI / size as f64;

        let r: f64 = rand::random();
        // let r = 1.0 - r * r;
        let r = r * radius;

        let position = center
            + x * r * angle.cos()
            + y * r * angle.sin()
            + z * rand::random::<f64>() * thickness;

        let star = Star {
            pos: center + position,
            mass: rand::random::<f64>() * (mass_range.1 - mass_range.0) + mass_range.0,
            photons_wavelength: WaveLength(rand::random::<f64>() * 370.0 + 380.0),
            ..Default::default()
        };
        stars.push(star);
    }
}
