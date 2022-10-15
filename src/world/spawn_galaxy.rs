use glam::DVec3;

use crate::star::Star;

pub fn spawn_galaxy(
    stars: &mut Vec<Star>,
    center: DVec3,
    top: DVec3,
    radius: f64,
    thickness: f64,
    size: usize,
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
            position: center + position,
            ..Default::default()
        };
        stars.push(star);
    }
}
