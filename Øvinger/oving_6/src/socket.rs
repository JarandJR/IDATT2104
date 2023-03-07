use std::io::prelude::*;
use std::net::TcpStream;

use sha1::{Digest, Sha1};

use crate::http_parser::{HTTPRequest, HTTPTag};

pub struct SocketRequest {
    http_request: HTTPRequest,
    sec_key: String,
    GUID: String,
}

impl SocketRequest {
    pub fn new(mut http_request: HTTPRequest) -> Self {
        let header_value = http_request
            .get_header_value_key(HTTPTag::SecWebSocketKey)
            .expect("Could not find value with key");
        Self {
            http_request,
            sec_key: header_value,
            GUID: String::from("258EAFA5-E914-47DA-95CA-C5AB0DC85B11"),
        }
    }

    pub fn send_message(&self, mut stream: TcpStream) {
        let response = self.create_response();
        println!("\n{}", response);
        stream
            .write_all(response.as_bytes())
            .expect("Could not write bytes");
        stream.flush().expect("Could not flush bytes");
    }

    fn create_response(&self) -> String {
        println!("sec-key: {}", self.sec_key);
        let websocket_accept = Self::base64_sha1(&self.sec_key);
        println!("base64: {websocket_accept}");
        let response = format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {websocket_accept}\r\n\r\n");
        response
    }

    fn base64_sha1(input: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();

        let message_digest = result.iter().fold(String::new(), |mut acc, byte| {
            acc.push_str(&format!("{:02x}", byte));
            acc
        });
        println!("sha1: {message_digest}");
        base64::encode(message_digest)
    }
}
