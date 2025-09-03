use pulldown_cmark::{Event, MetadataBlockKind, Options, Tag, TagEnd};

pub fn convert(md_string: &str) -> (String, String) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    let mut inside = false;

    let parser: Vec<_> = pulldown_cmark::Parser::new_ext(md_string, options)
        .map(|event| match event {
            Event::Start(tag) => match tag {
                Tag::MetadataBlock(kind) => {
                    if kind == MetadataBlockKind::YamlStyle {
                        inside = true;
                    }
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::MetadataBlock(kind) => {
                    if kind == MetadataBlockKind::YamlStyle {
                        inside = false;
                    }
                }
                _ => {}
            },
            Event::Text(text) => {
                if inside {
                    println!("{:?}", text);
                }
            }
            _ => {}
        })
        .collect();

    //let mut html_output = String::new();
    //pulldown_cmark::html::push_html(&mut html_output, parser);
    //println!("{html_output}");

    //println!("{}", code_block);

    // code_block.source.into_string()

    ("fuck u".to_string(), "fuck u".to_string())
}
