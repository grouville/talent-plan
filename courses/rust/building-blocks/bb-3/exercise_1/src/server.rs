use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    if buffer.starts_with("*1\r\n$4\r\nPING\r\n") {
        stream.write_all(b"+PONG\r\n").unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}

