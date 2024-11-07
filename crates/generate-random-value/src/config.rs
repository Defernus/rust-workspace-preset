use envstruct::prelude::*;
use eyre::Context;

#[derive(EnvStruct)]
pub struct Config {
    #[env(default = "0")]
    pub min_value: usize,

    #[env(default = "100")]
    pub max_value: usize,
}

impl Config {
    pub fn init() -> eyre::Result<Self> {
        if dotenvy::dotenv().is_ok() {
            println!("Loaded .env file");
        }

        Self::with_prefix("RANDOM_VALUE").wrap_err("failed to load config")
    }
}
