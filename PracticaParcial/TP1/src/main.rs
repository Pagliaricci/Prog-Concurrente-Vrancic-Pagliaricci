mod thread_pool;
// <--- minúscula y guion bajo, NO threadPool

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use crate::thread_pool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").expect("No se pudo bindear el puerto");
    println!("Servidor escuchando en http://localhost:3030");

    let pool = Arc::new(ThreadPool::new(4)); // Pool de 4 threads
    let pool_for_work = Arc::clone(&pool);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let pool = Arc::clone(&pool_for_work);
                pool.execute(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Error en la conexión: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    if stream.read(&mut buffer).is_ok() {
        let request = String::from_utf8_lossy(&buffer[..]);
        let request_line = request.lines().next().unwrap_or("");

        if request_line.starts_with("GET /pi/") {
            if let Some(i_str) = request_line.split_whitespace().nth(1) {
                if let Some(i_str) = i_str.strip_prefix("/pi/") {
                    if let Ok(i) = i_str.parse::<u64>() {
                        let pi = calcular_pi(i);

                        let response_body = format!(
                            "Valor de Pi para el término {}: {:.10}",
                            i, pi
                        );

                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                            response_body.len(),
                            response_body
                        );

                        stream.write_all(response.as_bytes()).unwrap();
                        return;
                    }
                }
            }
        }

        let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 13\r\nContent-Type: text/plain\r\n\r\n404 Not Found";
        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn calcular_pi(terminos: u64) -> f64 {
    let mut pi = 0.0;
    let mut signo = 1.0;

    for k in 0..terminos {
        pi += signo / (2 * k + 1) as f64;
        signo = -signo;
    }

    pi * 4.0
}