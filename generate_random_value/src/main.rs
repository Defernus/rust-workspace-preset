use generate_random_value::config::Config;
use rand::Rng;

pub fn main() {
    let cfg = Config::init();

    let value = rand::thread_rng().gen_range(cfg.min_value..cfg.max_value);

    println!("Your random value: {}", value);
}
