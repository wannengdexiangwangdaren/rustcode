use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut n = 3;
    while n != 0 {
        n -= 1;
        match stream.read(&mut buffer) {
            Ok(_) => {
                let request = String::from_utf8_lossy(&buffer[..]);
                println!("Received request: {}", request);

                // 在这里添加处理请求的逻辑

                let response = "Hello this is server\n";
                stream.write(response.as_bytes()).unwrap();
                continue;
            }
            Err(e) => {
                println!("Failed to read from connection: {}", e);
            }
        }
    }
}

fn main() {
    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(addr).expect("Failed to bind address");

    println!("Server listening on {addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected: {:?}", stream.peer_addr());
                handle_client(stream);
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}
