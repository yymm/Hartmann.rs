use std::io::TcpStream;

fn main() {
    {
        let mut socket = TcpStream::connect("127.0.0.1:8000").unwrap();
    
        // ignore the Result
        let _ = socket.write(b"GET / HTTP/1.0\n\n");
    
        let response = socket.read_to_end();

        println!("response: {}", response);
    } // the stream is closed here
}
