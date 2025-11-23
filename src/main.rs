pub mod cmd;
pub mod config;
pub mod convert;
pub mod errors;
pub mod post_process;
pub mod utils;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
    Build {
        #[arg(short, long, default_value = "./")]
        path: PathBuf,
    },
    Serve {
        #[arg(short, long, default_value = "./")]
        path: PathBuf,

        #[arg(long, default_value_t = 3030)]
        port: u32,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Init => cmd::init::init(),
        Commands::Build { path } => cmd::build::build(path, false).unwrap(),
        Commands::Serve { path, port } => cmd::serve::serve(path, *port),
    }
}
