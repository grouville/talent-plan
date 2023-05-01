mod protocol;
use protocol::RedisMessage;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use serde_json::{to_string, from_str};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();

    let ping = RedisMessage::Ping;
    let serialized_ping = to_string(&ping).unwrap();
    stream.write_all(serialized_ping.as_bytes()).unwrap();

    let mut reader = BufReader::new(&stream);
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();

    let pong: RedisMessage = from_str(&buffer).unwrap();
    println!("Received: {:?}", pong);
}

