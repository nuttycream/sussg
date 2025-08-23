pub fn convert(md_string: String) -> String {
    let parser = pulldown_cmark::Parser::new(&md_string);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    html_output
}
