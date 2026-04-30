use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub style: StyleConfig,
    pub template: TemplateConfig,
    pub serve: ServeConfig,
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct GeneralConfig {
    pub url: String,
    pub output_dir: String,
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct StyleConfig {
    pub main: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct TemplateConfig {
    pub base: String,
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct ServeConfig {
    pub port: u32,
    pub drafts: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            style: StyleConfig::default(),
            template: TemplateConfig::default(),
            serve: ServeConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            url: "/".to_string(),
            output_dir: "public".to_string(),
        }
    }
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            main: vec!["main".to_string()],
        }
    }
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            base: "base".to_string(),
        }
    }
}

impl Default for ServeConfig {
    fn default() -> Self {
        Self {
            port: 3030,
            drafts: true,
        }
    }
}

/// util to load config, if no config found
/// use default
pub fn load_config(path: &Path) -> Config {
    let mut path = path.to_path_buf();
    path.push("config.toml");

    match fs::read_to_string(path) {
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
