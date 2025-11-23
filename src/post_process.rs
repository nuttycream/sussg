use sussg::Heading;

pub fn post_process(html: &str, headings: &[Heading]) -> String {
    // ideally we can make some of these configurable
    // for now just handle headings ids
    add_heading_ids(html, headings)
}

fn add_heading_ids(html: &str, headings: &[Heading]) -> String {
    let mut to_add = html.to_string();

    for heading in headings {
        let without_id = format!("<h{}>", heading.level);
        let with_id = format!("<h{} id=\"{}\">", heading.level, heading.id);

        //println!("with_id:{}\nwithout_id:{}", with_id, without_id);

        // replace the ones without em
        if let Some(pos) = to_add.find(&without_id) {
            to_add = format!(
                "{}{}{}",
                &to_add[..pos],
                with_id,
                &to_add[pos + without_id.len()..]
            );
        }
    }

    to_add
}
