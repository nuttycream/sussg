use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub style: StyleConfig,
    pub template: TemplateConfig,
}

#[derive(Deserialize, Serialize)]
pub struct GeneralConfig {
    pub url: String,
    pub output_dir: String,
}

#[derive(Deserialize, Serialize)]
pub struct StyleConfig {
    pub main: String,
}

#[derive(Deserialize, Serialize)]
pub struct TemplateConfig {
    pub base: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                url: "/".to_string(),
                output_dir: "public".to_string(),
            },

            style: StyleConfig {
                main: "main".to_string(),
            },

            template: TemplateConfig {
                base: "base".to_string(),
            },
        }
    }
}

/// util to load config, if no config found
/// use default
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
