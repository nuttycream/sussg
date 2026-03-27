pub mod cmd;
pub mod config;
pub mod convert;
pub mod errors;
pub mod post_process;
pub mod utils;

use std::path::PathBuf;

fn usage() {
    println!("usage:");
    println!("sussg init");
    println!("sussg build [-p|--path <path>]");
    println!("sussg serve [-p|--path <path>] [--port <port>]");
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        usage();
        return;
    }

    match args[1].as_str() {
        "init" => cmd::init::init(),
        "build" => {
            let path = path_arg(&args[2..]).unwrap_or_else(|| PathBuf::from("./"));

            cmd::build::build(&path, false).unwrap();
        }
        "serve" => {
            let path = path_arg(&args[2..]).unwrap_or_else(|| PathBuf::from("./"));

            let port = port_arg(&args[2..]).unwrap_or(3030);

            cmd::serve::serve(&path, port).unwrap();
        }
        other => {
            println!("unknown cmd: {other}");
            usage();
        }
    }
}

fn path_arg(args: &[String]) -> Option<PathBuf> {
    args.windows(2).find_map(|pair| {
        if pair[0] == "-p" || pair[0] == "--path" {
            Some(PathBuf::from(&pair[1]))
        } else {
            None
        }
    })
}

fn port_arg(args: &[String]) -> Option<u32> {
    args.windows(2).find_map(|pair| {
        if pair[0] == "--port" {
            pair[1].parse().ok()
        } else {
            None
        }
    })
}
