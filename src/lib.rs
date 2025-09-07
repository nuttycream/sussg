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
    pub template: String,
}

// from /content/posts
// this should all be gathered
// from the frontmatter
pub struct Post {
    pub title: String,
    pub description: Option<String>,
    pub date: Option<String>,
}

/// this is what'll be sent to the template
/// put anything we need here that may be used
/// everything needs a content.
/// BUT that's not really required in mustache
/// templates
#[derive(Content)]
pub struct RenderedContent {
    /// making this optional
    /// but we can reference this im pretty sure
    /// like {{frontmatter.title}}
    pub frontmatter: Frontmatter,

    /// can be referenced in mustache
    /// as {{{content}}}
    /// we need 3x brackets for the
    /// raw html
    pub content: String,
}

#[derive(Content, Debug, Deserialize, Serialize)]
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
    pub mustache: Mustache,
    pub content: String,
}
