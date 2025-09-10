use std::{
    ffi::OsStr,
    fs::{self},
    path::Path,
};

use ramhorns::Template;
use sussg::{Post, RenderedContent};

use crate::{config::Config, errors::ErrDis, utils::*};

pub fn build(config: Config) -> Result<(), ErrDis> {
    let output_dir = config.general.output_dir;
    let site_url = config.general.url;

    match fs::create_dir_all(&output_dir) {
        Ok(_) => println!("created {output_dir}"),
        Err(e) => println!("somehow failed to create {output_dir}: {e}"),
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

    let content = match read_content(
        Path::new(OsStr::new("./content")),
        &styles,
        &mustaches,
        &config.style.main,
        &config.template.base,
    ) {
        Ok(c) => c,
        Err(e) => return Err(ErrDis::BadContent(e.to_string())),
    };

    //println!("avail_styles:{:?}", styles);
    //println!("avail_templs:{:?}", mustaches);
    //println!("content:{:?}", content);

    let mut posts = Vec::new();
    for thing in content.iter().clone() {
        if thing.is_post {
            let frontmatter = thing.frontmatter.clone();
            let url = get_post_url(&site_url, &thing.path);

            posts.push(Post {
                title: frontmatter.title,
                url,
                description: frontmatter.description,
                date: frontmatter.date,
            });
        }
    }

    for thing in content {
        // this is where we'll start to populate
        // templates and then write them out to html

        println!("creating:{}", thing.path.display());

        let tpl = match Template::new(thing.mustache.template) {
            Ok(t) => t,
            Err(e) => return Err(ErrDis::BadTemplates(e.to_string())),
        };

        let frontmatter = thing.frontmatter.clone();

        let most_recent = posts
            .last()
            .unwrap_or(&Post {
                title: "no posts".to_string(),
                ..Default::default()
            })
            .clone();

        let mut rendered = tpl.render(&RenderedContent {
            title: thing.frontmatter.title,
            content: thing.content,
            frontmatter,
            most_recent,
            posts: posts.clone(),
        });
        //println!("{rendered:?}");

        let mut link = String::new();
        for style in &thing.styles {
            link.push_str(&format!(
                "<link rel=\"stylesheet\" href=\"{}{}\">\n",
                site_url,
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
