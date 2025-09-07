use std::{
    ffi::OsStr,
    fs::{self},
    path::Path,
};

use walkdir::WalkDir;

use crate::{
    errors::ErrDis,
    fs::{read_content, read_styles, read_templates},
};

pub fn build() -> Result<(), ErrDis> {
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

    let styles = match read_styles(Path::new(OsStr::new("./styles"))) {
        Ok(s) => s,
        Err(e) => return Err(ErrDis::BadStyles(e.to_string())),
    };

    let mustaches = match read_templates(Path::new(OsStr::new("./templates"))) {
        Ok(m) => m,
        Err(e) => return Err(ErrDis::BadTemplates(e.to_string())),
    };

    let content = match read_content(Path::new(OsStr::new("./content")), &styles, &mustaches) {
        Ok(c) => c,
        Err(e) => return Err(ErrDis::BadContent(e.to_string())),
    };

    //println!("avail_styles:{:?}", styles);
    //println!("avail_templs:{:?}", mustaches);
    //println!("content:{:?}", content);

    Ok(())
}
