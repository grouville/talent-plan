use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();
    stream.write_all(b"*1\r\n$4\r\nPING\r\n").unwrap();
    let mut reader = BufReader::new(&stream);
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    println!("Received: {}", buffer);
}

