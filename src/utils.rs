use crate::{convert::convert, errors::ErrDis};
use std::{
    ffi::OsStr,
    fs,
    os::linux::raw::stat,
    path::{Path, PathBuf},
};

use sussg::{Frontmatter, Style, Template, TheThing};
use walkdir::WalkDir;

/// neat helper func specific for posts
/// we can reuse get_out_path() but i gotta
/// strip ./public and also make it an absolute
/// path
pub fn get_post_url(site_url: &str, content_path: &Path) -> String {
    let out_path = get_out_path(content_path);
    let relative_path = out_path
        .strip_prefix("./public")
        .expect("somehow failed to strip ./public from rel_path");

    let base_path = url::Url::parse(site_url)
        .ok()
        .map(|u| u.path().trim_end_matches('/').to_string())
        .unwrap_or_else(|| String::new());

    if relative_path.file_name() == Some(OsStr::new("index.html")) {
        let parent = relative_path
            .parent()
            .expect("somehow failed to get parent, i need to do some better err handling lol");

        if parent == Path::new("") {
            // root
            if base_path.is_empty() || base_path == "/" {
                "/".to_string()
            } else {
                format!("{}/", base_path)
            }
        } else {
            format!("{}/{}/", base_path, parent.display())
        }
    } else {
        format!("{}/{}", base_path, relative_path.display())
    }
}

/// neat helper func
/// if index.md -> index.html
/// else example.md -> example/index.html
pub fn get_out_path(content_root_path: &Path) -> PathBuf {
    let public = Path::new("./public").join(content_root_path);

    if content_root_path.file_stem() == Some(OsStr::new("index")) {
        public.with_extension("html")
    } else {
        let dir = public.with_extension("");
        dir.join("index.html")
    }
}

pub fn read_static(static_path: &Path) -> Result<(), ErrDis> {
    // maybe add some optimizations to images here? hmmmm?
    for static_file in WalkDir::new(static_path).into_iter().filter_map(|e| e.ok()) {
        let from = static_file.path();
        let to = Path::new("public").join(from.strip_prefix(static_path).unwrap());

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
    mustaches: &Vec<Template>,
    main_style: &Vec<String>,
    base_template: &str,
) -> Result<Vec<TheThing>, ErrDis> {
    let mut things = Vec::new();
    for content in WalkDir::new(content_root_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if content.path().extension() == Some(OsStr::new("md")) {
            let thing =
                match read_page(content.path(), styles, mustaches, main_style, base_template) {
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
        if style_file.path().extension() != Some(OsStr::new("css")) {
            continue;
        }

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

        println!("processing style:{}", path.display());

        style.path = path
            .strip_prefix(styles_path)
            .expect("Somehow failed to strip_prefix for ./styles")
            .to_path_buf();

        let out = Path::new("public").join(&style.path);

        fs::create_dir_all(
            out.parent()
                .expect("failed to get parent of current styles dir"),
        )
        .expect("somehow failed to create dir for styles");

        fs::copy(path, &out).expect("somehow failed to copy the current style file");

        styles.push(style);
    }

    Ok(styles)
}

pub fn read_templates(template_path: &Path) -> Result<Vec<Template>, ErrDis> {
    let mut mustaches = Vec::new();

    for template_file in WalkDir::new(template_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let yea = template_file.path().extension() == Some(OsStr::new("html"));

        if yea {
            println!("processing templ:{}", template_file.path().display());
            let name = template_file
                .path()
                .file_name()
                .and_then(OsStr::to_str)
                .filter(|name| name.ends_with(".html"))
                .map(|name| name.strip_suffix(".html").unwrap_or(name))
                .unwrap_or("")
                .to_string();
            let template = match fs::read_to_string(template_file.path()) {
                Ok(t) => t,
                Err(e) => return Err(ErrDis::BadTemplates(e.to_string())),
            };

            mustaches.push(Template { name, template });
        }
    }

    Ok(mustaches)
}

fn read_page(
    page_path: &Path,
    avail_styles: &Vec<Style>,
    avail_templs: &Vec<Template>,
    main_styles: &Vec<String>,
    base_template: &str,
) -> Result<TheThing, ErrDis> {
    let (frontmatter, html_output) = match read_markdown(page_path) {
        Ok((fm, c)) => (fm, c),
        Err(e) => return Err(ErrDis::BadMarkdown(e.to_string())),
    };

    println!("processing page:{}", page_path.display());

    let mut content_path = page_path.to_path_buf();
    content_path.pop();

    let path = page_path
        .strip_prefix(content_path)
        .expect("Somehow failed to strip_prefix for ./content")
        .to_path_buf();

    let styles: Vec<Style> = {
        let mut styles = Vec::new();

        for avail_style in avail_styles {
            if main_styles.contains(&avail_style.name) {
                styles.push(avail_style.clone());
            }
        }

        if let Some(ref style_strings) = frontmatter.styles {
            for avail_style in avail_styles {
                if style_strings.contains(&avail_style.name)
                    && !main_styles.contains(&avail_style.name)
                {
                    styles.push(avail_style.clone());
                }
            }
        }

        styles
    };

    let mut mustache = avail_templs
        .iter()
        .find(|m| m.name == base_template)
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

    let is_post = frontmatter.is_post.is_some_and(|b| b == true);

    Ok(TheThing {
        path,
        frontmatter,
        styles,
        template: mustache,
        content: html_output,
        is_post,
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
