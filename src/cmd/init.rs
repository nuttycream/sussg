use std::{
    fs::{self, File},
    io::Write,
};

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

    File::create("Config.toml")
        .and_then(|mut file| file.write_all("test4testes".as_bytes()))
        .map(|_| println!("config created successfully"))
        .unwrap_or_else(|e| println!("failed to create config: {e}"));

    println!(
        "
finished initializing ssssg start creating your markdown
content in the content folder, css goes into the styles folder,
then run 'ssssg serve' to preview locally.
    "
    );
}
