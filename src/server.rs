use serde::{Deserialize, Serialize};
use std::io::Read;
use std::net::{TcpListener, TcpStream};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Message {
    name: String,
    message: String,
}

pub fn handle_connection(mut stream: TcpStream) {
    // handle ze connection
    let mut rx_bytes = [0u8; 1000];
    // Read from the current data in the TcpStream
    stream.read(&mut rx_bytes);
    let decoded: Message = bincode::deserialize(&rx_bytes[..]).unwrap();
    println!("{} > {}", decoded.name, decoded.message);
}

pub fn main() {
    println!("running server");

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => { /* connection failed */ }
        }
    }
}
