use std::{
    ffi::OsStr,
    fs::{self},
    path::Path,
};

use ramhorns::Template;
use sussg::RenderedContent;

use crate::{
    errors::ErrDis,
    utils::{get_out_path, read_content, read_static, read_styles, read_templates},
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
        // templates and then write them out to html

        println!("creating:{}", thing.path.display());

        let tpl = match Template::new(thing.mustache.template) {
            Ok(t) => t,
            Err(e) => return Err(ErrDis::BadTemplates(e.to_string())),
        };

        let frontmatter = thing.frontmatter.clone();

        let mut rendered = tpl.render(&RenderedContent {
            title: thing.frontmatter.title,
            content: thing.content,
            frontmatter,
        });
        //println!("{rendered:?}");

        let mut link = String::new();
        for style in &thing.styles {
            link.push_str(&format!(
                "<link rel=\"stylesheet\" href=\"/{}\">\n",
                style.path.display()
            ));
        }

        rendered = link + &rendered;
        //println!("{rendered:?}");

        let out = get_out_path(&thing.path);

        fs::create_dir_all(out.parent().expect("failed to get parent for current dir"))
            .expect("somehow failed to create directory for content");

        fs::write(&out, rendered).expect("somehow failed to write out file to current dir");

        println!("created: {}", out.display());
    }

    Ok(())
}
