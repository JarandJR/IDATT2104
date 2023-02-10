use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use oving_3::ThreadPool;

fn main() {
    //Note that this will panic at runtime if it fails
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(3);

    for stream in listener.incoming() {
        //Note that this will panic at runtime if this fails
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

    let get = b"GET / HTTP/1.1\r\n";
    let add = b"GET /add";
    let subtract = b"GET /subtract";

    let mut result = 0;
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(add) {
        let request = std::str::from_utf8(&buffer).expect("msg");
        let (a, b) = _get_params_from_request(request, "/add");
        result = a + b;

        ("HTTP/1.1 200 OK", "result.html")
    } else if buffer.starts_with(subtract) {
        let request = std::str::from_utf8(&buffer).expect("msg");
        let (a, b) = _get_params_from_request(request, "/subtract");
        result = a - b;

        ("HTTP/1.1 200 OK", "result.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let result = result.to_string();
    let contents = fs::read_to_string(filename).expect("Could not read file");
    let contents = contents.replace("{{result}}", &result);

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

fn _get_params_from_request(request: &str, method: &str) -> (i32, i32) {
    let params: Vec<&str> = request.split(' ').collect();
    let params: Vec<&str> = params[1].split(method).collect();
    let params: Vec<&str> = params[1].split('&').collect();
    (
        params[0].parse().expect("Could not parse"),
        params[1].parse().expect("Could not parse"),
    )
}
