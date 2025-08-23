use pulldown_cmark::Parser;
use pulldown_cmark_frontmatter::FrontmatterExtractor;

pub fn convert(md_string: String) -> String {
    let parser = pulldown_cmark::Parser::new(&md_string);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    html_output
}

fn extract_frontmatter(md_string: String) -> String {
    let mut extractor = FrontmatterExtractor::new(Parser::new(&md_string));
    let frontmatter = extractor.frontmatter.expect("no frontmatter detected");
}
