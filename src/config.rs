use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub style: StyleConfig,
    pub template: TemplateConfig,
    pub serve: ServeConfig,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct GeneralConfig {
    /// site_url
    /// defaults to "/"
    /// locally serving will
    /// always use "/"
    pub url: String,
    /// output directory
    /// defaults to public/
    pub output_dir: PathBuf,
    /// allow content marked as drafts
    /// to be included
    pub drafts: bool,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct StyleConfig {
    /// main list of styles
    /// to inject, bypasses
    /// frontmatter
    pub main: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct TemplateConfig {
    /// base template file name
    /// that is used when no
    /// template is defined
    pub base: String,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct ServeConfig {
    /// port to serve to
    /// defaults to 3030
    pub port: u32,
    /// show drafts during
    /// serve
    pub drafts: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            url: "/".to_string(),
            output_dir: "public".into(),
            drafts: false,
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
                eprintln!("failed to convert config.toml:\n{e}\nusing defaults");
                Config::default()
            }
        },
        Err(e) => {
            eprintln!("failed to read config.toml:\n{e}\nusing defaults");
            Config::default()
        }
    }
}
