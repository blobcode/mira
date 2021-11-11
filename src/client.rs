use serde::{Deserialize, Serialize};
use std::io::Write;
use std::net::TcpStream;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Message {
    name: String,
    message: String,
}

pub fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
        println!("Connected to the server!");

        let tosend = Message {
            name: "me 1".to_string(),
            message: "testmessage".to_string(),
        };

        let encoded: Vec<u8> = bincode::serialize(&tosend).unwrap();

        stream.write_all(&encoded);
    } else {
        println!("Couldn't connect to server...");
    }
}
