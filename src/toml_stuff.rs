use ramhorns::Content;
use serde::{Deserialize, Serialize};

#[derive(Debug, Content, Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: String,
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
