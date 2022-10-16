use colors_transform::{Color, Hsl};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct WaveLength(pub f64);

impl WaveLength {
    pub const WHITE: WaveLength = WaveLength(550.0);

    pub fn new(value: f64) -> WaveLength {
        WaveLength(value)
    }
}

impl Default for WaveLength {
    fn default() -> Self {
        WaveLength::WHITE
    }
}

impl Into<(f32, f32, f32)> for WaveLength {
    /// wavelength to rgb
    fn into(self) -> (f32, f32, f32) {
        if (self.0 < 380.) || (self.0 > 750.) {
            return (0., 0., 0.);
        }

        let hue = (650. - self.0 as f32) * 240. / (650. - 475.);
        let rgb = Hsl::from(hue, 100., 50.0).to_rgb();

        rgb.as_tuple()
    }
}

impl Into<[u8; 3]> for WaveLength {
    /// wavelength to rgb
    fn into(self) -> [u8; 3] {
        let v: (f32, f32, f32) = self.into();
        [
            (v.0 * 255.0) as u8,
            (v.1 * 255.0) as u8,
            (v.2 * 255.0) as u8,
        ]
    }
}
