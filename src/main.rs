use std::{
    fs::{self, File},
    io::Write,
};

use axum::{Router, response::Html, routing::get};
use clap::{Parser, Subcommand};
use tokio::signal::{self};
use tower_livereload::LiveReloadLayer;

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
        Commands::Serve => serve(),
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

fn build() {}

#[tokio::main]
async fn serve() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(LiveReloadLayer::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
