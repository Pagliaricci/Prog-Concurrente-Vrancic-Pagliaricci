use std::net::TcpListener;
use std::io::{Read, Write};
use std::net::TcpStream;
use threadpool::ThreadPool;

const THREAD_POOL_SIZE: usize = 4;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").expect("Failed to bind address");
    let pool = ThreadPool::new(THREAD_POOL_SIZE);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| handle_connection(stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Ok(_) = stream.read(&mut buffer) {
        let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
        stream.write_all(response.as_bytes()).expect("Failed to write response");
        stream.flush().expect("Failed to flush stream");
    }
}