use std::{
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
    time::Duration,
};

pub fn serve(path: &Path, port: u32) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    todo!()
}
