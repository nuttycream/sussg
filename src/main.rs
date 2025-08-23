pub mod md_to_html;

use std::{
    fs::{self, File},
    io::Write,
};

use axum::Router;
use clap::{Parser, Subcommand};
use tokio::signal::{self};
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

use crate::md_to_html::convert;

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

fn build() {
    match fs::create_dir("public") {
        Ok(_) => println!("./public created successfully"),
        Err(e) => println!("failed to create ./public: {e}"),
    };

    let contents = fs::read_dir("./content").unwrap();

    for content in contents {
        let entry = content.unwrap();
        let path = entry.path();
        if path.file_name().unwrap() == "index.md" {
            let md_string = fs::read_to_string(&path).unwrap();
            let html = convert(md_string);
            let root = File::create("./public/index.html");
            root.and_then(|mut file| file.write_all(html.as_bytes()))
                .map(|_| println!("index created successfully"))
                .unwrap_or_else(|e| println!("failed to create index.html: {e}"));
        }
    }
}

#[tokio::main]
async fn serve() {
    let static_files = ServeDir::new("./public");

    // todo: cannot use "/" to server static files
    // need workaround
    let app = Router::new()
        .nest_service("/index", static_files)
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
