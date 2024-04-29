use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[arg(long, env, default_value = "0")]
    pub min_value: usize,

    #[arg(long, env, default_value = "100")]
    pub max_value: usize,
}

impl Config {
    pub fn init() -> Self {
        if dotenvy::dotenv().is_ok() {
            println!("Loaded .env file");
        }

        Self::parse()
    }
}
