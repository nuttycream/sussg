use std::{
    ffi::OsStr,
    fs::{self},
    io,
    ops::{Index, IndexMut},
    os::linux::raw::stat,
    path::Path,
};

use ramhorns::{Content, Template};
use walkdir::WalkDir;

use crate::{convert::convert, toml_stuff::Frontmatter};

#[derive(Default, Debug, Clone)]
struct Style {
    name: String,
    path: String,
}

#[derive(Default, Debug, Clone)]
struct Mustache {
    name: String,
    path: String,
}

#[derive(Debug)]
struct TheThing {
    name: String,
    path: String,
    styles: Vec<Style>,
    mustache: Mustache,
    content: String,
}

#[derive(Content)]
struct RenderedContent {
    title: String,
    content: String,
}

pub fn build() {
    match fs::exists("public") {
        Ok(exists) => {
            if !exists {
                match fs::create_dir("public") {
                    Ok(_) => println!("./public created successfully"),
                    Err(e) => println!("failed to create ./public: {e}"),
                };
            } else {
                println!("./public already exists");
            }
        }
        Err(e) => println!("failed to somehow check if public exists: {e}"),
    }

    // maybe add some optimizations to images here? hmmmm?
    for static_file in WalkDir::new("./static").into_iter().filter_map(|e| e.ok()) {
        let from = static_file.path();
        let to = Path::new("./public").join(from.strip_prefix("./static").unwrap());

        if static_file.file_type().is_dir() {
            match fs::create_dir(to) {
                Ok(_) => {}
                Err(e) => println!("failed to create_dir {e}"),
            }
        } else if static_file.file_type().is_file() {
            fs::copy(from, to).unwrap();
        }
    }

    let mut styles = Vec::new();
    for style_file in WalkDir::new("./styles").into_iter().filter_map(|e| e.ok()) {
        if style_file.path().extension() == Some(OsStr::new("css")) {
            let mut style = Style::default();
            let path = style_file.path();
            style.name = path
                .file_name()
                .and_then(OsStr::to_str)
                .filter(|name| name.ends_with(".css"))
                .and_then(|name| name.strip_suffix(".css"))
                .unwrap_or("")
                .to_string();

            style.path = path
                .strip_prefix("./styles")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            styles.push(style);

            // recreate them later
            let rel_path = path.strip_prefix("./styles").unwrap().to_path_buf();
            let out = Path::new("./public").join(rel_path);

            fs::create_dir_all(out.parent().unwrap()).unwrap();

            fs::copy(path, &out).unwrap();
            println!("created: {}", out.display());
        }
    }

    let mut mustaches = Vec::new();
    for template_file in WalkDir::new("./templates")
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

            mustaches.push(Mustache {
                name,
                path: template_file.path().to_str().unwrap().to_string(),
            });
        }
    }

    println!("avail_styles:{:?}", styles);
    println!("avail_templs:{:?}", mustaches);
    for content in WalkDir::new("./content").into_iter().filter_map(|e| e.ok()) {
        if content.path().extension() == Some(OsStr::new("md")) {
            let path = content.path();
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            let md_string = fs::read_to_string(path).unwrap();

            println!("processing:{}", name);

            let (frontmatter_string, content) = convert(&md_string);

            let frontmatter: Frontmatter = toml::from_str(&frontmatter_string)
                .expect("could not convert frontmatter to struct");

            //println!("{:?}", frontmatter);

            let mut thing_styles = Vec::new();
            if let Some(styles_strings) = frontmatter.styles {
                for style in &styles {
                    if styles_strings.contains(&style.name) {
                        thing_styles.push(style.clone());
                    }
                }
            }

            // todo get this from mustache bro
            let mut thing_mustache = mustaches
                .iter()
                .find(|m| m.name == "base")
                .cloned()
                .unwrap();

            if let Some(mustache_name) = frontmatter.template {
                println!("{:?}", mustache_name);
                for avail_mustache in &mustaches {
                    if avail_mustache.name == mustache_name {
                        thing_mustache = avail_mustache.clone();
                    }
                }
            }

            println!("loaded_styles:{:?}", thing_styles);
            println!("loaded_templ:{:?}", thing_mustache);

            // HOLY FUCK LMAO
            let relative_path = path.strip_prefix("./content").unwrap();
            let count = relative_path.components().count();
            let redirect_path = if count <= 1 {
                "./"
            } else {
                &format!("{}/", "..".repeat(count - 1))
            };

            // thing is only used when building
            // so we can pragmatically store what we need
            // to build that file out
            // methinks :shrug:
            let mut thing = TheThing {
                name,
                path: path.to_str().unwrap().to_string(),
                content,
                styles: thing_styles,
                mustache: thing_mustache,
            };

            (0..thing.styles.len()).for_each(|n| {
                thing.styles.index_mut(n).path =
                    format!("{}{}", redirect_path, thing.styles.index(n).path);
            });

            // now build out the html
            let source = fs::read_to_string(thing.mustache.path).expect("mustache path is invalid");
            let tpl = Template::new(source).unwrap();

            let mut rendered = tpl.render(&RenderedContent {
                title: frontmatter.title,
                content: thing.content,
            });

            let mut link = String::new();
            for style in thing.styles {
                link.push_str(&format!(
                    "<link rel=\"stylesheet\" href=\"{}\">\n",
                    style.path
                ));
            }

            rendered = link + &rendered;

            println!("{}\n", rendered);
            let out = Path::new("./public")
                .join(relative_path)
                .with_extension("html");

            fs::create_dir_all(out.parent().unwrap()).unwrap();

            fs::write(&out, rendered).unwrap();
            println!("created: {}", out.display());
        }
    }
}
