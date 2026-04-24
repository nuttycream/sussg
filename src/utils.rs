use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

use crate::{convert::convert, errors::ErrDis};
use sussg::{Block, Frontmatter, Heading, Plugin, Style, Template, TheThing};

/// neat helper func specific for posts
/// we can reuse get_out_path() but i gotta
/// strip ./public and also make it an absolute
/// path
pub fn get_post_url(site_url: &str, content_path: &Path) -> String {
    let out_path = get_out_path(content_path);
    let relative_path = out_path
        .strip_prefix("./public")
        .expect("somehow failed to strip ./public from rel_path");

    let base_path = extract_url_path(site_url);

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
    templates: &Vec<Template>,
    plugins: &Vec<Plugin>,
    main_style: &Vec<String>,
    base_template: &str,
) -> Result<Vec<TheThing>, ErrDis> {
    let mut things = Vec::new();
    for content in WalkDir::new(content_root_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if content.path().extension() == Some(OsStr::new("md")) {
            let thing = match read_page(
                content.path(),
                content_root_path,
                styles,
                templates,
                plugins,
                main_style,
                base_template,
            ) {
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

pub fn read_plugins(plugin_path: &Path) -> Result<Vec<Plugin>, ErrDis> {
    let mut plugins = Vec::new();

    for plugin_file in WalkDir::new(plugin_path).into_iter().filter_map(|e| e.ok()) {
        let yea = plugin_file.path().extension() == Some(OsStr::new("html"));

        if yea {
            println!("processing plugin:{}", plugin_file.path().display());
            let name = plugin_file
                .path()
                .file_name()
                .and_then(OsStr::to_str)
                .filter(|name| name.ends_with(".html"))
                .map(|name| name.strip_suffix(".html").unwrap_or(name))
                .unwrap_or("")
                .to_string();

            let content = match fs::read_to_string(plugin_file.path()) {
                Ok(p) => p,
                Err(e) => return Err(ErrDis::BadPlugin(e.to_string())),
            };

            plugins.push(Plugin { name, content });
        }
    }

    Ok(plugins)
}

/// simplified version of the original slug::slugify
/// in the slug-rs crate
pub fn slugify(s: &str) -> String {
    let mut slug = String::with_capacity(s.len());

    // true to avoid leading -
    let mut prev_is_dash = true;

    for c in s.chars() {
        match c {
            'a'..='z' | '0'..='9' => {
                prev_is_dash = false;
                slug.push(c);
            }
            'A'..='Z' => {
                prev_is_dash = false;
                slug.push(c.to_ascii_lowercase());
            }
            _ => {
                if !prev_is_dash {
                    slug.push('-');
                    prev_is_dash = true;
                }
            }
        }
    }

    if slug.ends_with('-') {
        slug.pop();
    }
    slug
}

fn read_page(
    page_path: &Path,
    content_root_path: &Path,
    avail_styles: &Vec<Style>,
    avail_templs: &Vec<Template>,
    avail_plugins: &Vec<Plugin>,
    main_styles: &Vec<String>,
    base_template: &str,
) -> Result<TheThing, ErrDis> {
    let (frontmatter, html_output, headings, blocks) = match read_markdown(page_path) {
        Ok((fm, c, h, b)) => (fm, c, h, b),
        Err(e) => return Err(ErrDis::BadMarkdown(e.to_string())),
    };

    println!("processing page:{}", page_path.display());

    let path = page_path
        .strip_prefix(content_root_path)
        .expect("Somehow failed to strip_prefix for ./content")
        .to_path_buf();

    // get the page dir name
    // this would map to None
    // if the path is root like content/
    let section = if path.components().count() > 1 {
        path.components()
            .next()
            .and_then(|c| c.as_os_str().to_str())
            .map(|s| s.to_string())
    } else {
        None
    };

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

    Ok(TheThing {
        path,
        frontmatter,
        styles,
        template: mustache,
        content: html_output,
        section,
        headings,
        plugins: avail_plugins.to_owned(),
    })
}

fn read_markdown(path: &Path) -> Result<(Frontmatter, String, Vec<Heading>, Vec<Block>), ErrDis> {
    let md_string = match fs::read_to_string(path) {
        Ok(md) => md,
        Err(e) => return Err(ErrDis::BadMarkdownString(e.to_string())),
    };

    let (frontmatter_string, html_output, headings, blocks) = convert(&md_string);

    let blockies: Vec<Block> = blocks.iter().map(|b| toml::from_str(b).unwrap()).collect();

    //println!("{:#?}", blockies);

    let frontmatter: Frontmatter = match toml::from_str(&frontmatter_string) {
        Ok(fm) => fm,
        Err(e) => return Err(ErrDis::BadFrontmatter(frontmatter_string, e.to_string())),
    };

    Ok((frontmatter, html_output, headings, blockies))
}

/// find the :// then the next / after the main
fn extract_url_path(site_url: &str) -> String {
    if let Some(scheme_end) = site_url.find("://") {
        let after_scheme = &site_url[scheme_end + 3..];
        if let Some(slash_pos) = after_scheme.find('/') {
            let path = &after_scheme[slash_pos..];
            path.trim_end_matches('/').to_string()
        } else {
            // no path, just use root
            String::new()
        }
    } else {
        // no path, use root
        String::new()
    }
}
