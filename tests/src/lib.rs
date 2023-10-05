use std::env;

use generate_random_value::config::Config;

pub fn prepare_config(min: usize, max: usize) -> Config {
    env::set_var("MIN_VALUE", min.to_string());
    env::set_var("MAX_VALUE", max.to_string());

    Config::init()
}
