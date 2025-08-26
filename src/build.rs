use std::{
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

    let contents = fs::read_dir("./content").unwrap();
    let styles_dir = fs::read_dir("./styles").unwrap();

    for entry in WalkDir::new("./content") {
        println!("{}", entry.unwrap().path().display())
    }

    let mut styles = Vec::new();
    for style in styles_dir {
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
    }
}
