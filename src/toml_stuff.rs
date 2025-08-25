use serde::{Deserialize, Serialize};
use toml::value::Datetime;

#[derive(Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: String,
    pub style: String,

    pub overide_main_style: Option<bool>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub date: Option<Datetime>,

    pub github: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub url: String,
}
