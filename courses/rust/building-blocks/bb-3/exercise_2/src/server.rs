mod protocol;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use serde_json;
use protocol::RedisMessage;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = vec![0u8; 4096];
    let nbytes = stream.read(&mut buffer).unwrap();

    let deserialized_message: RedisMessage = serde_json::from_slice(&buffer[..nbytes]).unwrap();
    println!("Received: {:?}", deserialized_message);

    if deserialized_message == RedisMessage::Ping {
        let pong_message = RedisMessage::Pong;
        let serialized_pong = serde_json::to_string(&pong_message).unwrap();
        stream.write_all(serialized_pong.as_bytes()).unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

