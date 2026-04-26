use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
};

use anyhow::Context;
use minijinja::{Environment, context};
use sussg::SectionThing;

use crate::{config::load_config, utils::*};

pub fn build(path: &Path, is_local: bool, out: Option<&Path>, drafts: bool) -> anyhow::Result<()> {
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

    match fs::create_dir_all(output_dir) {
        Ok(_) => println!("created {output_dir}"),
        Err(e) => println!("somehow failed to create {output_dir}: {e}"),
    }

    read_static(&main_path.join("static"))?;

    let styles = read_styles(&main_path.join("styles"))?;
    let templates = read_templates(&main_path.join("templates"))?;

    let plugins = read_plugins(&main_path.join("plugins"))?;

    let mut env = Environment::new();
    minijinja_contrib::add_to_environment(&mut env);

    // add regular html templates
    for template in &templates {
        env.add_template(&template.name, &template.template)
            .with_context(|| format!("could not add template {}", template.name))?;
    }

    // plugins are also html
    for plugin in &plugins {
        env.add_template_owned(
            // need identifier, otherwise can collide with
            // regular templates, maybe add identifier prefix
            // to template as well?
            format!("plugins/{}", plugin.name),
            plugin.content.to_owned(),
        )
        .with_context(|| format!("could not add plugin {}", plugin.name))?;
    }

    let content = read_content(
        &main_path.join("content"),
        &styles,
        &templates,
        &config.style.main,
        &config.template.base,
    )?;

    // key is the content_dir_name
    let mut sections: HashMap<String, Vec<SectionThing>> = HashMap::new();

    for thing in content.iter() {
        // prevent minijinja from reading drafts if they're not enabled
        if thing.frontmatter.draft.unwrap_or(false) && !drafts {
            continue;
        }

        if let Some(section) = &thing.section {
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

    for mut thing in content {
        if thing.frontmatter.draft.unwrap_or(false) && !drafts {
            continue;
        }

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

        let template = env.get_template(&thing.template.name)?;

        for (idx, plugin) in thing.plugin_args.iter().enumerate() {
            let plugin_name = format!("plugins/{}", plugin.name);

            let rendered = match env.get_template(&plugin_name) {
                Ok(t) => t
                    .render(context! {
                        args => &plugin.args,
                        title => thing.frontmatter.title,
                        content => thing.content,
                        frontmatter => thing.frontmatter,
                        headings => thing.headings,
                        sections,
                        most_recent,
                        site_url,
                    })
                    .unwrap(),
                Err(_) => String::new(),
            };

            let emitter = format!("<!--baka:{}-->", idx);
            thing.content = thing.content.replace(&emitter, &rendered);
        }

        let mut rendered = template.render(context! {
            title => thing.frontmatter.title,
            content => thing.content,
            frontmatter => thing.frontmatter,
            headings => thing.headings,
            sections,
            most_recent,
            site_url,
        })?;

        rendered = link + &rendered;

        let out = get_out_path(&thing.path);

        fs::create_dir_all(out.parent().expect("failed to get parent for current dir"))
            .expect("somehow failed to create directory for content");

        fs::write(&out, rendered).expect("somehow failed to write out file to current dir");

        println!("created: {}", out.display());
    }

    Ok(())
}
