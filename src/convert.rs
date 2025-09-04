use pulldown_cmark::{Event, MetadataBlockKind, Options, Tag, TagEnd};

pub fn convert(md_string: &str) -> (String, String) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    let mut inside = false;
    let mut frontmatter = String::new();

    let parser = pulldown_cmark::Parser::new_ext(md_string, options).map(|event| {
        match &event {
            Event::Start(Tag::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                inside = true;
            }
            Event::End(TagEnd::MetadataBlock(MetadataBlockKind::YamlStyle)) => {
                inside = false;
            }
            Event::Text(text) if inside => {
                frontmatter = text.to_string();
                //println!("{:?}", text);
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
