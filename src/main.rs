pub mod cmd;
pub mod convert;
pub mod toml_stuff;

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
    Serve,
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Init => cmd::init::init(),
        Commands::Build => cmd::build::build(),
        Commands::Serve => cmd::serve::serve(),
    }
}
