use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = vec![0u8; 14];
    stream.read_exact(&mut buffer).unwrap();
    let command = String::from_utf8_lossy(&buffer);
    if command == "*1\r\n$4\r\nPING\r\n" {
        stream.write_all(b"+PONG\r\n").unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}

