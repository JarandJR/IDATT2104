use std::io::prelude::*;
use std::net::TcpStream;

use sha1::{Digest, Sha1};

use crate::http_parser::{HTTPRequest, HTTPTag};

pub struct Socket {
    sec_key: String,
    guid: String,
}

impl Socket {
    pub fn new(mut http_request: HTTPRequest) -> Self {
        let header_value = http_request
            .get_header_value_key(HTTPTag::SecWebSocketKey)
            .expect("Could not find value with key");
        Self {
            sec_key: header_value,
            guid: String::from("258EAFA5-E914-47DA-95CA-C5AB0DC85B11"),
        }
    }

    pub fn accept(&self, mut stream: TcpStream) {
        let sec_accept = self.generate_sec_accept();
        let response =  format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n", sec_accept);
        stream
            .write_all(response.as_bytes())
            .expect("Could not write bytes");
        stream.flush().expect("Could not flush bytes");
    }

    fn generate_sec_accept(&self) -> String {
        let mut hasher = Sha1::new();
        hasher.update(self.sec_key.as_bytes());
        hasher.update(self.guid.as_bytes());
        let result = hasher.finalize();

        let mut accept_key = [0u8; 20];
        accept_key.copy_from_slice(&result[..20]);
        let encoded_key = base64::encode(&accept_key);
        encoded_key
    }
}
