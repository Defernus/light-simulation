use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Config {
    /// Defines the number of photons that will be spawned from each star.
    #[envconfig(from = "PHOTONS_SPAWN_RATE", default = "1000")]
    pub photons_spawn_rate: usize,

    #[envconfig(from = "out_dir", default = "out")]
    pub out_dir: String,

    /// amount of max iterations count
    #[envconfig(from = "PHOTONS_TTL", default = "10")]
    pub photons_ttl: usize,

    #[envconfig(from = "CAMERA_HOLE_SIZE", default = "0.01")]
    pub camera_hole_size: f32,

    /// How fast pixel will fade out after each iteration
    #[envconfig(from = "FADE_OUT_SPEED", default = "0.9999")]
    pub fade_out_speed: f32,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::init_from_env().unwrap();
}
