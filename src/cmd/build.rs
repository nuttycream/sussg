use std::{
    ffi::OsStr,
    fs::{self},
    path::Path,
};

use walkdir::WalkDir;

use crate::{
    errors::ErrDis,
    utils::{read_content, read_static, read_styles, read_templates},
};

pub fn build() -> Result<(), ErrDis> {
    match fs::create_dir_all("public") {
        Ok(_) => println!("created ./public"),
        Err(e) => println!("somehow failed to create ./public: {e}"),
    }

    match read_static(Path::new(OsStr::new("./static"))) {
        Ok(()) => {}
        Err(e) => return Err(ErrDis::BadStaticFiles(e.to_string())),
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

    for thing in content {
        // this is where we'll start to populate
        // templates and then send write them out to html
    }

    Ok(())
}
