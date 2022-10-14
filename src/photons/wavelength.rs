use glam::DVec3;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct WaveLength(pub f64);

impl WaveLength {
    pub const WHITE: WaveLength = WaveLength(550.0);

    pub fn new(value: f64) -> WaveLength {
        WaveLength(value)
    }
}

impl Into<(f64, f64, f64)> for WaveLength {
    /// wavelength to rgb
    fn into(self) -> (f64, f64, f64) {
        // !TODO
        (1.0, 1.0, 1.0)
    }
}

impl From<(f64, f64, f64)> for WaveLength {
    /// rgb to wavelength
    fn from(_rgb: (f64, f64, f64)) -> WaveLength {
        // !TODO
        WaveLength(1.0)
    }
}

impl Into<[u8; 3]> for WaveLength {
    /// wavelength to rgb
    fn into(self) -> [u8; 3] {
        // !TODO
        let v: (f64, f64, f64) = self.into();
        [
            (v.0 * 255.0) as u8,
            (v.1 * 255.0) as u8,
            (v.2 * 255.0) as u8,
        ]
    }
}

impl From<[u8; 3]> for WaveLength {
    /// rgb to wavelength
    fn from(rgb: [u8; 3]) -> WaveLength {
        // !TODO
        let v = (
            rgb[0] as f64 / 255.0,
            rgb[1] as f64 / 255.0,
            rgb[2] as f64 / 255.0,
        );
        v.into()
    }
}

impl From<DVec3> for WaveLength {
    fn from(vec: DVec3) -> WaveLength {
        // !TODO
        (vec.x, vec.y, vec.z).into()
    }
}

impl Into<DVec3> for WaveLength {
    fn into(self) -> DVec3 {
        // !TODO
        let v: (f64, f64, f64) = self.into();
        DVec3::new(v.0, v.1, v.2)
    }
}
