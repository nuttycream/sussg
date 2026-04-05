use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
    time::{Duration, SystemTime},
};
use walkdir::WalkDir;

const POLL_RATE_MS: Duration = Duration::from_millis(50);
const PATHS_TO_WATCH: &[&str] = &["templates", "styles", "content", "static", "config.toml"];

pub fn serve(content_path: &Path, port: u32, out: Option<&Path>) -> std::io::Result<()> {
    let _ = crate::cmd::build::build(content_path, true, out, true);

    let public_dir = PathBuf::from("./public");
    let rx = watch_for_changes(content_path);

    let content_path = content_path.to_owned();
    let out = out.map(|p| p.to_owned());

    println!(
        "serving from: {}\nwatching for changes in:\n{}",
        public_dir.canonicalize()?.display(),
        PATHS_TO_WATCH.join("\n")
    );

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    println!("listening on http://127.0.0.1:{}", port);

    thread::spawn(move || {
        for _ in rx {
            println!("change detected, rebuilding...");
            let _ = crate::cmd::build::build(&content_path, true, out.as_deref(), true);
        }
    });

    for stream in listener.incoming() {
        let mut stream = stream?;
        let buf_reader = BufReader::new(&stream);

        // we only care about the first request line
        let req_line = match buf_reader.lines().next() {
            Some(Ok(line)) => line,
            _ => continue,
        };

        //println!("{req_line}");

        let req_path = req_line.split_whitespace().nth(1).unwrap_or("/");

        let mut file_path = public_dir.join(req_path.trim_start_matches('/'));

        // given dir:
        //  ➜ tree public
        // public
        // ├── favicon.ico
        // ├── fonts
        // │   └── IBMPlexSans.ttf
        // ├── fonts.css
        // ├── index.html
        // ├── main.css
        // ├── posts
        // │   ├── install
        // │   │   └── index.html
        // │   └── usage
        // │       └── index.html
        // └── sussg.svg
        if file_path.is_dir() {
            file_path.push("index.html");
        }

        match fs::read(&file_path) {
            Ok(contents) => {
                let mime = mime(&file_path);
                let header = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                    mime,
                    contents.len(),
                );

                stream.write_all(header.as_bytes())?;
                stream.write_all(&contents)?;
            }
            Err(_) => {
                let body = "404 Not Found";
                let response = format!(
                    "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );

                stream.write_all(response.as_bytes())?;
            }
        }
    }

    Ok(())
}

fn watch_for_changes(content_path: &Path) -> mpsc::Receiver<bool> {
    let (tx, rx) = mpsc::channel();
    let content_path = content_path.to_owned();

    thread::spawn(move || {
        let mut previous = check_metadata(&content_path).unwrap();

        loop {
            thread::sleep(POLL_RATE_MS);
            let curr = check_metadata(&content_path).unwrap();
            for (path, modified) in &curr {
                match previous.get(path) {
                    None => {
                        tx.send(true).ok();
                    }
                    Some(old) if *old != *modified => {
                        tx.send(true).ok();
                    }
                    _ => {}
                }
            }

            previous = curr;
        }
    });

    rx
}

fn check_metadata(path: &Path) -> std::io::Result<HashMap<PathBuf, SystemTime>> {
    let mut map = HashMap::new();

    for entry in WalkDir::new(path)
        .follow_links(true)
        .follow_root_links(true)
    {
        let path = entry?.path().to_owned();
        let modified = fs::metadata(&path)?.modified()?;

        map.insert(path, modified);

        //println!("{:#?}", metadata.modified()?);
    }

    Ok(map)
}

fn mime(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("xml") => "application/xml",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}
