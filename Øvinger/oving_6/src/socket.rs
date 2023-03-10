use std::{sync::{Arc, Condvar, Mutex}, thread};
use std::collections::VecDeque;
use std::{io::prelude::*};
use std::net::TcpStream;

use sha1::{Digest, Sha1};
use serde::{Serialize, Deserialize};

use crate::http_parser::{HTTPRequest, HTTPTag};

#[derive(Debug, Serialize, Deserialize)]
struct Coordinate {
    x: i32,
    y: i32
}

pub struct SocketServer {
    sec_key: String,
    guid: String,
    condvar: Arc<(Mutex<bool>, Condvar)>,
    messages: Arc<Mutex<VecDeque<Coordinate>>>,
    clients: Arc<Mutex<VecDeque<TcpStream>>>,
}

impl SocketServer {
    pub fn new(mut http_request: HTTPRequest, clients: Arc<Mutex<VecDeque<TcpStream>>>) -> Self {
        let header_value = http_request
            .get_header_value_key(HTTPTag::SecWebSocketKey)
            .expect("Could not find value with key");
        Self {
            sec_key: header_value,
            guid: String::from("258EAFA5-E914-47DA-95CA-C5AB0DC85B11"),
            condvar: Arc::new((Mutex::new(true), Condvar::new())),
            messages: Arc::new(Mutex::new(VecDeque::new())),
            clients
        }
    }

    pub fn accept(&mut self, mut stream: TcpStream) {
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

    pub fn start_writer_thread(&self) {
        let condvar_copy = self.condvar.clone();
        let messages_copy = self.messages.clone();
        let clients_copy = self.clients.clone();

        thread::spawn(move || {
            loop {
                let (lock, c) = &*condvar_copy;
                {
                    let mut wait = lock.lock().unwrap();
                    while *wait {
                        wait = c.wait(wait).unwrap();
                    }
                }

                while let Some(message_to_send) = messages_copy.lock().unwrap().pop_front() {
                    // Serialize the message and send it to the client
                    let message_to_send_str = serde_json::to_string(&message_to_send).unwrap();

                    let mut buf = Vec::new();
                    buf.push(129);
                    if message_to_send_str.len() <= 125 {
                        buf.push(message_to_send_str.len() as u8);
                    } else if message_to_send_str.len() <= 65535 {
                        buf.push(126);
                        buf.extend_from_slice(&(message_to_send_str.len() as u16).to_be_bytes());
                    } else {
                        buf.push(127);
                        buf.extend_from_slice(&(message_to_send_str.len() as u64).to_be_bytes());
                    }
                    buf.extend_from_slice(message_to_send_str.as_bytes());

                    for  stream in clients_copy.lock().unwrap().iter_mut() {
                        stream.write_all(&buf).expect("Could not write message");
                        stream.flush().expect("Could not send data");
                    }
                }
             // Set the conditional variable back to default
             *lock.lock().unwrap() = true;   
            }
        });
    }

    pub fn start_reader_thread(&self, mut stream: TcpStream) {
        loop {
            let message = match Self::read_websocket_message(&mut stream) {
                Ok(payload) => {
                    let message_str = std::str::from_utf8(&payload).unwrap();
                    Some(serde_json::from_str::<Coordinate>(message_str).unwrap())
                },
                Err(err) => {
                    eprintln!("Could not read WebSocket message: {}", err);
                    None
                }
            };
    
            if let Some(message_received) = message {
                // Add the message to the queue
                let mut messages = self.messages.lock().unwrap();
                messages.push_front(message_received);
                self.notify_all_thread();
            }
        }
    }

    fn notify_all_thread(&self) {
        let (lock, c) = &*self.condvar;
        *lock.lock().unwrap() = false;
        c.notify_all();
    }

    fn read_websocket_message(stream: &mut TcpStream) -> std::io::Result<Vec<u8>> {
        let mut header = [0u8; 2];
        stream.read_exact(&mut header)?;
    
        let is_fin = header[0] & 0b1000_0000 != 0;
        let opcode = header[0] & 0b0000_1111;
        let has_mask = header[1] & 0b1000_0000 != 0;
        let length = header[1] & 0b0111_1111;
    
        let length = match length {
            126 => {
                let mut buf = [0u8; 2];
                stream.read_exact(&mut buf)?;
                u16::from_be_bytes(buf) as usize
            },
            127 => {
                let mut buf = [0u8; 8];
                stream.read_exact(&mut buf)?;
                u64::from_be_bytes(buf) as usize
            },
            _ => length as usize,
        };
    
        let mask = if has_mask {
            let mut buf = [0u8; 4];
            stream.read_exact(&mut buf)?;
            Some(buf)
        } else {
            None
        };
    
        let mut payload = vec![0u8; length];
        stream.read_exact(&mut payload)?;
    
        if let Some(mask) = mask {
            for (i, byte) in payload.iter_mut().enumerate() {
                *byte ^= mask[i % 4];
            }
        }
    
        if is_fin && opcode == 1 {
            Ok(payload)
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid WebSocket message"))
        }
    }
}
