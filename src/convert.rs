use pulldown_cmark::{Event, MetadataBlockKind, Options, Tag, TextMergeStream};

pub fn convert(md_string: &str) -> (String, String) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    let parser = pulldown_cmark::Parser::new_ext(md_string, options);
    let iterator = TextMergeStream::new(parser);

    for event in iterator {
        match event {
            Event::Start(tag) => {
                if tag == Tag::MetadataBlock(MetadataBlockKind::YamlStyle) {
                    println!()
                }
            }
            _ => {}
        }
    }

    //let mut html_output = String::new();
    //pulldown_cmark::html::push_html(&mut html_output, parser);
    //println!("{html_output}");

    //println!("{}", code_block);

    // code_block.source.into_string()

    ("fuck u".to_string(), "fuck u".to_string())
}
