use generate_random_value::config::Config;
use init_log::init_logging;
use rand::Rng;

pub fn main() -> eyre::Result<()> {
    init_logging(true);

    let cfg = Config::init()?;

    let value = rand::thread_rng().gen_range(cfg.min_value..cfg.max_value);

    println!("Your random value: {}", value);

    Ok(())
}
