use std::path::PathBuf;

use ramhorns::Content;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone)]
pub struct Style {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Default, Debug, Clone)]
pub struct Mustache {
    pub name: String,
    pub path: PathBuf,
}

// from /content/posts
// this should all be gathered
// from the frontmatter
pub struct Post {
    pub title: String,
    pub description: Option<String>,
    pub date: Option<String>,
}

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

// Can be anything, a post,
// a page, a SR-71 BlackBird
#[derive(Debug)]
pub struct TheThing {
    pub path: PathBuf,
    pub frontmatter: Frontmatter,
    pub styles: Vec<Style>,
    pub mustache: Mustache,
    pub content: String,
}
