use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "MIN_VALUE", default = "0")]
    pub min_value: usize,

    #[envconfig(from = "MAX_VALUE", default = "100")]
    pub max_value: usize,
}

impl Config {
    pub fn init() -> Self {
        if dotenvy::dotenv().is_ok() {
            println!("Loaded .env file");
        }

        Self::init_from_env().expect("Config init failed")
    }
}
