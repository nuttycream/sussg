use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone)]
pub struct Style {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Default, Debug, Clone)]
pub struct Template {
    pub name: String,
    pub template: String,
}

// from /content/posts
// this should all be gathered
// from the frontmatter
#[derive(Clone, Default, Serialize)]
pub struct Post {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub date: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: String,

    /// putting the post thingymajig here
    /// I can iterate through all pages
    /// then collect the posts using this bool
    /// without having to use a specific post/ dir
    /// GOD IM A GENIUS
    pub is_post: Option<bool>, // defaults to false

    /// can be optional,
    /// it will still inherit base.html
    /// unless overriden
    pub template: Option<String>,
    pub use_base: Option<bool>, // should default to true

    pub description: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,

    pub styles: Option<Vec<String>>,
    pub use_main: Option<bool>, // similar to use_base

    pub github: Option<String>,
}

// Can be anything, a post,
// a page, a SR-71 BlackBird
#[derive(Debug)]
pub struct TheThing {
    pub path: PathBuf,
    pub frontmatter: Frontmatter,
    pub styles: Vec<Style>,
    pub template: Template,
    pub content: String,
    pub is_post: bool,
}
