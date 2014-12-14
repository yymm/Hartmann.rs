#![feature(phase)]
#[phase(plugin, link)] extern crate log;

use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std::io::Command;

fn display_notifier() {
    let output = match Command::new("./src/notifier").arg("0").output() {
        Ok(output) => output,
        Err(e) => panic!("failed to execute process: {}", e),
    };

    debug!("status: {}", output.status);
    debug!("stdout: {}", String::from_utf8_lossy(output.output.as_slice()));
    debug!("stderr: {}", String::from_utf8_lossy(output.error.as_slice()));
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000");
    
    // bind the listener to the specified address
    let mut acceptor = listener.listen();

    println!("\nrun server\n");

    fn handle_client(mut stream: TcpStream) {
        debug!("peer: {}", stream.peer_name());
        debug!("socket: {}", stream.socket_name());
        let mut buf = [0u8, ..4096];
        let _ = stream.read(&mut buf);
        debug!("read: {}", String::from_utf8_lossy(&buf));
        display_notifier();
        let _ = stream.write(b"1");
    }

    // accept connections and process them, spawning a new tasks for each one
    for stream in acceptor.incoming() {
        match stream {
            Ok(stream) => spawn(move || {
                handle_client(stream);
            }),
            Err(e) => panic!("failed to connect client: {}", e),
        }
    }
    
    // close the socket server
    drop(acceptor);
}
