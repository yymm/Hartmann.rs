extern crate regex;

use std::os;
use std::io::IoError;
use regex::Regex;
use std::io::TcpStream;
use std::io::Command;
use std::io::net::ip::{SocketAddr, Ipv4Addr};

fn is_match_ipv4_and_port(s: &str) -> bool {
    let re = match Regex::new(r"(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3}):[0-9]{4}") {
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };
    re.is_match(s)
}

fn convert_socketaddr_from_str(s: &str) -> SocketAddr {
    let mut colon_split = s.split_str(":");
    let ip = colon_split.next().unwrap();
    let port = colon_split.next().unwrap();
    let mut comma_split = ip.split_str(".");
    SocketAddr {
        ip: Ipv4Addr(
                from_str(comma_split.next().unwrap()).unwrap(),
                from_str(comma_split.next().unwrap()).unwrap(),
                from_str(comma_split.next().unwrap()).unwrap(),
                from_str(comma_split.next().unwrap()).unwrap()),
        port: from_str(port).unwrap()
    }
}

fn send_message_via_tcp(peer: SocketAddr, msg: &str) -> Result<String, IoError> {
    let mut socket = TcpStream::connect(peer).unwrap();
    let _ = socket.write(msg.as_bytes());
    socket.read_to_string()
}

fn run_subprocess(cmd: &str, opts: &[String]) -> &'static str {
    let output = match Command::new(cmd.to_string()).args(opts).output() {
        Ok(output) => output,
        Err(e) => panic!("failed to execute process: {}", e),
    };

    // TODO: save to redis
    println!("stdout:\n{}", String::from_utf8_lossy(output.output.as_slice()));
    println!("stderr:\n{}", String::from_utf8_lossy(output.error.as_slice()));

    if output.status.success() { "0".as_slice() } else { "1".as_slice() }
}

fn main() {
    let mut args = os::args();
    if args.len() < 3 {
        panic!("Less Argments...");
    }
    let ipv4_and_port = args[1].to_string();
    let cmd = args[2].to_string();
    args.remove(0);
    args.remove(0);
    args.remove(0);
    let opts = args.as_slice();
    println!("{}", cmd);
    println!("{}", opts);

    if is_match_ipv4_and_port(ipv4_and_port.as_slice()) {
        let peer: SocketAddr = convert_socketaddr_from_str(ipv4_and_port.as_slice()); 
        match send_message_via_tcp(peer, run_subprocess(cmd.as_slice(), opts)) {
            Ok(t) => println!("response: {}", t),
            Err(e) => panic!("Connection error: {}",e), 
        };
    } else {
        panic!("valid ip and port");
    }
}
