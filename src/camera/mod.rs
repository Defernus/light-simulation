use glam::{DVec2, DVec3, Vec3Swizzles};

use crate::photons::Photon;

/// ### Camera obscura.
/// Camera's sensor - is a rectangle, belongs to XY plane faced to -Z direction.
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// pow 2 of Camera's hole radius. The larger the hole, the more light will pass through, but the less sharp the image will be.
    pub hole_radius: f64,

    /// distance between camera's sensor and a hole
    pub focal_length: f64,

    /// Size of the sensor rectangle.
    pub sensor_size: DVec2,
}

impl Camera {
    /// Return intersection between camera's sensor and a ray segment (if no intersection - return None) and accuracy (squared distance between center of the hole and hole intersection point).
    /// The segment must belong to a straight line passing through hole (if it is not - return None).  
    pub fn get_intersection(&self, photon: Photon) -> Option<(DVec2, f64)> {
        let pos = photon.get_position();
        let dir = photon.get_direction();

        if pos.z.is_sign_positive() || (pos.z + dir.z).is_sign_negative() {
            return None;
        }

        let factor = -pos.z;
        let sensor_overlap_position = pos + dir * factor;

        let uv = sensor_overlap_position.xy() / self.sensor_size / 2.;

        // check if overlap point is belongs to sensor rectangle
        if uv.x.abs() > 0.5 || uv.y.abs() > 0.5 {
            return None;
        }

        // shift all vectors so hole will be at Z = 0
        let pos = pos + DVec3::Z * self.focal_length;
        let hole_overlap_position = pos - dir * (self.focal_length - factor);
        let hole_overlap_uv = (hole_overlap_position / self.hole_radius).xy();

        let dist_sq = hole_overlap_uv.length_squared();
        if dist_sq > 1. {
            return None;
        }

        return Some((uv + 0.5, dist_sq));
    }
}
