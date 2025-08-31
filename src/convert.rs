use pulldown_cmark_frontmatter::FrontmatterExtractor;

pub fn convert(md_string: String, styles: Vec<String>) -> (String, String) {
    let mut extractor = FrontmatterExtractor::new(pulldown_cmark::Parser::new(&md_string));
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, &mut extractor);

    let frontmatter = extractor.frontmatter.expect("no frontmatter detected");
    let code_block = frontmatter
        .code_block
        .expect("code block not detected")
        .source
        .to_string();

    println!("{}", code_block);

    // code_block.source.into_string()

    let mut link = String::new();
    for style in styles {
        link.push_str(&format!("<link rel=\"stylesheet\" href=\"{}\">\n", style));
    }

    html_output = link + &html_output;
    (code_block, html_output)
}
