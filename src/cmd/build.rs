use std::{
    ffi::OsStr,
    fs::{self},
    path::Path,
};

use minijinja::{Environment, context};
use sussg::Post;

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

    let templates = match read_templates(Path::new(OsStr::new("./templates"))) {
        Ok(m) => m,
        Err(e) => return Err(ErrDis::BadTemplates(e.to_string())),
    };

    let mut env = Environment::new();
    for template in &templates {
        match env.add_template(&template.name, &template.template) {
            Ok(_) => println!("added template: {} to environment", template.name),
            Err(e) => println!("could not add template {} because: {}", template.name, e),
        }
    }

    let content = match read_content(
        Path::new(OsStr::new("./content")),
        &styles,
        &templates,
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

        let most_recent = posts
            .last()
            .unwrap_or(&Post {
                title: "no posts".to_string(),
                ..Default::default()
            })
            .clone();

        let mut link = String::new();
        for style in &thing.styles {
            link.push_str(&format!(
                "<link rel=\"stylesheet\" href=\"{}{}\">\n",
                site_url,
                style.path.display()
            ));
        }

        let template = match env.get_template(&thing.template.name) {
            Ok(t) => t,
            Err(e) => return Err(ErrDis::BadTemplates(e.to_string())),
        };

        let mut rendered = match template.render(context! {
            title => thing.frontmatter.title,
            content => thing.content,
            frontmatter => thing.frontmatter,
            posts,
            most_recent,
            site_url
        }) {
            Ok(r) => r,
            Err(e) => return Err(ErrDis::BadRender(e.to_string())),
        };

        rendered = link + &rendered;

        let out = get_out_path(&thing.path);

        fs::create_dir_all(out.parent().expect("failed to get parent for current dir"))
            .expect("somehow failed to create directory for content");

        fs::write(&out, rendered).expect("somehow failed to write out file to current dir");

        println!("created: {}", out.display());
    }

    Ok(())
}
