/// speed of photons
pub const C_SI: f64 = 299792458.0;

pub const TIME_SPEED: f32 = 1.0;

/// time between each frame in seconds
pub const TIME_SI: f64 = 365. * 24. * 60. * 60.;
const TIME_SI_2: f64 = TIME_SI * TIME_SI;

/// distance which each photon will travel in one frame
pub const DIST_SI: f64 = C_SI * TIME_SI;
const DIST_SI_3: f64 = DIST_SI * DIST_SI * DIST_SI;

/// mass of sun in kg
pub const MASS_SI: f64 = 1.989E30;
pub const MASS_SI_2: f64 = MASS_SI * MASS_SI;

pub const GRAVITY_CONSTANT_SI: f64 = 6.67408e-11;

/// Gravitational constant in game units
///
/// F_SI = G * m1 * MASS_SI * m2 * MASS_SI / (r * DIST_SI * r * DIST_SI)  <br>
/// F_SI = G * m1 * m2 / (r * r) * MASS_SI_2 / DIST_SI_2  <br>
///
/// A_SI = F_SI / m1 * MASS_SI  <br>
/// A_SI = (G * m1 * m2 / (r * r) * MASS_SI_2 / DIST_SI_2) / m1 * MASS_SI  <br>
/// A_SI = G * m2 * MASS_SI / (r * r * DIST_SI_2)  <br>
///
/// A = A_SI * TIME_SI * TIME_SI / DIST_SI  <br>
/// A = G * m2 * MASS_SI / (r * r * DIST_SI_2) * TIME_SI * TIME_SI / DIST_SI  <br>
/// A = m2 / (r * r) * (G * MASS_SI * TIME_SI_2 / DIST_SI_3  <br>
///
pub const GRAVITY_CONSTANT_UNIT: f32 =
    (GRAVITY_CONSTANT_SI * MASS_SI * TIME_SI_2 / DIST_SI_3) as f32;

/// calculate the gravitational acceleration towards a body of mass `other_mass`  <br>
/// r_2 - distance squared
pub fn get_gravity_acceleration(other_mass: f32, r_2: f32) -> f32 {
    other_mass / r_2 * GRAVITY_CONSTANT_UNIT
}
