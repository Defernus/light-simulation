use glam::{DVec2, Vec3Swizzles};

use crate::photons::Photon;

/// ### Camera obscura.
/// Camera's sensor - is a rectangle, belongs to XY plane faced to -Z direction.
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// Camera's hole width. Hole have same ratio as sensor. The larger the hole, the more light will pass through, but the less sharp the image will be.
    pub hole_width: f64,

    /// distance between camera's sensor and a hole
    pub focal_length: f64,

    /// Size of the sensor rectangle.
    pub sensor_size: DVec2,
}

impl Camera {
    /// Return intersection between camera's sensor and a ray segment (if no intersection - return None).
    /// The segment must belong to a straight line passing through hole (if it is not - return None).
    pub fn get_intersection(&self, photon: Photon) -> Option<DVec2> {
        let pos = photon.get_position();
        let dir = photon.get_direction();

        if pos.z.is_sign_positive() || (pos.z + dir.z).is_sign_negative() {
            return None;
        }

        let factor = pos.z / dir.z;
        let overlap_position = pos + dir * factor;

        let uv = overlap_position.xy() / self.sensor_size / 2.;

        // check if overlap point is belongs to sensor rectangle
        if uv.x.abs() > 0.5 || uv.y.abs() > 0.5 {
            return None;
        }

        return Some(uv);
    }
}
