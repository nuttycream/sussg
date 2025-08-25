pub mod convert;
pub mod serve;

use std::{
    fs::{self, File},
    io::Write,
    path::{self, Path},
};

use clap::{Parser, Subcommand};

use crate::convert::convert;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
    Build,
    Serve,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Init => init(),
        Commands::Build => build(),
        Commands::Serve => serve::serve(),
    }
}

fn init() {
    match fs::create_dir("content") {
        Ok(_) => println!("./content created successfully"),
        Err(e) => println!("failed to create ./content: {e}"),
    };

    match fs::create_dir("styles") {
        Ok(_) => println!("./styles created successfully"),
        Err(e) => println!("failed to create ./styles: {e}"),
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

fn build() {
    match fs::create_dir("public") {
        Ok(_) => println!("./public created successfully"),
        Err(e) => println!("failed to create ./public: {e}"),
    };

    let contents = fs::read_dir("./content").unwrap();
    let styles_dir = fs::read_dir("./styles").unwrap();
    let mut styles = Vec::new();
    for style in styles_dir {
        let entry = style.unwrap();
        let path = entry.path();
        if let Some(extension) = path.extension()
            && extension == "css"
        {
            let file_name = path.file_name().unwrap();
            let dest_path = Path::new("./public").join(file_name);
            fs::copy(&path, &dest_path).unwrap();
            println!(
                "copying {} to ./public/{}",
                path.display(),
                file_name.to_string_lossy()
            );

            styles.push(file_name.to_string_lossy().to_string());
        }
    }

    for content in contents {
        let entry = content.unwrap();
        let path = entry.path();
        if path.file_name().unwrap() == "index.md" {
            let md_string = fs::read_to_string(&path).unwrap();
            let html = convert(md_string, styles.clone());
            let root = File::create("./public/index.html");
            root.and_then(|mut file| file.write_all(html.as_bytes()))
                .map(|_| println!("index created successfully"))
                .unwrap_or_else(|e| println!("failed to create index.html: {e}"));
        }
    }
}
