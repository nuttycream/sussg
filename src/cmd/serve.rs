use std::path::Path;

use axum::Router;
use notify::{Error, Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::signal;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

use crate::config::load_config;

#[tokio::main]
pub async fn serve(port: u32) {
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    let mut watcher = RecommendedWatcher::new(
        move |result: Result<Event, Error>| {
            let event = result.unwrap();

            if event.kind.is_modify() {
                let mut cfg = load_config();
                cfg.general.url = "/".to_string();
                let _ = crate::cmd::build::build(cfg).unwrap();
                reloader.reload();
            }
        },
        notify::Config::default(),
    )
    .unwrap();

    let mut watch_these = watcher.paths_mut();
    let paths: Vec<&Path> = vec![
        Path::new("content"),
        Path::new("styles"),
        Path::new("static"),
        Path::new("templates"),
        Path::new("config.toml"),
    ];

    for path in paths {
        watch_these.add(path, RecursiveMode::Recursive).unwrap();
    }

    let static_files = ServeDir::new("./public");

    let mut cfg = load_config();
    cfg.general.url = "/".to_string();
    let _ = crate::cmd::build::build(cfg).unwrap();

    let app = Router::new()
        .fallback_service(static_files)
        .layer(livereload);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
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
