use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();
    let ping_command = b"*1\r\n$4\r\nPING\r\n";
    stream.write_all(ping_command).unwrap();

    let mut buffer = [0; 256];
    stream.read_exact(&mut buffer).unwrap();
    assert_eq!(&buffer[..7], b"+PONG\r\n");
    println!("Received PONG from server");
}

