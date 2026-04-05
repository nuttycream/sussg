use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
};

use minijinja::{Environment, context};
use sussg::SectionThing;

use crate::{config::load_config, errors::ErrDis, utils::*};

pub fn build(path: &Path, is_local: bool, out: Option<&Path>) -> Result<(), ErrDis> {
    let mut config = load_config(path);

    if is_local {
        config.general.url = "/".to_owned();
    }

    let main_path = path.to_path_buf();

    let output_dir = if let Some(out) = out {
        out.to_str().unwrap_or(&config.general.output_dir)
    } else {
        &config.general.output_dir
    };

    let site_url = config.general.url;

    match fs::create_dir_all(&output_dir) {
        Ok(_) => println!("created {output_dir}"),
        Err(e) => println!("somehow failed to create {output_dir}: {e}"),
    }

    match read_static(&main_path.join("static")) {
        Ok(()) => {}
        Err(e) => return Err(ErrDis::BadStaticFiles(e.to_string())),
    }

    let styles = match read_styles(&main_path.join("styles")) {
        Ok(s) => s,
        Err(e) => return Err(ErrDis::BadStyles(e.to_string())),
    };

    let templates = match read_templates(&main_path.join("templates")) {
        Ok(m) => m,
        Err(e) => return Err(ErrDis::BadTemplates(e.to_string())),
    };

    let mut env = Environment::new();
    minijinja_contrib::add_to_environment(&mut env);

    for template in &templates {
        match env.add_template(&template.name, &template.template) {
            Ok(_) => println!("added template: {} to environment", template.name),
            Err(e) => println!("could not add template {} because: {}", template.name, e),
        }
    }

    let content = match read_content(
        &main_path.join("content"),
        &styles,
        &templates,
        &config.style.main,
        &config.template.base,
    ) {
        Ok(c) => c,
        Err(e) => return Err(ErrDis::BadContent(e.to_string())),
    };

    // key is the content_dir_name
    let mut sections: HashMap<String, Vec<SectionThing>> = HashMap::new();

    for thing in content.iter() {
        if let Some(ref section) = thing.section {
            let url = get_post_url(&site_url, &thing.path);

            let entry = sections.entry(section.to_owned()).or_default();

            entry.push(SectionThing {
                title: thing.frontmatter.title.to_owned(),
                url,
                description: thing.frontmatter.description.to_owned(),
                date: thing.frontmatter.date.to_owned(),
                headings: thing.headings.to_owned(),
            });
        }
    }

    //println!("avail_styles:{:?}", styles);
    //println!("avail_templs:{:?}", mustaches);
    //println!("content:{:?}", content);

    for thing in content {
        // this is where we'll start to populate
        // templates and then write them out to html

        println!("creating:{}", thing.path.display());

        // most recent maps to sections like so:
        // {{ most_recent.posts.title }}
        let most_recent: HashMap<String, SectionThing> = sections
            .iter()
            .filter_map(|(name, items)| items.first().cloned().map(|item| (name.clone(), item)))
            .collect();

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
            headings => thing.headings,
            sections,
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
