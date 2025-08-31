use std::{
    ffi::OsStr,
    fs::{self},
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::convert::convert;

#[derive(Default)]
struct Style {
    name: String,
    path: PathBuf,
}

#[derive(Default, Debug)]
struct Mustache {
    name: String,
    path: PathBuf,
}

struct TheThing {
    name: String,
    path: PathBuf,
    styles: Vec<Style>,
    mustaches: Vec<Mustache>,
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

    let mut styles = Vec::new();
    for style_file in WalkDir::new("./styles").into_iter().filter_map(|e| e.ok()) {
        if style_file.path().extension() == Some(OsStr::new("css")) {
            let mut style = Style::default();
            let path = style_file.path();
            style.name = path.file_name().unwrap().to_str().unwrap().to_string();
            style.path = path.strip_prefix("./styles").unwrap().to_path_buf();
            println!("{:?}", style.path);

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
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            mustaches.push(Mustache {
                name,
                path: template_file.path().to_path_buf(),
            });
        }
    }

    for content in WalkDir::new("./content").into_iter().filter_map(|e| e.ok()) {
        if content.path().extension() == Some(OsStr::new("md")) {
            let path = content.path();
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            let md_string = fs::read_to_string(path).unwrap();

            let thing = TheThing {
                name,
                path: path.to_path_buf(),
                content: md_string,
                styles: Vec::new(),
                mustaches: Vec::new(),
            };

            let relative_path = path.strip_prefix("./content").unwrap();
        }
    }
    println!("{:?}", mustaches);

    // Frontmatter
    /*
    let source = fs::read_to_string(template_file.path()).unwrap();
    let tpl = ramhorns::Template::new(source).unwrap();

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
    */

    // once we have all the styles loaded
}
