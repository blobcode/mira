use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

use crate::client;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 1024;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

struct Message {
    addr: SocketAddr,
    msg: String,
}

pub fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<Message>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        // parse incoming message
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                        println!("{}: {:?}", addr, msg);

                        // create message struct
                        let msgt = Message { addr, msg };

                        // send message to threads
                        tx.send(msgt).expect("failed to send msg to threads");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("closing connection with: {}", addr);
                        break;
                    }
                }

                sleep();
            });
        }

        // on thread recive message
        if let Ok(msg) = rx.try_recv() {
            for mut client in &clients {
                let mut buff = msg.msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);

                if client.peer_addr().unwrap() != msg.addr {
                    client.write_all(&buff).map(|_| client).unwrap();
                };
            }
        }

        sleep();
    }
}
