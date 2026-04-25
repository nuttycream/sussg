use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime},
};
use walkdir::WalkDir;

const SSE_RELOAD_JS: &[u8] = br#"<script data-event-stream="/events">(() => {
  const inputs = document.currentScript.dataset;
  addEventListener("pageshow", () => {
    const source = new EventSource(inputs.eventStream);
    source.addEventListener("reload", () => {
      source.close();
      window.location.reload();
    });
    const onerror = () => {
      source.removeEventListener("error", onerror);
      source.addEventListener("init", () => {
        source.close();
        window.location.reload();
      });
    };
    source.addEventListener("error", onerror);
    addEventListener("pagehide", () => {
      source.removeEventListener("error", onerror);
      source.close();
    });
  });
})();</script>"#;
const POLL_RATE_MS: Duration = Duration::from_millis(50);
const PATHS_TO_WATCH: &[&str] = &[
    "templates",
    "styles",
    "content",
    "static",
    "plugins",
    "config.toml",
];

#[derive(Clone, Default)]
struct Reloader {
    clients: Arc<Mutex<Vec<TcpStream>>>,
}

impl Reloader {
    fn register(&self, stream: TcpStream) {
        self.clients.lock().unwrap().push(stream);
    }

    fn notify(&self) {
        self.clients
            .lock()
            .unwrap()
            .retain_mut(|s| s.write_all(b"event: reload\ndata:\n\n").is_ok());
    }
}

pub fn serve(
    content_path: &Path,
    port: u32,
    out: Option<&Path>,
    drafts: bool,
) -> std::io::Result<()> {
    let _ = crate::cmd::build::build(content_path, true, out, drafts);

    let public_dir = PathBuf::from("./public");

    let reloader = Reloader::default();

    let content_path = content_path.to_owned();
    let out = out.map(|p| p.to_owned());

    println!(
        "serving from: {}\nwatching for changes in:\n{}",
        public_dir.canonicalize()?.display(),
        PATHS_TO_WATCH.join("\n")
    );

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    println!("listening on http://127.0.0.1:{}", port);

    watch_for_changes(
        content_path.to_owned(),
        out.map(|p| p.to_owned()),
        drafts,
        reloader.to_owned(),
    );

    for stream in listener.incoming() {
        let stream = stream?;
        let public_dir = public_dir.to_owned();
        let reloader = reloader.to_owned();

        thread::spawn(move || {
            let _ = handle_connection(stream, &public_dir, &reloader);
        });
    }

    Ok(())
}

fn watch_for_changes(
    content_path: PathBuf,
    out: Option<PathBuf>,
    drafts: bool,
    reloader: Reloader,
) {
    thread::spawn(move || {
        let mut previous = check_metadata(&content_path).unwrap();
        loop {
            thread::sleep(POLL_RATE_MS);

            let Ok(curr) = check_metadata(&content_path) else {
                continue;
            };

            if curr != previous {
                println!("change detected, rebuilding...");
                let _ = crate::cmd::build::build(&content_path, true, out.as_deref(), drafts);
                reloader.notify();
                previous = curr;
            }
        }
    });
}

fn handle_connection(
    mut stream: TcpStream,
    public_dir: &Path,
    reloader: &Reloader,
) -> std::io::Result<()> {
    let req_line = {
        let mut buf_reader = BufReader::new(&stream);
        let mut line = String::new();
        if buf_reader.read_line(&mut line)? == 0 {
            return Ok(());
        }
        line
    };

    let req_path = req_line.split_whitespace().nth(1).unwrap_or("/");

    if req_path == "/events" {
        return handle_events(stream, reloader);
    }

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
            let contents = if mime.starts_with("text/html") {
                let mut buf = contents;
                buf.extend_from_slice(SSE_RELOAD_JS);
                buf
            } else {
                contents
            };
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

    Ok(())
}

fn handle_events(mut stream: TcpStream, reloader: &Reloader) -> std::io::Result<()> {
    // https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#sending_events_from_the_server
    stream.write_all(
        b"HTTP/1.1 200 OK\r\n\
          Content-Type: text/event-stream\r\n\
          Cache-Control: no-cache\r\n\
          \r\n\
          event: init\ndata:\nretry: 1000\n\n",
    )?;

    stream.flush()?;

    reloader.register(stream);
    Ok(())
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
