use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    path::{Path, PathBuf},
};

pub fn serve(path: &Path, port: u32) -> std::io::Result<()> {
    let _ = crate::cmd::build::build(path, true);

    let public_dir = PathBuf::from("./public");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    println!(
        "Serving from: {}",
        public_dir.canonicalize().unwrap().display()
    );

    println!("Listening on http://127.0.0.1:{}", port);

    for stream in listener.incoming() {
        let mut stream = stream?;
        let buf_reader = BufReader::new(&stream);

        // Parse just the request line (e.g. "GET /index.html HTTP/1.1")
        let req_line = match buf_reader.lines().next() {
            Some(Ok(line)) => line,
            _ => continue,
        };

        let req_path = req_line.split_whitespace().nth(1).unwrap_or("/");

        let mut file_path = public_dir.join(req_path.trim_start_matches('/'));
        if file_path.is_dir() {
            file_path.push("index.html");
        }

        match fs::read(&file_path) {
            Ok(contents) => {
                let mime = mime(&file_path);
                let header = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                    mime,
                    contents.len()
                );

                stream.write_all(header.as_bytes())?;
                stream.write_all(&contents)?;
            }
            Err(_) => {
                let body = "404 Not Found";
                let header = format!(
                    "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n",
                    body.len()
                );

                stream.write_all(header.as_bytes())?;

                stream.write_all(body.as_bytes())?;
            }
        }
    }

    Ok(())
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
