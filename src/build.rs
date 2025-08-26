use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
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
            let out = Path::new("./public").join(relative_path);
            styles.push(relative_path.to_string_lossy().to_string());
            fs::create_dir_all(out.parent().unwrap()).unwrap();
            fs::copy(path, &out).unwrap();

            println!("created: {}", out.display());
        }
    }

    for content in WalkDir::new("./content").into_iter().filter_map(|e| e.ok()) {
        if content.path().extension() == Some(OsStr::new("md")) {
            let path = content.path();
            let md_string = fs::read_to_string(path).unwrap();
            let html = convert(md_string, styles.clone());

            let relative_path = path.strip_prefix("./content").unwrap();
            let out = Path::new("./public")
                .join(relative_path)
                .with_extension("html");

            fs::create_dir_all(out.parent().unwrap()).unwrap();

            fs::write(&out, html).unwrap();
            println!("created: {}", out.display());
        }
    }

    /* for style in styles_dir {
        let entry = style.unwrap();
        let path = entry.path();
        if let Some(extension) = path.extension()
            && extension == "css"
        {
            let file_name = path.file_name().unwrap();
            let dest_path = Path::new("./public").join(file_name);
            fs::copy(&path, &dest_path).unwrap();
            println!(
                "copying {} to ./public/{}",
                path.display(),
                file_name.to_string_lossy()
            );

            styles.push(file_name.to_string_lossy().to_string());
        }
    }

    for content in contents {
        let entry = content.unwrap();
        let path = entry.path();
        if let Some(extension) = path.extension()
            && extension == "md"
        {
            if path.file_stem().unwrap() == "index" {
                let md_string = fs::read_to_string(&path).unwrap();
                let html = convert(md_string, styles.clone());
                let root = File::create("./public/index.html");
                root.and_then(|mut file| file.write_all(html.as_bytes()))
                    .map(|_| println!("index created successfully"))
                    .unwrap_or_else(|e| println!("failed to create index.html: {e}"));
            } else if path.is_file() {
                let md_string = fs::read_to_string(&path).unwrap();
                let html = convert(md_string, styles.clone());
                let root = File::create(format!(
                    "./public/{}.html",
                    path.file_name().unwrap().to_str().unwrap()
                ));
                root.and_then(|mut file| file.write_all(html.as_bytes()))
                    .map(|_| println!("created successfully"))
                    .unwrap_or_else(|e| println!("failed to create index.html: {e}"));
            }
        }
    } */
}
