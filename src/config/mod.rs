use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "PHOTONS_SPAWN_RATE", default = "10000")]
    pub photons_spawn_rate: usize,

    /// amount of max iterations count
    #[envconfig(from = "PHOTONS_TTL", default = "1024")]
    pub photons_ttl: usize,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::init_from_env().unwrap();
}
