use pulldown_cmark::{Event, MetadataBlockKind, Options, Tag as TagStart, TagEnd};

pub fn convert(md_string: &str) -> (String, String) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_GFM);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_WIKILINKS);
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);

    let mut inside_yaml = false;
    let mut frontmatter = String::new();

    let mut curr_heading_level: Option<u8> = None;
    let mut curr_heading_str = String::new();

    let parser = pulldown_cmark::Parser::new_ext(md_string, options).map(|event| {
        match &event {
            Event::Start(TagStart::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                inside_yaml = true;
            }
            Event::End(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                inside_yaml = false;
            }
            Event::Start(TagStart::Heading { level, .. }) => {
                curr_heading_str.clear();
                curr_heading_level = Some(*level as u8);
                //println!("heading level: {}", curr_heading_level);
            }
            Event::End(TagEnd::Heading(_)) => {
                //println!("heading: {}", curr_heading_str);
                curr_heading_str.clear();
                curr_heading_level = None;
            }
            Event::Text(text) => {
                if inside_yaml {
                    frontmatter = text.to_string();
                    //println!("{:?}", text);
                }
                if curr_heading_level.is_some() {
                    curr_heading_str.push_str(text);
                }
            }
            _ => {}
        }
        event
    });

    //println!("frontmatter:{:?}", frontmatter);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    //println!("{html_output}");

    (frontmatter, html_output)
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

        let (frontmatter, html) = convert(markdown);

        assert!(frontmatter.contains("title: test title"));
        assert!(html.contains("<h1"));
        assert!(html.contains("H1 test"));
        assert!(html.contains("<h2"));
        assert!(html.contains("H2 test"));
        assert!(html.contains("<h3"));
        assert!(html.contains("H3 test"));
    }
}
