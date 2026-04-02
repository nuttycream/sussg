pub mod cmd;
pub mod config;
pub mod convert;
pub mod errors;
pub mod post_process;
pub mod utils;

use std::path::PathBuf;

const HELP: &str = "\
sussg - a simple static site generator

USAGE:
  sussg <COMMAND> [OPTIONS]

COMMANDS:
  init                  initialize a new site
  build                 build the site
  serve                 build and serve the site

OPTIONS:
  -h, --help            print this
  -p, --path PATH       specify site path [default: ./]
  -l, --local           build site with local site_url aka root url: \"/\"
  --port PORT           specify port [default: 3030]
";

#[derive(Debug)]
enum Command {
    Init,
    Build { path: PathBuf, local: bool },
    Serve { path: PathBuf, port: u32 },
}

fn parse_args() -> Result<Command, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let subcommand = pargs.subcommand()?;
    let cmd = match subcommand.as_deref() {
        Some("init") => Command::Init,
        Some("build") => {
            let path: PathBuf = pargs
                .opt_value_from_str(["-p", "--path"])?
                .unwrap_or_else(|| PathBuf::from("./"));

            let local = pargs
                .opt_value_from_str(["-l", "--local"])?
                .unwrap_or(false);

            Command::Build { path, local }
        }
        Some("serve") => {
            let path: PathBuf = pargs
                .opt_value_from_str(["-p", "--path"])?
                .unwrap_or_else(|| PathBuf::from("./"));
            let port: u32 = pargs.opt_value_from_str("--port")?.unwrap_or(3030);
            Command::Serve { path, port }
        }
        Some(other) => {
            eprintln!("unknown command: {other}");
            print!("{}", HELP);
            std::process::exit(1);
        }
        None => {
            print!("{}", HELP);
            std::process::exit(1);
        }
    };

    let remaining = pargs.finish();

    if !remaining.is_empty() {
        println!("unused arguments left: {:?}.", remaining);
    }

    Ok(cmd)
}

fn main() {
    let cmd = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    match cmd {
        Command::Init => cmd::init::init(),
        Command::Build { path, local } => {
            cmd::build::build(&path, local).unwrap();
        }
        Command::Serve { path, port } => {
            cmd::serve::serve(&path, port).unwrap();
        }
    }
}
