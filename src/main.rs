pub mod cmd;
pub mod config;
pub mod convert;
pub mod errors;
pub mod utils;

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
    Build,
    Serve {
        #[arg(short, long, default_value_t = 3030)]
        port: u32,
    },
}

fn main() {
    let args = Args::parse();

    let cfg = config::load_config();

    match &args.command {
        Commands::Init => cmd::init::init(),
        Commands::Build => cmd::build::build(cfg).unwrap(),
        Commands::Serve { port } => cmd::serve::serve(*port),
    }
}
