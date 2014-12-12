#![feature(phase)]
#[phase(plugin, link)] extern crate log;

use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000");
    
    // bind the listener to the specified address
    let mut acceptor = listener.listen();

    fn handle_client(mut stream: TcpStream) {
        println!("peer: {}", stream.peer_name());
        println!("socket: {}", stream.socket_name());
        let mut buf = [0];
        let _ = stream.read(&mut buf);
        println!("read: {}", buf);
        stream.write(b"get back");
    }

    // accept connections and process them, spawning a new tasks for each one
    for stream in acceptor.incoming() {
        match stream {
            Err(e) => {
                panic!("failed to connect client: {}", e);
            }
            Ok(stream) => spawn(proc() {
                // connection succeeded
                handle_client(stream);
            })
        }
    }
    
    // close the socket server
    drop(acceptor);
}
