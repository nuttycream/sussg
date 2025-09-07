use crate::{convert::convert, errors::ErrDis};
use std::{ffi::OsStr, fs, path::Path};

use sussg::{Frontmatter, Mustache, Style, TheThing};
use walkdir::WalkDir;

pub fn read_static(static_path: &Path) -> Result<(), ErrDis> {
    // maybe add some optimizations to images here? hmmmm?
    for static_file in WalkDir::new(static_path).into_iter().filter_map(|e| e.ok()) {
        let from = static_file.path();
        let to = Path::new("./public").join(from.strip_prefix("./static").unwrap());

        if static_file.file_type().is_dir() {
            match fs::create_dir_all(&to) {
                Ok(_) => {}
                Err(e) => println!("somehow failed to create {}: {}", to.display(), e),
            }
        } else if static_file.file_type().is_file() {
            fs::copy(from, to).expect("failed to copy file");
        }
    }

    Ok(())
}

pub fn read_content(
    content_root_path: &Path,
    styles: &Vec<Style>,
    mustaches: &Vec<Mustache>,
) -> Result<Vec<TheThing>, ErrDis> {
    let mut things = Vec::new();
    for content in WalkDir::new(content_root_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if content.path().extension() == Some(OsStr::new("md")) {
            let thing = match read_page(content.path(), styles, mustaches) {
                Ok(thing) => thing,
                Err(e) => return Err(ErrDis::BadPage(e.to_string())),
            };

            things.push(thing);
        }
    }

    Ok(things)
}

pub fn read_styles(styles_path: &Path) -> Result<Vec<Style>, ErrDis> {
    let mut styles = Vec::new();
    for style_file in WalkDir::new(styles_path).into_iter().filter_map(|e| e.ok()) {
        let mut style = Style::default();
        let path = style_file.path();

        // subbing this in, since file_prefix()
        // is a nightly feature
        style.name = path
            .file_name()
            .and_then(OsStr::to_str)
            .filter(|name| name.ends_with(".css"))
            .and_then(|name| name.strip_suffix(".css"))
            .unwrap_or("")
            .to_string();

        style.path = path
            .strip_prefix("./styles")
            .expect("Somehow failed to strip_prefix for ./styles")
            .to_path_buf();

        styles.push(style);
    }

    Ok(styles)
}

pub fn read_templates(template_path: &Path) -> Result<Vec<Mustache>, ErrDis> {
    let mut mustaches = Vec::new();

    for template_file in WalkDir::new(template_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let yea = template_file.path().extension() == Some(OsStr::new("html"));
        let oh = template_file.path().extension() == Some(OsStr::new("moustache"));

        if yea || oh {
            let name = template_file
                .path()
                .file_name()
                .and_then(OsStr::to_str)
                .filter(|name| name.ends_with(".html") || name.ends_with(".moustache"))
                .map(|name| {
                    name.strip_suffix(".html")
                        .or_else(|| name.strip_suffix(".mustache"))
                        .unwrap_or(name)
                })
                .unwrap_or("")
                .to_string();

            let template = match fs::read_to_string(template_file.path()) {
                Ok(t) => t,
                Err(e) => return Err(ErrDis::BadTemplates(e.to_string())),
            };

            mustaches.push(Mustache {
                name,
                path: template_file.path().to_path_buf(),
                template,
            });
        }
    }

    Ok(mustaches)
}

fn read_page(
    page_path: &Path,
    avail_styles: &Vec<Style>,
    avail_templs: &Vec<Mustache>,
) -> Result<TheThing, ErrDis> {
    let (frontmatter, html_output) = match read_markdown(page_path) {
        Ok((fm, c)) => (fm, c),
        Err(e) => return Err(ErrDis::BadMarkdown(e.to_string())),
    };

    let path = page_path
        .strip_prefix("./content")
        .expect("Somehow failed to strip_prefix for ./content")
        .to_path_buf();

    let styles: Vec<Style> = match frontmatter.styles {
        Some(ref style_strings) => {
            let mut styles = Vec::new();
            for avail_style in avail_styles {
                if style_strings.contains(&avail_style.name) {
                    styles.push(avail_style.clone());
                }
            }
            styles
        }
        None => Vec::new(),
    };

    let mut mustache = avail_templs
        .iter()
        .find(|m| m.name == "base")
        .cloned()
        .expect("Base template not found!!");

    match frontmatter.template {
        Some(ref mustache_name) => {
            for avail_mustache in avail_templs {
                if avail_mustache.name == *mustache_name {
                    mustache = avail_mustache.clone();
                }
            }
        }
        None => {}
    }

    Ok(TheThing {
        path,
        frontmatter,
        styles,
        mustache,
        content: html_output,
    })
}

fn read_markdown(path: &Path) -> Result<(Frontmatter, String), ErrDis> {
    let md_string = match fs::read_to_string(path) {
        Ok(md) => md,
        Err(e) => return Err(ErrDis::BadMarkdownString(e.to_string())),
    };

    let (frontmatter_string, html_output) = convert(&md_string);
    let frontmatter: Frontmatter = match serde_yaml::from_str(&frontmatter_string) {
        Ok(fm) => fm,
        Err(e) => return Err(ErrDis::BadFrontmatter(frontmatter_string, e.to_string())),
    };

    Ok((frontmatter, html_output))
}
