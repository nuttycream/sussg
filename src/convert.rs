use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Tag as TagStart, TagEnd};
use sussg::{Block, Frontmatter, Heading, PluginArgs};

pub fn convert(md_string: &str) -> (Frontmatter, String, Vec<Heading>, Vec<PluginArgs>) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_WIKILINKS);

    let mut inside_sussg = false;
    let mut sussg_text = String::new();
    let mut blocks: Vec<String> = Vec::new();

    let mut headings = Vec::new();
    let mut curr_heading_level: Option<u8> = None;
    let mut curr_heading_str = String::new();
    let mut curr_heading_id: Option<String> = None;

    let parser = pulldown_cmark::Parser::new_ext(md_string, options).filter_map(|event| {
        match &event {
            Event::Start(TagStart::CodeBlock(CodeBlockKind::Fenced(l)))
                if l.as_ref() == "sussg" =>
            {
                inside_sussg = true;
                None
            }
            Event::End(TagEnd::CodeBlock) if inside_sussg => {
                inside_sussg = false;

                let idx = blocks.len();
                blocks.push(sussg_text.to_owned());
                sussg_text.clear();

                Some(Event::Html(CowStr::Boxed(
                    format!("<!--baka:{}-->", idx).into_boxed_str(),
                )))
            }
            Event::Start(TagStart::Heading { level, id, .. }) => {
                curr_heading_str.clear();
                curr_heading_level = Some(*level as u8);
                if let Some(id) = id {
                    curr_heading_id = Some(id.to_string());
                } else {
                    curr_heading_id = None;
                }
                //println!("heading level: {}", curr_heading_level);
                Some(event)
            }
            Event::End(TagEnd::Heading(_)) => {
                //println!("heading: {}", curr_heading_str);
                if let Some(level) = curr_heading_level {
                    headings.push(Heading {
                        level,
                        text: curr_heading_str.to_owned(),
                        id: if let Some(id) = &curr_heading_id {
                            id.to_owned()
                        } else {
                            slugify(&curr_heading_str.to_owned())
                        },
                    });

                    curr_heading_str.clear();
                    curr_heading_level = None;
                    curr_heading_id = None;
                }
                Some(event)
            }
            Event::Text(text) => {
                if curr_heading_level.is_some() {
                    curr_heading_str.push_str(text);
                }

                if inside_sussg {
                    sussg_text.push_str(text);
                    None
                } else {
                    Some(event)
                }
            }
            _ => Some(event),
        }
    });

    //println!("frontmatter:{:?}", frontmatter);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let blockies: Vec<Block> = blocks.iter().map(|b| toml::from_str(b).unwrap()).collect();

    let mut frontmatter = Frontmatter::default();
    let mut plugin_args = Vec::new();

    for blocky in blockies {
        match blocky.kind {
            sussg::BlockType::Frontmatter => {
                frontmatter = blocky.data.to_owned().try_into().unwrap();
                //println!("{:#?}", frontmatter)
            }
            sussg::BlockType::Plugin => {
                let plugin: PluginArgs = blocky.data.to_owned().try_into().unwrap();
                plugin_args.push(plugin);
                //println!("{:#?}", plugin_args)
            }
        }
    }

    //println!("{:#?}", blocks);

    // handle plugins in post_process

    //println!("{html_output}");
    //println!("headings: {:#?}", headings);

    (frontmatter, html_output, headings, plugin_args)
}

/// simplified version of the original slug::slugify
/// in the slug-rs crate
fn slugify(s: &str) -> String {
    let mut slug = String::with_capacity(s.len());

    // true to avoid leading -
    let mut prev_is_dash = true;

    for c in s.chars() {
        match c {
            'a'..='z' | '0'..='9' => {
                prev_is_dash = false;
                slug.push(c);
            }
            'A'..='Z' => {
                prev_is_dash = false;
                slug.push(c.to_ascii_lowercase());
            }
            _ => {
                if !prev_is_dash {
                    slug.push('-');
                    prev_is_dash = true;
                }
            }
        }
    }

    if slug.ends_with('-') {
        slug.pop();
    }
    slug
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        let markdown = r#"---
title: test title
author: test author
---

# H1 test
## H2 test
### H3 test
"#;

        let (frontmatter, html, _headings, _plugin_args) = convert(markdown);

        assert!(frontmatter.title.contains("title: test title"));
        assert!(html.contains("<h1"));
        assert!(html.contains("H1 test"));
        assert!(html.contains("<h2"));
        assert!(html.contains("H2 test"));
        assert!(html.contains("<h3"));
        assert!(html.contains("H3 test"));
    }
}
