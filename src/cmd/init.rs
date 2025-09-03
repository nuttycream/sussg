use std::{
    fs::{self, File},
    io::Write,
};

use crate::config::{Config, GeneralConfig};

pub fn init() {
    match fs::create_dir("content") {
        Ok(_) => println!("./content created successfully"),
        Err(e) => println!("failed to create ./content: {e}"),
    };

    match fs::create_dir("styles") {
        Ok(_) => println!("./styles created successfully"),
        Err(e) => println!("failed to create ./styles: {e}"),
    };

    match fs::create_dir("templates") {
        Ok(_) => println!("./templates created successfully"),
        Err(e) => println!("failed to create ./templates: {e}"),
    };

    match fs::create_dir("static") {
        Ok(_) => println!("./static created successfully"),
        Err(e) => println!("failed to create ./static: {e}"),
    };

    let general = GeneralConfig {
        url: "www.some.com".to_string(),
        output_dir: "public".to_string(),
    };

    let toml = toml::to_string(&Config { general }).unwrap();

    File::create("config.toml")
        .and_then(|mut file| file.write_all(toml.as_bytes()))
        .map(|_| println!("config created successfully"))
        .unwrap_or_else(|e| println!("failed to create config: {e}"));

    println!(
        "
finished initializing sussg start creating your markdown
content in the content folder, css goes into the styles folder,
static files such as images/pdfs/etc go in static
then run 'sussg serve' to preview locally.
    "
    );
}
