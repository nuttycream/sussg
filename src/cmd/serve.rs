use std::path::Path;

use axum::Router;
use notify::{Error, Event, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::signal;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

#[tokio::main]
pub async fn serve(path: &Path, port: u32) {
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    let static_files = ServeDir::new("./public");

    let _ = crate::cmd::build::build(path, true).unwrap();

    let app = Router::new()
        .fallback_service(static_files)
        .layer(livereload);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .unwrap();

    let path = path.to_owned();
    let main_path = path.clone();

    let mut watcher = RecommendedWatcher::new(
        move |result: Result<Event, Error>| {
            let event = result.unwrap();

            if event.kind.is_modify() {
                let _ = crate::cmd::build::build(&path, true).unwrap();
                reloader.reload();
            }
        },
        notify::Config::default(),
    )
    .unwrap();

    let mut watch_these = watcher.paths_mut();
    let content = main_path.join("content");
    let styles = main_path.join("styles");
    let static_path = main_path.join("static");
    let templates = main_path.join("templates");
    let config_toml = main_path.join("config.toml");

    let paths: Vec<&Path> = vec![&content, &styles, &static_path, &templates, &config_toml];

    for path in paths {
        watch_these.add(path, RecursiveMode::Recursive).unwrap();
    }

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
