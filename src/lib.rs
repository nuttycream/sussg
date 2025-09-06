use std::path::PathBuf;

use ramhorns::Content;

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

// todo move this
// in fact this whole fuckign
// file needs a refactor
#[derive(Content)]
pub struct RenderedContent {
    pub title: String,
    pub content: String,
}

// from /content/posts
// this should all be gathered
// from the frontmatter
pub struct Post {
    pub title: String,
    pub description: Option<String>,
    pub date: Option<String>,
}

// Can be anything, a post,
// a page, a SR-71 BlackBird
#[derive(Debug)]
pub struct TheThing {
    pub path: PathBuf,
    pub styles: Vec<Style>,
    pub mustache: Mustache,
    pub content: String,
}
