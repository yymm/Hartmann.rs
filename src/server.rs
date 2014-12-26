#![feature(phase)]
#[phase(plugin, link)] extern crate log;

use std::os;
use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std::io::Command;

fn display_notifier(arg: &str) {
    //let output = match Command::new("./src/notifier").arg(arg).output() {
    os::getenv("(uname)")
    let output = match Command::new("terminal-notifier").args(&["-group", "'Hartmann'",
                                                                "-title", "Success!",
                                                                "-subtitle", "Sir! This is a Hartmann notification",
                                                                "-message", "'[Success] Test All Passed!'",
                                                                "-contentImage", "./src/hartmann.jpg",
                                                                "-appIcon", "./src/hartmann.jpg"]).output() {
        Ok(output) => output,
        Err(e) => panic!("failed to execute process: {}", e),
    };

    debug!("status: {}", output.status);
    debug!("stdout: {}", String::from_utf8_lossy(output.output.as_slice()));
    debug!("stderr: {}", String::from_utf8_lossy(output.error.as_slice()));
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080");
    
    // bind the listener to the specified address
    let mut acceptor = listener.listen();

    println!("\nrun server\n");

    fn handle_client(mut stream: TcpStream) {
        debug!("peer: {}", stream.peer_name());
        debug!("socket: {}", stream.socket_name());
        let mut buf = [0];
        let _ = stream.read(&mut buf);
        debug!("read: {}", String::from_utf8_lossy(&buf));
        display_notifier(String::from_utf8_lossy(&buf).as_slice());
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
