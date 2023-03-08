mod http_parser;
mod pool;
mod socket;

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use http_parser::{HTTPRequest, HTTPTag, Method};
use pool::ThreadPool;
use socket::Socket;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(3);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.post_task(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Could not read stream");

    let request = std::str::from_utf8(&buffer).expect("Could not parse buffer to utf8");
    let mut http_request = HTTPRequest::new(request);

    if let Some(upgrade) = http_request.get_header_value_key(HTTPTag::UPGRADE) {
        if upgrade == "websocket" {
            handle_socket_connection(stream, http_request);
        }
    } else {
        handle_http_connection(stream, http_request);
    }
}

fn handle_socket_connection(stream: TcpStream, http_request: HTTPRequest) {
    let socket = Socket::new(http_request);
    socket.accept(stream);
}

fn handle_http_connection(mut stream: TcpStream, http_request: HTTPRequest) {
    let (status_line, filename) = if matches!(http_request.method, Method::GET) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).expect("Could not read file");

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream
        .write_all(response.as_bytes())
        .expect("Could not write bytes");
    stream.flush().expect("Could not flush bytes");
}
