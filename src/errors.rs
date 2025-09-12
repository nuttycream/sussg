use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ErrDis {
    BadDirectory,
    BadStaticFiles(String),
    BadContent(String),
    BadTemplates(String),
    BadStyles(String),
    BadPage(String),
    BadMarkdown(String),
    BadMarkdownString(String),
    BadFrontmatter(String, String),
    BadRender(String),
}

impl Error for ErrDis {}

impl fmt::Display for ErrDis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrDis::BadDirectory => {
                write!(f, "Bad Directory")
            }
            ErrDis::BadStaticFiles(e) => {
                write!(f, "Failed to read static: {e}")
            }
            ErrDis::BadContent(e) => {
                write!(f, "Failed to read content: {e}")
            }
            ErrDis::BadTemplates(e) => {
                write!(f, "Failed to read templates: {e}")
            }
            ErrDis::BadStyles(e) => {
                write!(f, "Failed to read styles: {e}")
            }
            ErrDis::BadPage(e) => {
                write!(f, "Failed to read page: {e}")
            }
            ErrDis::BadMarkdown(e) => {
                write!(f, "Failed to read markdown: {e}")
            }
            ErrDis::BadMarkdownString(e) => {
                write!(f, "Failed to convert string to markdown: {e}")
            }
            ErrDis::BadFrontmatter(fm, e) => {
                write!(f, "Failed to parse frontmatter: {fm}\nBecause:{e}")
            }
            ErrDis::BadRender(e) => write!(f, "Failed to render page: {e}"),
        }
    }
}
