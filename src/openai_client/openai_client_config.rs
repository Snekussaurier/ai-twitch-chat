use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub openai_api_key: String,
    pub system_message: String,
}

pub fn load_config() -> Config {
    let config_str = fs::read_to_string("config.yaml").expect("Failed to load yaml config!");
    serde_yaml::from_str(&config_str).expect("Failed to parse config from file")
}
