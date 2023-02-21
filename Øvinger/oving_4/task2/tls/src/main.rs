use std::fs;
use std::io::prelude::*;

use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::fs::File;
use std::io::{Read};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Application started");

    let mut file = File::open("keystore/keystore.pfx").unwrap();
    let mut identity = vec![];
    file.read_to_end(&mut identity).unwrap();
    let identity = Identity::from_pkcs12(&identity, "password").unwrap();

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let acceptor = TlsAcceptor::new(identity).unwrap();

    println!("TcpListener up and running");

    for stream in listener.incoming() {
        println!("stream..");
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                std::thread::spawn(move || {
                    println!("connection received");
                    let stream = acceptor.accept(stream).unwrap();
                    println!("HANDLING CONECTIONS..");
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Connection failed");
                println!("ERROR! {}", e);
            }
        }
        println!("Stream sent to thread for handle!");
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TlsStream<TcpStream>) {
    println!("\nstarted making response..");
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Could not read stream");

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
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
