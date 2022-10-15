use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "PHOTONS_SPAWN_RATE", default = "100000")]
    pub photons_spawn_rate: usize,

    /// amount of max iterations count
    #[envconfig(from = "PHOTONS_TTL", default = "128")]
    pub photons_ttl: usize,

    /// How fast pixel will fade out after each iteration
    #[envconfig(from = "FADE_OUT_SPEED", default = "0.99")]
    pub fade_out_speed: f64,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::init_from_env().unwrap();
}
