use std::{
    ffi::OsStr,
    fs::{self},
    ops::{Index, IndexMut},
    path::Path,
};

use walkdir::WalkDir;

use crate::convert;

pub fn build() {
    match fs::create_dir("public") {
        Ok(_) => println!("./public created successfully"),
        Err(e) => println!("failed to create ./public: {e}"),
    };

    let mut styles = Vec::new();
    for style in WalkDir::new("./styles").into_iter().filter_map(|e| e.ok()) {
        if style.path().extension() == Some(OsStr::new("css")) {
            let path = style.path();
            let relative_path = path.strip_prefix("./styles").unwrap();

            styles.push(relative_path.to_string_lossy().to_string());
            let out = Path::new("./public").join(relative_path);

            fs::create_dir_all(out.parent().unwrap()).unwrap();

            fs::copy(path, &out).unwrap();
            println!("created: {}", out.display());
        }
    }

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

    // build out the templates
    for template in WalkDir::new("./templates")
        .into_iter()
        .filter_map(|e| e.ok())
    {}
}
