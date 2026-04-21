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
pub struct SectionThing {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub date: Option<String>,
    pub headings: Vec<Heading>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: String,

    pub is_archived: Option<bool>,

    /// can be optional,
    /// it will still inherit base.html
    /// unless overriden
    pub template: Option<String>,
    pub use_base: Option<bool>, // should default to true

    pub description: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub draft: Option<bool>,

    pub styles: Option<Vec<String>>,
    pub use_main: Option<bool>, // similar to use_base
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
    /// store content dir names as sections
    /// content/posts, content/pages, etc
    /// so that they can be accessed by the
    /// minijinja context
    pub section: Option<String>,
    pub headings: Vec<Heading>,
}

/// Heading for toc info
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    pub id: String,
}
