use std::{
    ffi::OsStr,
    fs::{self},
    ops::{Index, IndexMut},
    path::{Path, PathBuf},
};

use ramhorns::Template;
use walkdir::WalkDir;

use crate::{convert, toml_stuff::Frontmatter};

#[derive(Default)]
struct Style {
    name: String,
    path: PathBuf,
}

struct TheThing {
    name: String,
    path: PathBuf,
    style: Vec<Style>,
    template: String,
    content: String,
}

struct Site {
    root: PathBuf,
    things: Vec<TheThing>,
}

impl TheThing {
    fn new(name: String, path: PathBuf, content: String) -> Self {
        TheThing {
            name,
            path,
            style: Vec::new(),
            template: String::new(),
            content,
        }
    }
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

    let mut styles = Vec::new();
    for style_dir in WalkDir::new("./styles").into_iter().filter_map(|e| e.ok()) {
        if style_dir.path().extension() == Some(OsStr::new("css")) {
            let mut style = Style::default();
            let path = style_dir.path();
            style.name = path.file_name().unwrap().to_str().unwrap().to_string();
            style.path = path.strip_prefix("./styles").unwrap().to_path_buf();
            println!("{:?}", style.path);

            styles.push(style);

            // recreate them later
            /*
            let out = Path::new("./public").join(style.path);

            fs::create_dir_all(out.parent().unwrap()).unwrap();

            fs::copy(path, &out).unwrap();
            println!("created: {}", out.display());
            */
        }
    }

    // once we have all the styles loaded

    /*
        for content in WalkDir::new("./content").into_iter().filter_map(|e| e.ok()) {
            if content.path().extension() == Some(OsStr::new("md")) {
                let path = content.path();
                let md_string = fs::read_to_string(path).unwrap();

                let relative_path = path.strip_prefix("./content").unwrap();
                let mut relative_styles = styles.clone();
                println!("{}", relative_path.components().count());

                // HOLY FUCK LMAO
                let count = relative_path.components().count();
                let redirect_path = if count <= 1 {
                    "./"
                } else {
                    &format!("{}/", "..".repeat(count - 1))
                };

                (0..relative_styles.len()).for_each(|n| {
                    *relative_styles.index_mut(n) =
                        format!("{}{}", redirect_path, relative_styles.index(n));
                });

                let html = convert(md_string, relative_styles);
                let out = Path::new("./public")
                    .join(relative_path)
                    .with_extension("html");

                fs::create_dir_all(out.parent().unwrap()).unwrap();

                fs::write(&out, html).unwrap();
                println!("created: {}", out.display());
            }
        }
    */

    // build out the templates
    for template in WalkDir::new("./templates")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let yea = template.path().extension() == Some(OsStr::new("html"));
        let oh = template.path().extension() == Some(OsStr::new("moustache"));

        if yea || oh {
            let source = fs::read_to_string(template.path()).unwrap();
            let tpl = Template::new(source).unwrap();

            /*
            title: String,
            style: String,
            overide_main_style: Option<bool>,
            description: Option<String>,
            author: Option<String>,
            date: Option<Datetime>,
            github: Option<String>, */
            // need to get data from frontmatter md
            // Right?
            // need to rethink the structure of the ssg
            // get style -> get templates ->
            // extract frontmatter -> put frontmatter data into templ ->
            // put content into templ -> render full html
            //
            // go back to drawing board for creating dirs in ./public
            // we are not immediately sending out the html after we convert

            let rendered = tpl.render(&Frontmatter {
                title: "Same".to_string(),
                style: "same".to_string(),
                overide_main_style: None,
                description: None,
                author: None,
                date: None,
                github: None,
            });
        }
    }
}
