use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: String,

    /// can be optional,
    /// it will still inherit base.html
    /// unless overriden
    pub template: Option<String>,
    pub use_base: Option<bool>, // should default to true

    pub description: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,

    pub styles: Option<Vec<String>>,
    pub overide_main_style: Option<bool>,

    pub github: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub url: String,
}
