use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    pub general: GeneralConfig,
}

#[derive(Default, Deserialize, Serialize)]
pub struct GeneralConfig {
    pub url: String,
    pub output_dir: String,
}

pub fn load_config() -> Config {
    match fs::read_to_string("config.toml") {
        Ok(cfg_string) => match toml::from_str(&cfg_string) {
            Ok(cfg) => cfg,
            Err(e) => {
                println!("failed to convert config.toml: {e}");
                Config::default()
            }
        },
        Err(e) => {
            println!("failed to read config.toml: {e}");
            Config::default()
        }
    }
}
