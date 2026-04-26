pub mod cmd;
pub mod config;
pub mod convert;
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
  -o, --out PATH        override the output path for the build 
  -l, --local           build site with local site_url aka root url: \"/\"
  -d, --drafts          build site with draft contents [default: false]
                         - this is set to true on serve, use -d false to disable
  --port PORT           specify port [default: 3030]
";

#[derive(Debug)]
enum Command {
    Init {
        path: PathBuf,
    },
    Build {
        path: PathBuf,
        local: bool,
        out: Option<PathBuf>,
        drafts: bool,
    },
    Serve {
        path: PathBuf,
        port: u32,
        out: Option<PathBuf>,
        drafts: bool,
    },
}

fn parse_args() -> Result<Command, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let subcommand = pargs.subcommand()?;
    let cmd = match subcommand.as_deref() {
        Some("init") => {
            let path = pargs
                .opt_value_from_str(["-p", "--path"])?
                .unwrap_or_else(|| PathBuf::from("./"));

            Command::Init { path }
        }
        Some("build") => {
            let drafts = pargs
                .opt_value_from_str(["-d", "--drafts"])?
                .unwrap_or(false);

            let local = pargs
                .opt_value_from_str(["-l", "--local"])?
                .unwrap_or(false);

            let path = pargs
                .opt_value_from_str(["-p", "--path"])?
                .unwrap_or_else(|| PathBuf::from("./"));

            let out = pargs.opt_value_from_str(["-o", "--out"])?;

            Command::Build {
                path,
                local,
                out,
                drafts,
            }
        }
        Some("serve") => {
            let drafts = pargs
                .opt_value_from_str(["-d", "--drafts"])?
                .unwrap_or(true);

            let path: PathBuf = pargs
                .opt_value_from_str(["-p", "--path"])?
                .unwrap_or_else(|| PathBuf::from("./"));

            let out = pargs.opt_value_from_str(["-o", "--out"])?;

            let port: u32 = pargs.opt_value_from_str("--port")?.unwrap_or(3030);

            Command::Serve {
                path,
                port,
                out,
                drafts,
            }
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
        println!("not a valid argument/s: {:?}.", remaining);
        std::process::exit(1);
    }

    Ok(cmd)
}

fn main() {
    let cmd = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {e:#}.");
            std::process::exit(1);
        }
    };

    let res = match cmd {
        Command::Init { path } => cmd::init::init(&path),
        Command::Build {
            path,
            local,
            out,
            drafts,
        } => cmd::build::build(&path, local, out.as_deref(), drafts),
        Command::Serve {
            path,
            port,
            out,
            drafts,
        } => cmd::serve::serve(&path, port, out.as_deref(), drafts),
    };

    if let Err(e) = res {
        eprintln!("error: {e:#}.");
        std::process::exit(1);
    }
}
