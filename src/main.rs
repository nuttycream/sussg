pub mod build;
pub mod convert;
pub mod init;
pub mod serve;
pub mod toml_stuff;

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
        Commands::Init => init::init(),
        Commands::Build => build::build(),
        Commands::Serve => serve::serve(),
    }
}
