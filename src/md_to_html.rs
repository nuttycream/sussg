use pulldown_cmark_frontmatter::FrontmatterExtractor;

pub fn convert(md_string: String, styles: Vec<String>) -> String {
    extract_frontmatter(md_string.clone());

    let parser = pulldown_cmark::Parser::new(&md_string);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let mut link = String::new();
    for style in styles {
        link.push_str(&format!("<link rel=\"stylesheet\" href=\"{}\">\n", style));
    }

    html_output = link + &html_output;

    html_output
}

fn extract_frontmatter(md_string: String) -> String {
    let extractor = FrontmatterExtractor::from_markdown(&md_string);
    let frontmatter = extractor.extract().expect("no frontmatter detected");

    let code_block = frontmatter.code_block.expect("code block not detected");

    println!("{}", code_block.source.clone().into_string());

    code_block.source.into_string()
}
