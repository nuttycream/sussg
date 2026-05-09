use std::{
    fs::{self, File},
    io::{ErrorKind, Write},
    path::Path,
};

use anyhow::Context;

use crate::config::Config;

// todo
const DIRS: &[&str] = &["content", "styles", "templates", "static", "plugins"];

pub fn init(path: &Path) -> anyhow::Result<()> {
    for dir in DIRS {
        let path = path.join(dir);
        match fs::create_dir(path) {
            Ok(()) => println!("./{dir} created successfully"),
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                println!("./{dir} already exists, skipping");
            }
            Err(e) => {
                return Err(e).with_context(|| format!("could not create ./{dir}"));
            }
        }
    }

    let path = path.join("config.toml");
    match File::create(path) {
        Ok(mut file) => {
            let toml = toml::to_string(&Config::default())?;

            file.write_all(toml.as_bytes())?;

            println!("config.toml created successfully");
        }
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            println!("config.toml already exists, skipping");
        }
        Err(e) => return Err(e).context("could not create config.toml")?,
    }

    println!(
        "
finished initializing sussg start creating your markdown
content in the content folder, css goes into the styles folder,
static files such as images/pdfs/etc go in static
then run 'sussg serve' to preview locally.
    "
    );

    Ok(())
}
